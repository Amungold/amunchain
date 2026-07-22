use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcHeadResponse {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub validator: String,
}

#[derive(Debug, Clone)]
pub struct ChainHead {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub validator: String,
}

impl From<RpcHeadResponse> for ChainHead {
    fn from(rpc: RpcHeadResponse) -> Self {
        Self {
            height: rpc.height,
            hash: rpc.hash,
            timestamp: rpc.timestamp,
            validator: rpc.validator,
        }
    }
}
