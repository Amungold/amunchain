use amun_tokenomics::{EpochEconomics, BLOCKS_PER_EPOCH};
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// N132 Economic Execution Integration — Phase 1
// ---------------------------------------------------------------------------

/// Economic changes applied to the ledger for a single block.
#[derive(Debug, Clone, Default)]
pub struct EconomicDelta {
    pub treasury_deposit: u64,
    pub validator_reward: u64,
    pub ecosystem_deposit: u64,
    pub burn_amount: u64,
    /// Positive = stake, negative = unstake
    pub staked_delta: i64,
}



impl EconomicDelta {
    /// Merge another delta into this one using saturating arithmetic.
    pub fn merge(&mut self, other: &EconomicDelta) {
        self.treasury_deposit = self
            .treasury_deposit
            .saturating_add(other.treasury_deposit);

        self.validator_reward = self
            .validator_reward
            .saturating_add(other.validator_reward);

        self.ecosystem_deposit = self
            .ecosystem_deposit
            .saturating_add(other.ecosystem_deposit);

        self.burn_amount = self
            .burn_amount
            .saturating_add(other.burn_amount);

        self.staked_delta = self
            .staked_delta
            .saturating_add(other.staked_delta);
    }

    /// Convenience wrapper.
    pub fn add_delta(&mut self, other: &EconomicDelta) {
        self.merge(other);
    }
}
#[derive(Debug, Clone)]
pub struct EconomicLedger {
    pub treasury_balance: u64,
    pub validator_reward_pool: u64,
    pub ecosystem_reward_pool: u64,
    pub total_issued_ntr: u64,
    pub current_epoch: u64,
    pub blocks_in_current_epoch: u64,
    pub last_epoch_reward: u64,
    pub burned_supply: u64,
    pub staked_supply: u64,
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
            burned_supply: 0,
            staked_supply: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------
    pub fn treasury(&self) -> u64 { self.treasury_balance }
    pub fn validator_pool(&self) -> u64 { self.validator_reward_pool }
    pub fn ecosystem_pool(&self) -> u64 { self.ecosystem_reward_pool }
    pub fn issued_supply(&self) -> u64 { self.total_issued_ntr }
    pub fn burned_supply(&self) -> u64 { self.burned_supply }
    pub fn staked_supply(&self) -> u64 { self.staked_supply }

    // -----------------------------------------------------------------------
    // Atomic mutations
    // -----------------------------------------------------------------------
    pub fn deposit_treasury(&mut self, amount: u64) {
        self.treasury_balance = self.treasury_balance.saturating_add(amount);
    }
    pub fn credit_validator_rewards(&mut self, amount: u64) {
        self.validator_reward_pool = self.validator_reward_pool.saturating_add(amount);
    }
    pub fn credit_ecosystem(&mut self, amount: u64) {
        self.ecosystem_reward_pool = self.ecosystem_reward_pool.saturating_add(amount);
    }
    pub fn burn_supply(&mut self, amount: u64) {
        self.burned_supply = self.burned_supply.saturating_add(amount);
    }
    pub fn stake_supply(&mut self, amount: u64) {
        self.staked_supply = self.staked_supply.saturating_add(amount);
    }
    pub fn unstake_supply(&mut self, amount: u64) {
        self.staked_supply = self.staked_supply.saturating_sub(amount);
    }

    // -----------------------------------------------------------------------
    // Batch update — canonical way to apply block-level economics
    // -----------------------------------------------------------------------
    pub fn apply_block_economics(&mut self, delta: &EconomicDelta) {
        self.deposit_treasury(delta.treasury_deposit);
        self.credit_validator_rewards(delta.validator_reward);
        self.credit_ecosystem(delta.ecosystem_deposit);
        self.burn_supply(delta.burn_amount);
        if delta.staked_delta > 0 {
            self.stake_supply(delta.staked_delta as u64);
        } else if delta.staked_delta < 0 {
            self.unstake_supply((-delta.staked_delta) as u64);
        }
    }

    // -----------------------------------------------------------------------
    // End‑of‑block logic (unchanged)
    // -----------------------------------------------------------------------
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
        hasher.update(self.burned_supply.to_le_bytes());
        hasher.update(self.staked_supply.to_le_bytes());

        hasher.finalize().into()
    }

    pub fn compute_ledger_root(&self) -> [u8; 32] {
        self.compute_economic_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ledger_all_zeros() {
        let ledger = EconomicLedger::new();
        assert_eq!(ledger.treasury(), 0);
        assert_eq!(ledger.validator_pool(), 0);
        assert_eq!(ledger.ecosystem_pool(), 0);
        assert_eq!(ledger.issued_supply(), 0);
        assert_eq!(ledger.burned_supply(), 0);
        assert_eq!(ledger.staked_supply(), 0);
    }

    #[test]
    fn deposit_treasury_increases() {
        let mut ledger = EconomicLedger::new();
        ledger.deposit_treasury(100);
        assert_eq!(ledger.treasury(), 100);
    }

    #[test]
    fn burn_supply_increases() {
        let mut ledger = EconomicLedger::new();
        ledger.burn_supply(50);
        assert_eq!(ledger.burned_supply(), 50);
    }

    #[test]
    fn stake_and_unstake() {
        let mut ledger = EconomicLedger::new();
        ledger.stake_supply(200);
        assert_eq!(ledger.staked_supply(), 200);
        ledger.unstake_supply(50);
        assert_eq!(ledger.staked_supply(), 150);
    }

    #[test]
    fn apply_block_economics_batch() {
        let mut ledger = EconomicLedger::new();
        let delta = EconomicDelta {
            treasury_deposit: 10,
            validator_reward: 5,
            ecosystem_deposit: 3,
            burn_amount: 2,
            staked_delta: 100,
        };
        ledger.apply_block_economics(&delta);
        assert_eq!(ledger.treasury(), 10);
        assert_eq!(ledger.validator_pool(), 5);
        assert_eq!(ledger.ecosystem_pool(), 3);
        assert_eq!(ledger.burned_supply(), 2);
        assert_eq!(ledger.staked_supply(), 100);
    }

    #[test]
    fn economic_root_changes_on_mutation() {
        let mut ledger = EconomicLedger::new();
        let root1 = ledger.compute_economic_root();
        ledger.deposit_treasury(100);
        let root2 = ledger.compute_economic_root();
        assert_ne!(root1, root2);
    }
}
