use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use amun_nft_constitutional_registry::{ConstitutionalRegistry};
use amun_nft_bridge::BridgeLedger;
use amun_nft_governance::GovernanceLedger;
use amun_nft_royalty::RoyaltyEngine;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RightsEnforcementResult {
    pub token_id: [u8; 32],
    pub allowed: bool,
    pub reason: Option<String>,
    pub required_royalty: Option<u64>,
}

pub struct RightsEnforcementEngine;

impl RightsEnforcementEngine {
    pub fn validate_transfer(
        registry: &ConstitutionalRegistry,
        bridge_ledger: &BridgeLedger,
        governance_ledger: &GovernanceLedger,
        token_id: &[u8; 32],
        seller: &[u8; 32],
        _buyer: &[u8; 32],
        sale_price: u64,
    ) -> RightsEnforcementResult {
        let record = match registry.get(token_id) {
            Some(r) => r,
            None => return RightsEnforcementResult {
                token_id: *token_id, allowed: false,
                reason: Some("Token not registered".into()),
                required_royalty: None,
            },
        };

        if record.owner != *seller {
            return RightsEnforcementResult {
                token_id: *token_id, allowed: false,
                reason: Some("Seller is not constitutional owner".into()),
                required_royalty: None,
            };
        }

        if let Some(ref lock) = record.bridge_lock {
            let lock_id = compute_lock_id(lock);
            if bridge_ledger.is_locked(&lock_id) {
                return RightsEnforcementResult {
                    token_id: *token_id, allowed: false,
                    reason: Some("Token is locked in cross-chain bridge".into()),
                    required_royalty: None,
                };
            }
        }

        if let Some(ref gov) = record.governance_right {
            if !governance_ledger.can_propose(token_id, seller) && gov.voting_power > 0 {
                return RightsEnforcementResult {
                    token_id: *token_id, allowed: false,
                    reason: Some("Seller lacks active governance rights".into()),
                    required_royalty: None,
                };
            }
        }

        let required_royalty = record.royalty_policy.as_ref().map(|policy| {
            RoyaltyEngine::compute_royalty(sale_price, policy.royalty_bps)
        });

        RightsEnforcementResult {
            token_id: *token_id,
            allowed: true,
            reason: None,
            required_royalty,
        }
    }

    pub fn produce_enforcement_proof(
        registry: &ConstitutionalRegistry,
        marketplace_root: [u8; 32],
        royalty_root: [u8; 32],
        governance_root: [u8; 32],
        bridge_root: [u8; 32],
        enforcement_root: [u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_RIGHTS_ENFORCEMENT_V1");
        hasher.update(registry.compute_constitutional_root());
        hasher.update(marketplace_root);
        hasher.update(royalty_root);
        hasher.update(governance_root);
        hasher.update(bridge_root);
        hasher.update(enforcement_root);
        hasher.finalize().into()
    }
}

fn compute_lock_id(lock: &amun_nft_bridge::BridgeLock) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"AMUN_BRIDGE_LOCK_V1");
    hasher.update(lock.source_chain.to_le_bytes());
    hasher.update(lock.token_id);
    hasher.update(lock.owner);
    hasher.update(lock.destination_chain.to_le_bytes());
    hasher.update(lock.destination_owner);
    hasher.update(lock.lock_height.to_le_bytes());
    hasher.finalize().into()
}
