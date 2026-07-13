use reqwest::Client;
use serde::{Deserialize, Serialize};

const RPC: &str = "http://127.0.0.1:9070";

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeadResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub history_root: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetricsResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub rounds_active: usize,
    pub peer_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MempoolResponse {
    pub pending_transactions: usize,
}

pub async fn status() -> Result<StatusResponse, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/status"))
        .send()
        .await?
        .json()
        .await
}

pub async fn head() -> Result<HeadResponse, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/head"))
        .send()
        .await?
        .json()
        .await
}

pub async fn metrics() -> Result<MetricsResponse, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/metrics"))
        .send()
        .await?
        .json()
        .await
}

pub async fn mempool() -> Result<MempoolResponse, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/mempool/count"))
        .send()
        .await?
        .json()
        .await
}

pub async fn constitutional() -> Result<serde_json::Value, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/constitutional/status"))
        .send()
        .await?
        .json()
        .await
}

#[derive(Debug, serde::Deserialize)]
pub struct BlockResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub certificate_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct RangeResponse {
    pub blocks: Vec<BlockResponse>,
}

pub async fn blocks(from: u64, to: u64) -> Result<RangeResponse, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/blocks/{}/{}", from, to))
        .send()
        .await?
        .json()
        .await
}

pub async fn block(height: u64) -> Result<serde_json::Value, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/block/{height}"))
        .send()
        .await?
        .json()
        .await
}

pub async fn block_range(from: u64, to: u64) -> Result<serde_json::Value, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/blocks/{from}/{to}"))
        .send()
        .await?
        .json()
        .await
}

pub async fn validators() -> Result<serde_json::Value, reqwest::Error> {
    Client::new()
        .get(format!("{RPC}/explorer/validators"))
        .send()
        .await?
        .json()
        .await
}
