use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConstitutionalStatusResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub evidence_root: String,
    pub verdict_hash: String,
    pub evidence_record_hash: String,
    pub slashing_root: String,
    pub timestamp: u64,
    pub commitment_root: String,
    pub constitutional_root: String,
    pub economic_root: String,
    pub identity_root: String,
    pub governance_root: String,
}

pub async fn constitutional_status(
    Path(height): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<ConstitutionalStatusResponse>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(ConstitutionalStatusResponse {
            height: record.height,
            block_hash: hex::encode(record.block_hash),
            state_root: hex::encode(record.state_root),
            evidence_root: hex::encode(record.evidence_root),
            verdict_hash: hex::encode(record.verdict_hash),
            evidence_record_hash: hex::encode(record.evidence_record_hash),
            slashing_root: hex::encode(record.slashing_root),
            timestamp: record.timestamp,
            commitment_root: hex::encode(record.commitment_root),
            constitutional_root: hex::encode(record.constitutional_root),
            economic_root: hex::encode(record.economic_root),
            identity_root: hex::encode(record.identity_root),
            governance_root: hex::encode(record.governance_root),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
