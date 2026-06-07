use serde::{Deserialize, Serialize};

/// A gossip message for block headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    pub block_height: u64,
    pub block_hash: String,
    pub state_root: String,
}
