use crate::services::chain_service::ChainService;
use axum::{extract::Path, routing::get, Router};

async fn get_head() -> crate::errors::ApiResult<crate::types::BlockSummary> {
    ChainService::get_head()
}
async fn get_block_by_height(
    Path(height): Path<u64>,
) -> crate::errors::ApiResult<crate::types::BlockSummary> {
    ChainService::get_block_by_height(height)
}
async fn get_block_by_hash(
    Path(hash): Path<String>,
) -> crate::errors::ApiResult<crate::types::BlockSummary> {
    ChainService::get_block_by_hash(&hash)
}
async fn get_transaction(
    Path(hash): Path<String>,
) -> crate::errors::ApiResult<crate::types::TransactionSummary> {
    ChainService::get_transaction(&hash)
}

pub fn chain_routes() -> Router {
    Router::new()
        .route("/head", get(get_head))
        .route("/block/{height}", get(get_block_by_height))
        .route("/block/{hash}", get(get_block_by_hash))
        .route("/transaction/{hash}", get(get_transaction))
}

pub fn validator_routes() -> axum::Router {
    use axum::routing::get;
    use axum::Json;
    use serde_json::json;
    axum::Router::new()
        .route("/validators", get(|| async { 
            Json(json!({"validators":[], "total":0}))
        }))
}

pub fn mempool_routes() -> axum::Router {
    use axum::routing::get;
    use axum::Json;
    use serde_json::json;
    axum::Router::new()
        .route("/count", get(|| async { 
            Json(json!({"pending_transactions": 0}))
        }))
}
