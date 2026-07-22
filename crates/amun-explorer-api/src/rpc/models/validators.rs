use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcValidatorResponse {
    pub node_id: String,
    pub stake: u64,
    pub is_active: bool,
    pub uptime_percent: f64,
    pub blocks_produced: u64,
}

#[derive(Debug, Clone)]
pub struct Validator {
    pub node_id: String,
    pub stake: u64,
    pub is_active: bool,
    pub uptime_percent: f64,
    pub blocks_produced: u64,
}

impl From<RpcValidatorResponse> for Validator {
    fn from(rpc: RpcValidatorResponse) -> Self {
        Self {
            node_id: rpc.node_id,
            stake: rpc.stake,
            is_active: rpc.is_active,
            uptime_percent: rpc.uptime_percent,
            blocks_produced: rpc.blocks_produced,
        }
    }
}
