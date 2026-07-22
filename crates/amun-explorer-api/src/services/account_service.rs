use crate::error::RpcError;
use crate::types::{AccountSummary, TransactionSummary};
use axum::extract::Path;

pub struct AccountService;

impl AccountService {
    pub fn get_account(addr: &str) -> Result<AccountSummary, RpcError> {
        if addr.is_empty() {
            return Err(RpcError::new("INVALID_REQUEST", "Address is empty"));
        }
        Ok(AccountSummary {
            address: addr.to_string(),
            balance: 100_000,
            nonce: 42,
            transaction_count: 15,
        })
    }

    pub fn get_transactions(addr: &str) -> Result<Vec<TransactionSummary>, RpcError> {
        if addr.is_empty() {
            return Err(RpcError::new("INVALID_REQUEST", "Address is empty"));
        }
        Ok(vec![
            TransactionSummary {
                hash: "0xtx1".into(),
                block_height: 847,
                sender: addr.into(),
                receiver: "0xbob".into(),
                value: 500,
                status: "confirmed".into(),
            },
        ])
    }
}
