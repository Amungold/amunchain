use axum::{extract::Path, routing::get, Router};
use crate::services::account_service::AccountService;

async fn get_balance(Path(address): Path<String>) -> crate::errors::ApiResult<crate::types::AccountResponse> {
    AccountService::get_balance(&address)
}
async fn get_nonce(Path(address): Path<String>) -> crate::errors::ApiResult<crate::types::AccountResponse> {
    AccountService::get_nonce(&address)
}
async fn get_account(Path(address): Path<String>) -> crate::errors::ApiResult<crate::types::AccountResponse> {
    AccountService::get_account(&address)
}
pub fn account_routes() -> Router {
    Router::new()
        .route("/{address}/balance", get(get_balance))
        .route("/{address}/nonce", get(get_nonce))
        .route("/{address}", get(get_account))
}
