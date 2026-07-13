use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/mempool/count", get(mempool_count))
        .route("/tx/submit", post(submit_tx))
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    eprintln!("########## RPC SUBMIT_TX HIT ##########");
    eprintln!("[RPC] submit_tx called");
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

    eprintln!("[RPC] tx={} nonce={}", hex::encode(hash), tx.nonce);
    let mut mp = state
        .mempool
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    eprintln!("[RPC] mempool before={}", mp.pending_count());
    mp.add_transaction(tx)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    eprintln!("[RPC] mempool after={}", mp.pending_count());

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
