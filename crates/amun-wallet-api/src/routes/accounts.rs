use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

async fn create_wallet() -> Json<serde_json::Value> {
    let keypair = amun_wallet_management::keygen::generate_keypair();
    Json(serde_json::json!({
        "address": keypair.address().to_string(),
        "public_key": keypair.public_key_hex(),
        "created": true
    }))
}

async fn get_balance(Path(address): Path<String>) -> Json<serde_json::Value> {
    // TODO: integrate real store via AppState
    Json(serde_json::json!({
        "address": address,
        "balance": 0,
        "nonce": 0,
        "status": "ok"
    }))
}

async fn get_nonce(Path(address): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "address": address,
        "nonce": 0
    }))
}

async fn get_account(Path(address): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "address": address,
        "balance": 0,
        "nonce": 0
    }))
}

pub fn account_routes() -> Router {
    Router::new()
        .route("/create", post(create_wallet))
        .route("/{address}/balance", get(get_balance))
        .route("/{address}/nonce", get(get_nonce))
        .route("/{address}", get(get_account))
}
