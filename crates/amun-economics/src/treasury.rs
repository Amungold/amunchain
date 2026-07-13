use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
pub struct Treasury {
    pub balance: u64,
}
impl Treasury {
    pub fn new() -> Self {
        Self { balance: 0 }
    }
    pub fn deposit(&mut self, a: u64) -> AmunResult<()> {
        self.balance = self.balance.checked_add(a).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0012, 0x0020)
        })?;
        Ok(())
    }
    pub fn withdraw(&mut self, a: u64) -> AmunResult<()> {
        if a > self.balance {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x0012,
                0x0022,
            ));
        }
        self.balance = self.balance.saturating_sub(a);
        Ok(())
    }
}

impl Default for Treasury {
    fn default() -> Self {
        Self::new()
    }
}
