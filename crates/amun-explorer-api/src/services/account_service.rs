use crate::errors::{ApiError, ApiResult};
use crate::types::AccountSummary;
use axum::Json;

pub struct AccountService;

impl AccountService {
    pub fn get_account(address: &str) -> ApiResult<AccountSummary> {
        if address.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Address is empty"));
        }
        Ok(Json(AccountSummary {
            address: address.to_string(),
            balance: 100_000,
            nonce: 42,
            transaction_count: 15,
        }))
    }

    pub fn get_transactions(address: &str) -> ApiResult<Vec<crate::types::TransactionSummary>> {
        if address.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Address is empty"));
        }
        Ok(Json(vec![
            crate::types::TransactionSummary {
                hash: "0xtx1".into(),
                block_height: 847,
                sender: address.to_string(),
                recipient: "0xbob".into(),
                amount: 500,
                status: "confirmed".into(),
            },
            crate::types::TransactionSummary {
                hash: "0xtx2".into(),
                block_height: 846,
                sender: "0xbob".into(),
                recipient: address.to_string(),
                amount: 300,
                status: "confirmed".into(),
            },
        ]))
    }
}
