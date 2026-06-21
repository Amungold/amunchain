use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use amun_nft_bridge::BridgeLedger;
use amun_nft_constitutional_registry::{ConstitutionalRegistry};

/// Constitutional proof combining all NFT roots
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalProof {
    pub nft_root: [u8; 32],
    pub marketplace_root: [u8; 32],
    pub royalty_root: [u8; 32],
    pub governance_root: [u8; 32],
    pub bridge_root: [u8; 32],
    pub unified_root: [u8; 32],
}

/// Enforcement engine
pub struct EnforcementEngine;

impl EnforcementEngine {
    /// Check if an NFT is locked in a bridge (cannot be sold)
    pub fn can_be_sold(
        registry: &ConstitutionalRegistry,
        bridge_ledger: &BridgeLedger,
        token_id: &[u8; 32],
    ) -> bool {
        if let Some(record) = registry.get(token_id) {
            if let Some(ref lock) = record.bridge_lock {
                let lock_id = compute_lock_id(lock);
                if bridge_ledger.is_locked(&lock_id) {
                    return false;
                }
            }
        }
        true
    }

    /// Transfer governance rights to a new owner
    pub fn transfer_governance(
        registry: &mut ConstitutionalRegistry,
        token_id: &[u8; 32],
        new_owner: &[u8; 32],
    ) {
        if let Some(record) = registry.get(token_id) {
            let mut updated = record.clone();
            updated.owner = *new_owner;
            if let Some(ref mut gov) = updated.governance_right {
                gov.owner = *new_owner;
            }
            registry.register(updated);
        }
    }

    /// Enforce royalty policy during a sale
    pub fn enforce_royalty(
        registry: &ConstitutionalRegistry,
        token_id: &[u8; 32],
        sale_price: u64,
    ) -> Option<u64> {
        if let Some(record) = registry.get(token_id) {
            if let Some(ref policy) = record.royalty_policy {
                return Some(amun_nft_royalty::RoyaltyEngine::compute_royalty(sale_price, policy.royalty_bps));
            }
        }
        None
    }

    /// Produce a unified constitutional proof
    pub fn produce_constitutional_proof(
        registry: &ConstitutionalRegistry,
        marketplace_root: [u8; 32],
        royalty_root: [u8; 32],
        governance_root: [u8; 32],
        bridge_root: [u8; 32],
    ) -> ConstitutionalProof {
        let nft_root = registry.compute_constitutional_root();
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_CONSTITUTIONAL_PROOF_V1");
        hasher.update(nft_root);
        hasher.update(marketplace_root);
        hasher.update(royalty_root);
        hasher.update(governance_root);
        hasher.update(bridge_root);
        let unified_root: [u8; 32] = hasher.finalize().into();
        ConstitutionalProof {
            nft_root,
            marketplace_root,
            royalty_root,
            governance_root,
            bridge_root,
            unified_root,
        }
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
