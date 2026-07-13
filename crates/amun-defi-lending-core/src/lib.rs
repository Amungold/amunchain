use amun_resource_core::ResourceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanPosition {
    pub loan_id: ResourceId,
    pub borrower: [u8; 32],
    pub principal: u64,
    pub outstanding: u64,
    pub interest_rate_bps: u64,
    pub start_height: u64,
    pub last_interest_height: u64,
    pub collateral_locked: u64,
    pub collateral_token: ResourceId,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralPosition {
    pub collateral_id: ResourceId,
    pub loan_id: ResourceId,
    pub owner: [u8; 32],
    pub amount: u64,
    pub token: ResourceId,
    pub locked: bool,
}

pub struct InterestModel;

impl InterestModel {
    pub fn compute_interest(principal: u64, rate_bps: u64, blocks_elapsed: u64) -> u64 {
        let annual_rate = rate_bps as u128;
        let blocks_per_year: u128 = 2_102_400;
        let interest =
            (principal as u128 * annual_rate * blocks_elapsed as u128) / (10_000 * blocks_per_year);
        interest as u64
    }

    pub fn compute_health_factor(collateral: u64, debt: u64, liquidation_threshold: u64) -> u64 {
        if debt == 0 {
            return u64::MAX;
        }
        (collateral as u128 * liquidation_threshold as u128 / (debt as u128 * 10_000)) as u64
    }

    pub fn is_liquidatable(health_factor: u64) -> bool {
        health_factor < 10_000
    }
}
