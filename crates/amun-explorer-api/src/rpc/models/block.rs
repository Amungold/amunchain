use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcBlockResponse {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub validator: String,
    pub transaction_count: u64,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub validator: String,
    pub transaction_count: u64,
    pub size_bytes: u64,
}

impl From<RpcBlockResponse> for Block {
    fn from(rpc: RpcBlockResponse) -> Self {
        Self {
            height: rpc.height,
            hash: rpc.hash,
            previous_hash: rpc.previous_hash,
            timestamp: rpc.timestamp,
            validator: rpc.validator,
            transaction_count: rpc.transaction_count,
            size_bytes: rpc.size_bytes,
        }
    }
}
