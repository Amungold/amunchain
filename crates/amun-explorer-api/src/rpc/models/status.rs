use serde::Deserialize;

/// Raw response from amun-rpc GET /status
#[derive(Debug, Clone, Deserialize)]
pub struct RpcStatusResponse {
    pub node_id: String,
    pub version: String,
    pub network: String,
    pub uptime_seconds: u64,
    pub connected_peers: usize,
}

/// Explorer domain model
#[derive(Debug, Clone)]
pub struct NodeStatus {
    pub node_id: String,
    pub version: String,
    pub network: String,
    pub uptime_seconds: u64,
    pub connected_peers: usize,
}

impl From<RpcStatusResponse> for NodeStatus {
    fn from(rpc: RpcStatusResponse) -> Self {
        Self {
            node_id: rpc.node_id,
            version: rpc.version,
            network: rpc.network,
            uptime_seconds: rpc.uptime_seconds,
            connected_peers: rpc.connected_peers,
        }
    }
}
