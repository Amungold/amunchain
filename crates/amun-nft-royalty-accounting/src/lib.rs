use amun_nft_royalty::RoyaltyRecord;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Accumulated royalty balance for a creator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoyaltyBalance {
    pub creator: [u8; 32],
    pub total_accumulated: u64,
}

/// Royalty ledger tracks per-creator balances
#[derive(Debug, Clone, Default)]
pub struct RoyaltyLedger {
    pub balances: BTreeMap<[u8; 32], u64>,
}

impl RoyaltyLedger {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Settle a royalty record: credit the creator
    pub fn settle(&mut self, record: &RoyaltyRecord) {
        let entry = self.balances.entry(record.creator).or_insert(0);
        *entry = entry.saturating_add(record.royalty_amount);
    }

    /// Get balance of a creator
    pub fn balance_of(&self, creator: &[u8; 32]) -> u64 {
        self.balances.get(creator).copied().unwrap_or(0)
    }

    /// Compute deterministic accounting root
    pub fn compute_accounting_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_ROYALTY_ACCOUNTING_V1");
        for (creator, balance) in &self.balances {
            hasher.update(creator);
            hasher.update(balance.to_le_bytes());
        }
        hasher.finalize().into()
    }
}
