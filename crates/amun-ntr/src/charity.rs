use amun_kernel_types::PublicHash32;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::constants::*;

pub struct CharityDistributor {
    pub total_allocated: u64,
    pub total_distributed: u64,
    pub remaining: u64,
}

impl CharityDistributor {
    pub fn new() -> Self {
        let allocated = (NTR_TOTAL_SUPPLY as u128)
            .checked_mul(NTR_CHARITY_ALLOCATION_BPS as u128)
            .and_then(|v| v.checked_div(10000))
            .and_then(|v| u64::try_from(v).ok())
            .unwrap_or(0);

        Self {
            total_allocated: allocated,
            total_distributed: 0,
            remaining: allocated,
        }
    }

    pub fn distribute(&mut self, _recipient: PublicHash32, amount: u64) -> AmunResult<()> {
        if amount > self.remaining {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0020,
                0x0010,
            ));
        }
        self.total_distributed = self
            .total_distributed
            .checked_add(amount)
            .ok_or_else(|| {
                FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0020, 0x0011)
            })?;
        self.remaining = self.remaining.checked_sub(amount).unwrap_or(0);
        Ok(())
    }
}
