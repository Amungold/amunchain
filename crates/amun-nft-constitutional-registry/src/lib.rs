use amun_nft_bridge::BridgeLock;
use amun_nft_governance::GovernanceRight;
use amun_nft_royalty::RoyaltyPolicy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Unified record for an NFT's constitutional state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftConstitutionalRecord {
    pub token_id: [u8; 32],
    pub owner: [u8; 32],
    pub collection_id: Option<[u8; 32]>,
    pub creator: [u8; 32],
    pub mining_origin: Option<String>,
    pub royalty_policy: Option<RoyaltyPolicy>,
    pub governance_right: Option<GovernanceRight>,
    pub bridge_lock: Option<BridgeLock>,
}

/// Constitutional registry holding all NFT records
#[derive(Debug, Clone, Default)]
pub struct ConstitutionalRegistry {
    pub records: BTreeMap<[u8; 32], NftConstitutionalRecord>,
}

impl ConstitutionalRegistry {
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
        }
    }

    /// Register or update an NFT constitutional record
    pub fn register(&mut self, record: NftConstitutionalRecord) {
        self.records.insert(record.token_id, record);
    }

    /// Get a record by token ID
    pub fn get(&self, token_id: &[u8; 32]) -> Option<&NftConstitutionalRecord> {
        self.records.get(token_id)
    }

    /// Compute the master constitutional root for all NFT records
    pub fn compute_constitutional_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_NFT_CONSTITUTIONAL_ROOT_V1");
        for (id, record) in &self.records {
            hasher.update(id);
            let bytes = serde_json::to_vec(record).unwrap();
            hasher.update(&bytes);
        }
        hasher.finalize().into()
    }
}
