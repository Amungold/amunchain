use crate::token_api::TokenApi;
use crate::staking_api::StakingApi;
use crate::governance_api::GovernanceApi;
use crate::charity_api::CharityApi;
use crate::treasury_api::TreasuryApi;
use amun_ntr::NtrToken;
use crate::types::SdkResult;

pub struct Sandbox {
    pub token_api: TokenApi,
    pub staking_api: StakingApi,
    pub governance_api: GovernanceApi,
    pub charity_api: CharityApi,
    pub treasury_api: TreasuryApi,
    pub accounts: heapless::Vec<NtrToken, 16>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self { token_api: TokenApi, staking_api: StakingApi::new(), governance_api: GovernanceApi::new(), charity_api: CharityApi::new(), treasury_api: TreasuryApi::new(), accounts: heapless::Vec::new() }
    }

    pub fn create_account(&mut self, balance: u64) -> SdkResult<usize> {
        let token = NtrToken::new(balance);
        if self.accounts.push(token).is_err() { return SdkResult::err("Account limit reached"); }
        SdkResult::ok(self.accounts.len() - 1)
    }

    pub fn simulate_transfer(&mut self, from: usize, to: usize, amount: u64) -> SdkResult<u64> {
        if from >= self.accounts.len() || to >= self.accounts.len() { return SdkResult::err("Invalid account"); }
        match self.accounts[from].transfer(amount) {
            Ok(tax) => { self.accounts[to].balance += amount; SdkResult::ok(tax) }
            Err(_) => SdkResult::err("Transfer failed"),
        }
    }

    pub fn simulate_stake(&mut self, account: usize, amount: u64) -> SdkResult<()> {
        if account >= self.accounts.len() { return SdkResult::err("Invalid account"); }
        match self.accounts[account].stake(amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Stake failed"),
        }
    }
}
