use crate::constants::*;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub balance: u64,
    pub staked: u64,
    pub locked: u64,
    pub nonce: u64,
}
impl Token {
    pub fn new(b: u64) -> Self {
        Self {
            balance: b,
            staked: 0,
            locked: 0,
            nonce: 0,
        }
    }
    pub fn total_balance(&self) -> u64 {
        self.balance.saturating_add(self.staked)
    }
    pub fn available_balance(&self) -> u64 {
        self.balance.saturating_sub(self.locked)
    }
    pub fn transfer(&mut self, amount: u64) -> AmunResult<()> {
        if amount == 0 || self.available_balance() < amount {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0012,
                0x0001,
            ));
        }
        self.balance = self.balance.checked_sub(amount).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0012, 0x0003)
        })?;
        self.nonce = self.nonce.checked_add(1).unwrap_or(0);
        Ok(())
    }
    pub fn stake(&mut self, amount: u64) -> AmunResult<()> {
        if amount < MIN_STAKE_AMOUNT || self.available_balance() < amount {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0012,
                0x0004,
            ));
        }
        self.balance = self.balance.checked_sub(amount).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0012, 0x0006)
        })?;
        self.staked = self.staked.checked_add(amount).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0012, 0x0007)
        })?;
        Ok(())
    }
    pub fn slash(&mut self, bps: u16) -> AmunResult<u64> {
        let amt = self
            .staked
            .checked_mul(bps as u64)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0);
        self.staked = self.staked.saturating_sub(amt);
        Ok(amt)
    }
}
