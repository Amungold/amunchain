use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use amun_nft_royalty::RoyaltyRecord;
use amun_nft_royalty_accounting::RoyaltyLedger;

/// Settlement record for a royalty payout
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementRecord {
    pub creator: [u8; 32],
    pub amount: u64,
    pub block_height: u64,
    pub settled_records: Vec<RoyaltyRecord>,
}

/// Settlement engine that processes accumulated royalties
#[derive(Debug, Clone, Default)]
pub struct SettlementEngine {
    pub settlements: Vec<SettlementRecord>,
}

impl SettlementEngine {
    pub fn new() -> Self {
        Self { settlements: Vec::new() }
    }

    /// Settle outstanding royalties for a creator
    pub fn settle(
        &mut self,
        ledger: &RoyaltyLedger,
        creator: &[u8; 32],
        block_height: u64,
        records: Vec<RoyaltyRecord>,
    ) -> Option<SettlementRecord> {
        let balance = ledger.balance_of(creator);
        if balance == 0 {
            return None;
        }
        let record = SettlementRecord {
            creator: *creator,
            amount: balance,
            block_height,
            settled_records: records,
        };
        self.settlements.push(record.clone());
        Some(record)
    }

    /// Compute settlement history root
    pub fn compute_settlement_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_ROYALTY_SETTLEMENT_V1");
        for record in &self.settlements {
            let bytes = serde_json::to_vec(record).unwrap();
            hasher.update(&bytes);
        }
        hasher.finalize().into()
    }
}
