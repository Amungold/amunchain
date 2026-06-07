use axum::{extract::Path, routing::get, routing::post, Json, Router};
use crate::services::transaction_service::TransactionService;
use crate::types::{BuildTransactionRequest, SubmitTransactionRequest};

async fn build_transaction(Json(req): Json<BuildTransactionRequest>) -> crate::errors::ApiResult<crate::types::BuildTransactionResponse> {
    TransactionService::build_transaction(req)
}
async fn submit_transaction(Json(req): Json<SubmitTransactionRequest>) -> crate::errors::ApiResult<crate::types::SubmitTransactionResponse> {
    TransactionService::submit_transaction(req)
}
async fn get_transaction_by_hash(Path(hash): Path<String>) -> crate::errors::ApiResult<crate::types::TransactionStatusResponse> {
    TransactionService::get_transaction(&hash)
}
async fn get_transaction_status(Path(hash): Path<String>) -> crate::errors::ApiResult<crate::types::TransactionStatusResponse> {
    TransactionService::get_transaction_status(&hash)
}
pub fn transaction_routes() -> Router {
    Router::new()
        .route("/build", post(build_transaction))
        .route("/submit", post(submit_transaction))
        .route("/status/{hash}", get(get_transaction_status))
        .route("/{hash}", get(get_transaction_by_hash))
}
