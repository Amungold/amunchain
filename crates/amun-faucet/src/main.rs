use axum::{Router, routing::post, Json};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct FaucetRequest {
    address: String,
}

async fn request_faucet(Json(req): Json<FaucetRequest>) -> Json<serde_json::Value> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    Json(serde_json::json!({
        "status": "success",
        "amount": 1000,
        "address": req.address,
        "tx_hash": format!("faucet_{}", timestamp)
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/faucet/request", post(request_faucet));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9073").await.unwrap();
    println!("Faucet listening on 9073");
    axum::serve(listener, app).await.unwrap();
}
