use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Royalty policy for an NFT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoyaltyPolicy {
    pub creator: [u8; 32],
    /// Royalty in basis points (10000 = 100%)
    pub royalty_bps: u16,
}

/// Royalty record stored in evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoyaltyRecord {
    pub token_id: [u8; 32],
    pub creator: [u8; 32],
    pub payer: [u8; 32],
    pub sale_price: u64,
    pub royalty_amount: u64,
    pub block_height: u64,
}

/// Royalty computation engine
pub struct RoyaltyEngine;

impl RoyaltyEngine {
    /// Compute royalty amount safely (prevents overflow)
    pub fn compute_royalty(sale_price: u64, royalty_bps: u16) -> u64 {
        if royalty_bps == 0 || sale_price == 0 {
            return 0;
        }
        // Use u128 to avoid overflow
        let amount = (sale_price as u128 * royalty_bps as u128) / 10_000u128;
        amount as u64
    }

    /// Generate royalty evidence record
    pub fn generate_royalty_record(
        token_id: [u8; 32],
        policy: &RoyaltyPolicy,
        payer: [u8; 32],
        sale_price: u64,
        block_height: u64,
    ) -> RoyaltyRecord {
        let royalty_amount = Self::compute_royalty(sale_price, policy.royalty_bps);
        RoyaltyRecord {
            token_id,
            creator: policy.creator,
            payer,
            sale_price,
            royalty_amount,
            block_height,
        }
    }
}

/// Accumulates royalty records into a Merkle root
pub fn accumulate_royalty_root(records: &[RoyaltyRecord]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"AMUN_ROYALTY_EVIDENCE_V1");
    for record in records {
        let bytes = serde_json::to_vec(record).unwrap();
        hasher.update(&bytes);
    }
    hasher.finalize().into()
}
