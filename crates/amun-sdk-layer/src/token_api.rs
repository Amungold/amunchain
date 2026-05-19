use amun_ntr::NtrToken;
use crate::types::SdkResult;

pub struct TokenApi;

impl TokenApi {
    pub fn create_account(initial_balance: u64) -> SdkResult<NtrToken> {
        SdkResult::ok(NtrToken::new(initial_balance))
    }

    pub fn transfer(token: &mut NtrToken, amount: u64) -> SdkResult<u64> {
        match token.transfer(amount) {
            Ok(treasury) => SdkResult::ok(treasury),
            Err(_) => SdkResult::err("Transfer failed"),
        }
    }

    pub fn stake(token: &mut NtrToken, amount: u64) -> SdkResult<()> {
        match token.stake(amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Stake failed"),
        }
    }

    pub fn get_balance(token: &NtrToken) -> SdkResult<u64> {
        SdkResult::ok(token.balance)
    }

    pub fn get_staked(token: &NtrToken) -> SdkResult<u64> {
        SdkResult::ok(token.staked)
    }
}
