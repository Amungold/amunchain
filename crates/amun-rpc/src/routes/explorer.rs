use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/explorer/summary", get(explorer_summary))
        .route("/explorer/validators", get(explorer_validators))
        .route("/explorer/blocks/:height", get(explorer_block_detail))
}

async fn explorer_summary(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let store = state.store.lock().unwrap();
    let mempool = state.mempool.lock().unwrap();

    Json(serde_json::json!({
        "height": engine.current_height,
        "validators": engine.total_validators,
        "qcs_formed": engine.metrics.qcs_formed,
        "blocks_finalized": engine.metrics.blocks_finalized,
        "votes_received": engine.metrics.votes_received,
        "pending_transactions": mempool.pending_count(),
        "chain_head": {
            "hash": store.load_tip().map(|r| hex::encode(r.block_hash)).unwrap_or_default(),
            "state_root": store.load_tip().map(|r| hex::encode(r.state_root)).unwrap_or_default(),
        }
    }))
}

async fn explorer_validators(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let validators: Vec<serde_json::Value> = engine
        .validator_ids
        .iter()
        .map(|id| {
            let power = engine.get_validator_voting_power(id);
            serde_json::json!({
                "id": hex::encode(id),
                "voting_power": power,
                "active": power > 0
            })
        })
        .collect();

    Json(serde_json::json!({
        "total": validators.len(),
        "validators": validators
    }))
}

async fn explorer_block_detail(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(serde_json::json!({
            "height": record.height,
            "block_hash": hex::encode(record.block_hash),
            "state_root": hex::encode(record.state_root),
            "history_root": hex::encode(record.history_root),
            "timestamp": record.timestamp,
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
