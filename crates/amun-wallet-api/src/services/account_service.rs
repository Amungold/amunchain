use crate::errors::{ApiError, ApiResult};
use crate::types::AccountResponse;
use axum::Json;

pub struct AccountService;

impl AccountService {
    pub fn get_balance(address: &str) -> ApiResult<AccountResponse> {
        if address.is_empty() {
            return Err(ApiError::new("INVALID_ADDRESS", "Address cannot be empty"));
        }
        Ok(Json(AccountResponse {
            address: address.to_string(),
            balance: 0,
            nonce: 0,
        }))
    }

    pub fn get_nonce(address: &str) -> ApiResult<AccountResponse> {
        Self::get_balance(address)
    }

    pub fn get_account(address: &str) -> ApiResult<AccountResponse> {
        Self::get_balance(address)
    }

    /// Configure the RPC provider URL for live chain access.
    pub fn set_provider(_url: &str) {
        // TODO: Store provider configuration for future RPC calls
    }
}
