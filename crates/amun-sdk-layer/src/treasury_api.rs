use amun_ntr::treasury::AmungoldTreasury;
use crate::types::SdkResult;

pub struct TreasuryApi {
    pub treasury: AmungoldTreasury,
}

impl TreasuryApi {
    pub fn new() -> Self { Self { treasury: AmungoldTreasury::new() } }

    pub fn deposit_tax(&mut self, amount: u64) -> SdkResult<()> {
        match self.treasury.deposit_tax(amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Deposit failed"),
        }
    }

    pub fn allocate_development(&mut self, amount: u64) -> SdkResult<()> {
        match self.treasury.allocate_development(amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Allocation failed"),
        }
    }

    pub fn get_balance(&self) -> SdkResult<u64> { SdkResult::ok(self.treasury.balance) }
    pub fn get_development_fund(&self) -> SdkResult<u64> { SdkResult::ok(self.treasury.development_fund) }
}
