pub struct EconomicSnapshot {
    pub total_supply: u64,
    pub treasury_balance: u64,
    pub validator_reward_pool: u64,
    pub ecosystem_pool: u64,
    pub burned_supply: u64,
    pub issued_supply: u64,
    pub staked_supply: u64,
    pub circulating_supply: u64,
}

pub const ECONOMIC_SCHEMA_VERSION: u16 = 1;
