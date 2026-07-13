pub mod rpc_client;

use axum::http::Method;
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;

#[derive(Clone)]
pub struct AppState {
    pub consensus: Arc<Mutex<ConsensusEngine>>,
    pub chain_store: Arc<Mutex<ChainStore>>,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct BlockResponse {
    height: u64,
    hash: String,
    state_root: String,
    evidence_root: String,
}

async fn api_status(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let status = rpc_client::status().await.unwrap();
    let metrics = rpc_client::metrics().await.unwrap();
    let mempool = rpc_client::mempool().await.unwrap();

    Json(serde_json::json!({
        "height": status.height,
        "peer_count": status.peer_count,
        "qcs_formed": metrics.qcs_formed,
        "blocks_finalized": metrics.blocks_finalized,
        "votes_received": metrics.votes_received,
        "rounds_active": metrics.rounds_active,
        "pending_transactions": mempool.pending_transactions
    }))
}

async fn api_blocks(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let head = rpc_client::head().await.unwrap();

    Json(serde_json::json!([{
        "height": head.height,
        "hash": head.block_hash,
        "state_root": head.state_root,
        "history_root": head.history_root,
        "timestamp": head.timestamp
    }]))
}

async fn api_metrics(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let m = rpc_client::metrics().await.unwrap();

    Json(serde_json::json!(m))
}

async fn api_head(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let h = rpc_client::head().await.unwrap();

    Json(serde_json::json!(h))
}

async fn api_mempool(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let m = rpc_client::mempool().await.unwrap();

    Json(serde_json::json!(m))
}

async fn api_constitutional(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let c = rpc_client::constitutional().await.unwrap();

    Json(c)
}

use axum::extract::Path;

async fn api_block(
    Path(height): Path<u64>,
    State(_state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(rpc_client::block(height).await.unwrap())
}

async fn api_block_range(
    Path((from, to)): Path<(u64, u64)>,
    State(_state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(rpc_client::block_range(from, to).await.unwrap())
}

async fn api_validators(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(rpc_client::validators().await.unwrap())
}
pub async fn serve(state: AppState, port: u16) {
    let app_state = Arc::new(state);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/status", get(api_status))
        .route("/api/head", get(api_head))
        .route("/api/metrics", get(api_metrics))
        .route("/api/mempool", get(api_mempool))
        .route("/api/constitutional", get(api_constitutional))
        .route("/api/block/:height", get(api_block))
        .route("/api/blocks/:from/:to", get(api_block_range))
        .route("/api/validators", get(api_validators))
        .route("/api/blocks", get(api_blocks))
        .layer(cors)
        .with_state(app_state);

    let addr = format!("0.0.0.0:{port}");

    println!("Constitutional Explorer API running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
