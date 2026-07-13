use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::constants::*;

pub struct AmungoldTreasury {
    pub balance: u64,
    pub total_collected: u64,
    pub total_distributed: u64,
    pub development_fund: u64,
    pub infrastructure_fund: u64,
}

impl AmungoldTreasury {
    pub fn new() -> Self {
        let allocated = (NTR_TOTAL_SUPPLY as u128)
            .checked_mul(NTR_TREASURY_ALLOCATION_BPS as u128)
            .and_then(|v| v.checked_div(10000))
            .and_then(|v| u64::try_from(v).ok())
            .unwrap_or(0);

        Self {
            balance: allocated,
            total_collected: allocated,
            total_distributed: 0,
            development_fund: 0,
            infrastructure_fund: 0,
        }
    }

    pub fn deposit_tax(&mut self, amount: u64) -> AmunResult<()> {
        self.balance = self.balance.checked_add(amount).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0020, 0x0020)
        })?;
        self.total_collected = self.total_collected.checked_add(amount).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0020, 0x0021)
        })?;
        Ok(())
    }

    pub fn allocate_development(&mut self, amount: u64) -> AmunResult<()> {
        if amount > self.balance {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0020,
                0x0022,
            ));
        }
        self.balance = self.balance.checked_sub(amount).unwrap_or(0);
        self.development_fund = self
            .development_fund
            .checked_add(amount)
            .unwrap_or(self.development_fund);
        Ok(())
    }

    pub fn allocate_infrastructure(&mut self, amount: u64) -> AmunResult<()> {
        if amount > self.balance {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0020,
                0x0023,
            ));
        }
        self.balance = self.balance.checked_sub(amount).unwrap_or(0);
        self.infrastructure_fund = self
            .infrastructure_fund
            .checked_add(amount)
            .unwrap_or(self.infrastructure_fund);
        Ok(())
    }
}
