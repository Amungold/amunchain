use axum::{extract::Path, routing::get, Router};
use crate::services::account_service::AccountService;

async fn get_account(Path(address): Path<String>) -> crate::errors::ApiResult<crate::types::AccountSummary> {
    AccountService::get_account(&address)
}
async fn get_transactions(Path(address): Path<String>) -> crate::errors::ApiResult<Vec<crate::types::TransactionSummary>> {
    AccountService::get_transactions(&address)
}

pub fn account_routes() -> Router {
    Router::new()
        .route("/{address}", get(get_account))
        .route("/{address}/transactions", get(get_transactions))
}
