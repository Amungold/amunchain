use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcMempoolCountResponse {
    pub pending_count: u64,
    pub total_size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct MempoolCount {
    pub pending_count: u64,
    pub total_size_bytes: u64,
}

impl From<RpcMempoolCountResponse> for MempoolCount {
    fn from(rpc: RpcMempoolCountResponse) -> Self {
        Self {
            pending_count: rpc.pending_count,
            total_size_bytes: rpc.total_size_bytes,
        }
    }
}
