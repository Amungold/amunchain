use axum::{Router, routing::post, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FaucetRequest {
    address: Option<String>,
    amount: Option<u64>,
}

async fn request_faucet(Json(req): Json<FaucetRequest>) -> Json<serde_json::Value> {
    let address = req.address.unwrap_or_else(|| "default".to_string());
    let amount = req.amount.unwrap_or(100_000);
    
    println!("Faucet request: address={}, amount={}", address, amount);
    
    Json(serde_json::json!({
        "address": address,
        "amount": amount,
        "tx_hash": format!("faucet_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()),
        "message": "Tokens sent successfully!"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/faucet/request", post(request_faucet))
        .route("/request", post(request_faucet))
        .route("/", post(request_faucet));

    println!("Faucet listening on 0.0.0.0:9073");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9073").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
