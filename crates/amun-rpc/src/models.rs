use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
}

#[derive(Serialize)]
pub struct HeadResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub history_root: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct BlockResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub certificate_hash: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct RangeResponse {
    pub blocks: Vec<BlockResponse>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub rounds_active: usize,
    pub peer_count: usize,
}
