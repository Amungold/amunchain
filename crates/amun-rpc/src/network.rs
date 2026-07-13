use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::AppState;

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}
