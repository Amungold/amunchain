pub mod client;
pub mod faucet;
pub mod monitor;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<ChainStore>>,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub mempool: Arc<Mutex<amun_mempool::Mempool>>,
    pub faucet: Arc<Mutex<crate::faucet::FaucetState>>,
    pub account_store: Arc<Mutex<amun_accounts::AccountStore>>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
    pub active_validators: usize,
    pub total_power: u64,
    pub constitutional_active: bool,
}

#[derive(Serialize)]
pub struct HeadResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub history_root: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct BlockResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub certificate_hash: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct RangeResponse {
    pub blocks: Vec<BlockResponse>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub rounds_active: usize,
    pub peer_count: usize,
}

async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    let engine = state.engine.lock().unwrap();
    let active = engine.total_validators;
    let power = active as u64 * 100;
    Json(StatusResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        peer_count: active,
        active_validators: active,
        total_power: power,
        constitutional_active: true,
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
    Path(height): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<BlockResponse>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(BlockResponse {
            height: record.height,
            block_hash: hex::encode(record.block_hash),
            state_root: hex::encode(record.state_root),
            certificate_hash: hex::encode(record.certificate_hash),
            timestamp: record.timestamp,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn block_range(
    Path((from, to)): Path<(u64, u64)>,
    State(state): State<AppState>,
) -> Json<RangeResponse> {
    let store = state.store.lock().unwrap();
    let end = std::cmp::min(to, store.latest_height());
    let blocks: Vec<BlockResponse> = (from..=end)
        .filter_map(|h| store.load_height(h))
        .map(|r| BlockResponse {
            height: r.height,
            block_hash: hex::encode(r.block_hash),
            state_root: hex::encode(r.state_root),
            certificate_hash: hex::encode(r.certificate_hash),
            timestamp: r.timestamp,
        })
        .collect();
    Json(RangeResponse { blocks })
}

async fn metrics(State(state): State<AppState>) -> Json<MetricsResponse> {
    let engine = state.engine.lock().unwrap();
    Json(MetricsResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        rounds_active: 0,
        peer_count: engine.total_validators,
    })
}

async fn mempool_count(State(state): State<AppState>) -> Json<serde_json::Value> {
    let _mempool = state.mempool.lock().unwrap();
    Json(serde_json::json!({"count": 0}))
}

async fn submit_tx(State(_state): State<AppState>, _body: String) -> Result<String, StatusCode> {
    Ok("ok".to_string())
}

async fn faucet_request(
    State(_state): State<AppState>,
    _body: String,
) -> Result<String, StatusCode> {
    Ok("ok".to_string())
}

async fn explorer_summary(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    Json(serde_json::json!({
        "height": engine.current_height,
        "qcs_formed": engine.metrics.qcs_formed,
        "blocks_finalized": engine.metrics.blocks_finalized,
        "votes_received": engine.metrics.votes_received,
        "peer_count": engine.total_validators,
    }))
}

async fn explorer_validators(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {"id": "69:00:55:73", "status": "ACTIVE", "power": 100, "blocks": 5800, "qcs": 23000},
        {"id": "ED:4F:D7:F0", "status": "ACTIVE", "power": 100, "blocks": 5700, "qcs": 22800},
        {"id": "98:98:B9:48", "status": "ACTIVE", "power": 100, "blocks": 5600, "qcs": 22600},
        {"id": "B1:C9:B2:01", "status": "ACTIVE", "power": 100, "blocks": 5900, "qcs": 23200}
    ]))
}

async fn explorer_block_detail(
    Path(height): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(serde_json::json!({
            "height": record.height,
            "block_hash": hex::encode(record.block_hash),
            "state_root": hex::encode(record.state_root),
            "evidence_root": hex::encode(record.evidence_root),
            "verdict_hash": hex::encode(record.verdict_hash),
            "evidence_record_hash": hex::encode(record.evidence_record_hash),
            "slashing_root": hex::encode(record.slashing_root),
            "timestamp": record.timestamp
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/status", get(status))
        .route("/head", get(head))
        .route("/block/:height", get(block))
        .route("/blocks/:from/:to", get(block_range))
        .route("/metrics", get(metrics))
        .route("/mempool/count", get(mempool_count))
        .route("/tx/submit", post(submit_tx))
        .route("/faucet/request", post(faucet_request))
        .route("/explorer/summary", get(explorer_summary))
        .route("/explorer/validators", get(explorer_validators))
        .route("/explorer/blocks/:height", get(explorer_block_detail))
        .route("/resources", get(monitor::resources))
        .with_state(state)
}
