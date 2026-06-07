use axum::Json;

use crate::errors::{ApiError, ApiResult};
use crate::types::AccountResponse;

pub struct AccountService;

impl AccountService {
    pub fn get_balance(address: &str) -> ApiResult<AccountResponse> {
        if address.is_empty() {
            return Err(ApiError::new("INVALID_ADDRESS", "Address is empty"));
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
}
