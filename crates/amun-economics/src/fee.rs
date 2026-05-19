use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::constants::BASE_FEE;
pub struct FeeMarket { pub base_fee: u64 }
impl FeeMarket { pub fn new() -> Self { Self { base_fee: BASE_FEE } } pub fn calculate_fee(&self, gas: u64) -> AmunResult<u64> { self.base_fee.checked_mul(gas).ok_or_else(|| FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0012, 0x0010)) } }
