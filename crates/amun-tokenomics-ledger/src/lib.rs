use amun_tokenomics::{EpochEconomics, BLOCKS_PER_EPOCH};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct EconomicLedger {
    pub treasury_balance: u64,
    pub validator_reward_pool: u64,
    pub ecosystem_reward_pool: u64,
    pub total_issued_ntr: u64,
    pub current_epoch: u64,
    pub blocks_in_current_epoch: u64,
    pub last_epoch_reward: u64,
}

impl Default for EconomicLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl EconomicLedger {
    pub fn new() -> Self {
        Self {
            treasury_balance: 0,
            validator_reward_pool: 0,
            ecosystem_reward_pool: 0,
            total_issued_ntr: 0,
            current_epoch: 0,
            blocks_in_current_epoch: 0,
            last_epoch_reward: 0,
        }
    }

    pub fn on_block_finalized(&mut self, epoch_economics: &EpochEconomics) {
        self.end_block(epoch_economics);
    }

    pub fn end_block(&mut self, epoch_economics: &EpochEconomics) {
        self.blocks_in_current_epoch += 1;

        if self.blocks_in_current_epoch >= BLOCKS_PER_EPOCH {
            self.current_epoch += 1;
            self.blocks_in_current_epoch = 0;

            self.last_epoch_reward = epoch_economics.reward_pool;
            self.treasury_balance += epoch_economics.treasury_share;
            self.validator_reward_pool += epoch_economics.validator_share;
            self.ecosystem_reward_pool += epoch_economics.ecosystem_share;
            self.total_issued_ntr += epoch_economics.reward_pool;
        }
    }

    pub fn compute_economic_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(b"AMUN_ECONOMIC_LEDGER_V1");
        hasher.update(self.treasury_balance.to_le_bytes());
        hasher.update(self.validator_reward_pool.to_le_bytes());
        hasher.update(self.ecosystem_reward_pool.to_le_bytes());
        hasher.update(self.total_issued_ntr.to_le_bytes());
        hasher.update(self.current_epoch.to_le_bytes());

        hasher.finalize().into()
    }

    pub fn compute_ledger_root(&self) -> [u8; 32] {
        self.compute_economic_root()
    }
}
