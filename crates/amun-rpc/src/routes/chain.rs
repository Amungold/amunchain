use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    models::{HeadResponse, MetricsResponse, StatusResponse},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/head", get(head))
        .route("/block/:height", get(block))
        .route("/blocks/:from/:to", get(block_range))
        .route("/metrics", get(metrics))
}

async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    let engine = state.engine.lock().unwrap();
    Json(StatusResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        peer_count: engine.total_validators,
    })
}

async fn head(State(state): State<AppState>) -> Result<Json<HeadResponse>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_tip() {
        Some(record) => Ok(Json(HeadResponse {
            height: record.height,
            block_hash: hex::encode(record.block_hash),
            state_root: hex::encode(record.state_root),
            history_root: hex::encode(record.history_root),
            timestamp: record.timestamp,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn block(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();

    match store.load_height(height) {
        Some(record) => Ok(Json(serde_json::json!({
            "height": record.height,
            "hash": hex::encode(record.block_hash),
            "previous_hash": "",
            "state_root": hex::encode(record.state_root),
            "timestamp": record.timestamp,
            "transaction_count": 0,
            "has_finality_certificate": true,
            "has_replay_evidence": false
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn block_range(
    State(state): State<AppState>,
    Path((from, to)): Path<(u64, u64)>,
) -> Json<serde_json::Value> {
    let store = state.store.lock().unwrap();

    let end = std::cmp::min(to, store.latest_height());
    let start = std::cmp::max(from, 1);

    let blocks: Vec<serde_json::Value> = (start..=end)
        .filter_map(|h| store.load_height(h))
        .map(|r| {
            serde_json::json!({
                "height": r.height,
                "hash": hex::encode(r.block_hash),
                "previous_hash": "",
                "state_root": hex::encode(r.state_root),
                "timestamp": r.timestamp,
                "transaction_count": 0,
                "has_finality_certificate": true,
                "has_replay_evidence": false
            })
        })
        .collect();

    Json(serde_json::Value::Array(blocks))
}

async fn metrics(State(state): State<AppState>) -> Json<MetricsResponse> {
    let engine = state.engine.lock().unwrap();
    Json(MetricsResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        rounds_active: engine.rounds.len(),
        peer_count: engine.total_validators,
    })
}
