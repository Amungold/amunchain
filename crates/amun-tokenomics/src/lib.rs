pub mod policy;

use amun_ntr::constants::{
    NTR_ECOSYSTEM_ALLOCATION_BPS, NTR_INITIAL_INFLATION_BPS, NTR_STAKING_ALLOCATION_BPS,
    NTR_TREASURY_ALLOCATION_BPS,
};

pub const BLOCKS_PER_EPOCH: u64 = 216_000;

#[derive(Debug, Clone)]
pub struct EpochEconomics {
    pub epoch: u64,
    pub reward_pool: u64,
    pub treasury_share: u64,
    pub validator_share: u64,
    pub ecosystem_share: u64,
}

impl Default for EpochEconomics {
    fn default() -> Self {
        Self::new()
    }
}

impl EpochEconomics {
    pub fn new() -> Self {
        Self {
            epoch: 0,
            reward_pool: 0,
            treasury_share: 0,
            validator_share: 0,
            ecosystem_share: 0,
        }
    }

    pub fn compute_epoch_rewards(total_supply: u64) -> u64 {
        ((total_supply as u128 * NTR_INITIAL_INFLATION_BPS as u128) / 10000u128) as u64
    }

    pub fn compute_distribution(reward: u64) -> (u64, u64, u64) {
        let treasury = (reward as u128 * NTR_TREASURY_ALLOCATION_BPS as u128 / 10000u128) as u64;

        let validators = (reward as u128 * NTR_STAKING_ALLOCATION_BPS as u128 / 10000u128) as u64;

        let ecosystem = (reward as u128 * NTR_ECOSYSTEM_ALLOCATION_BPS as u128 / 10000u128) as u64;

        (treasury, validators, ecosystem)
    }

    pub fn distribute_epoch(&mut self, total_new_ntr: u64) {
        self.reward_pool = total_new_ntr;

        let (treasury, validators, ecosystem) = Self::compute_distribution(total_new_ntr);

        self.treasury_share = treasury;
        self.validator_share = validators;
        self.ecosystem_share = ecosystem;
    }

    pub fn inflation_rate_bps(&self) -> u64 {
        NTR_INITIAL_INFLATION_BPS as u64
    }
}
