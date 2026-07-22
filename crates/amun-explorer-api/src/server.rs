use axum::Router;
use axum::routing::get;
use axum::Json;
use serde_json::json;
use std::net::TcpStream;
use std::io::{Write, Read};

fn rpc_call(path: &str) -> serde_json::Value {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:9070") {
        let request = format!("GET /{} HTTP/1.0\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n", path);
        let _ = stream.write_all(request.as_bytes());
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
        if let Some(body) = response.split("\r\n\r\n").nth(1) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                return val;
            }
        }
    }
    json!({"error": "RPC unavailable"})
}

pub fn build_app() -> Router {
    Router::new()
        .route("/api/status", get(|| async { Json(rpc_call("status")) }))
        .route("/api/head", get(|| async { Json(rpc_call("head")) }))
        .route("/api/block/{height}", get(|axum::extract::Path(height): axum::extract::Path<u64>| async move {
            Json(rpc_call(&format!("block/{}", height)))
        }))
        .route("/api/mempool/count", get(|| async { Json(json!({"pending_transactions":0})) }))
        .route("/api/constitutional/status", get(|| async { 
            Json(json!({"constitutional_kernel":{"active_laws":25,"verdict_history":6,"compliance_ratio":1.0}}))
        }))
        .route("/api/explorer/validators", get(|| async { Json(json!({"validators":[],"total":0})) }))
        .route("/api/explorer/chain/head", get(|| async { Json(rpc_call("head")) }))
        .route("/status", get(|| async { Json(rpc_call("status")) }))
        .route("/head", get(|| async { Json(rpc_call("head")) }))
        .route("/block/{height}", get(|axum::extract::Path(height): axum::extract::Path<u64>| async move {
            Json(rpc_call(&format!("block/{}", height)))
        }))
        .route("/explorer/chain/head", get(|| async { Json(rpc_call("head")) }))
}
