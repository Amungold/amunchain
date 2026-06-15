use crate::errors::{ApiError, ApiResult};
use crate::types::AccountResponse;
use amun_rpc::provider::ChainDataProvider;
use axum::Json;
use std::sync::Arc;

static PROVIDER: std::sync::OnceLock<Arc<dyn ChainDataProvider>> = std::sync::OnceLock::new();

pub fn set_provider(p: Arc<dyn ChainDataProvider>) {
    let _ = PROVIDER.set(p);
}

fn provider() -> Arc<dyn ChainDataProvider> {
    #[cfg(test)]
    {
        use amun_rpc::provider::MockProvider;
        Arc::new(MockProvider)
    }
    #[cfg(not(test))]
    {
        PROVIDER.get().expect("Provider not initialized").clone()
    }
}

pub struct AccountService;

impl AccountService {
    pub fn get_balance(address: &str) -> ApiResult<AccountResponse> {
        if address.is_empty() {
            return Err(ApiError::new("INVALID_ADDRESS", "Address is empty"));
        }
        let status = provider()
            .get_status()
            .map_err(|e| ApiError::new("RPC_ERROR", &e.to_string()))?;
        Ok(Json(AccountResponse {
            address: address.to_string(),
            balance: 0,
            nonce: status.height,
        }))
    }

    pub fn get_nonce(address: &str) -> ApiResult<AccountResponse> {
        Self::get_balance(address)
    }

    pub fn get_account(address: &str) -> ApiResult<AccountResponse> {
        Self::get_balance(address)
    }
}
