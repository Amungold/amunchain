use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::constants::*;
pub struct NtrToken { pub balance: u64, pub staked: u64, pub burned: u64 }
impl NtrToken {
    pub fn new(b: u64) -> Self { Self { balance: b, staked: 0, burned: 0 } }
    pub fn transfer(&mut self, a: u64) -> AmunResult<u64> {
        if a == 0 || self.balance < a { return Err(FailureContext::new(ConstitutionalFault::InvalidInput, 0x0020, 0x0001)); }
        let tax = a * NTR_TREASURY_TAX_BPS as u64 / 10000;
        let burn = tax * NTR_BURN_TAX_BPS as u64 / 10000;
        self.balance -= a; self.burned += burn;
        Ok(tax - burn)
    }
    pub fn stake(&mut self, a: u64) -> AmunResult<()> {
        if a < NTR_MIN_STAKE_AMOUNT || self.balance < a { return Err(FailureContext::new(ConstitutionalFault::InvalidInput, 0x0020, 0x0004)); }
        self.balance -= a; self.staked += a; Ok(())
    }
    pub fn slash(&mut self, bps: u16) -> u64 {
        let amt = self.staked * bps as u64 / 10000;
        self.staked = self.staked.saturating_sub(amt);
        amt
    }
}
