use axum::{Router, routing::get, Json, extract::State};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

use amun_consensus_network::engine::ConsensusEngine;
use amun_chain_store::store::ChainStore;

#[derive(Clone)]
pub struct AppState {
    pub consensus: Arc<Mutex<ConsensusEngine>>,
    pub chain_store: Arc<Mutex<ChainStore>>,
}

// ===== API Response Types =====
#[derive(Serialize)]
struct StatusResponse {
    height: u64,
    active_validators: usize,
    total_power: u64,
}

#[derive(Serialize)]
struct BlockResponse {
    height: u64,
    hash: String,
    state_root: String,
    evidence_root: String,
}

// ===== Route Handlers =====
async fn api_status(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let consensus = state.consensus.lock().unwrap();
    
    Json(StatusResponse {
        height: consensus.current_height,
        active_validators: consensus.active_validator_count(),
        total_power: consensus.total_voting_power,
    })
}

async fn api_blocks(State(state): State<Arc<AppState>>) -> Json<Vec<BlockResponse>> {
    let store = state.chain_store.lock().unwrap();
    let mut blocks = Vec::new();
    
    if let Some(record) = store.load_tip() {
        blocks.push(BlockResponse {
            height: record.height,
            hash: hex::encode(&record.block_hash),
            state_root: hex::encode(&record.state_root),
            evidence_root: hex::encode(&record.evidence_root),
        });
    }
    
    Json(blocks)
}

// ===== Server Startup =====
pub async fn serve(state: AppState, port: u16) {
    let app_state = Arc::new(state);
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/status", get(api_status))
        .route("/api/blocks", get(api_blocks))
        .layer(cors)
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    println!("Constitutional Explorer API running on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
