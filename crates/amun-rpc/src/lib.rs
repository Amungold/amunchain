pub mod client;
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
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
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
    State(state): State<AppState>,
    Path((from, to)): Path<(u64, u64)>,
) -> Json<RangeResponse> {
    let store = state.store.lock().unwrap();
    let end = std::cmp::min(to, store.latest_height());
    let start = std::cmp::max(from, 1);
    let blocks: Vec<BlockResponse> = (start..=end)
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
        rounds_active: engine.rounds.len(),
        peer_count: engine.total_validators,
    })
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tx_bytes = hex::decode(body["transaction_bytes"].as_str().unwrap_or(""))
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut tx: amun_transactions::Transaction =
        serde_json::from_slice(&tx_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(sig_hex) = body["signature"].as_str() {
        if let Ok(sig) = hex::decode(sig_hex) {
            if sig.len() == 64 {
                tx.signature = sig;
            }
        }
    }

    if !tx.verify() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hash = tx.tx_hash();
    let mut mp = state
        .mempool
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mp.add_transaction(tx)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(serde_json::json!({
        "hash": hex::encode(hash),
        "status": "pending"
    })))
}

async fn mempool_count(State(state): State<AppState>) -> Json<serde_json::Value> {
    let count = state.mempool.lock().unwrap().pending_count();
    Json(serde_json::json!({
        "pending_transactions": count
    }))
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
        .with_state(state)
}

pub async fn serve(state: AppState, port: u16) {
    let app = build_app(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    eprintln!("RPC server listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
pub mod provider;
