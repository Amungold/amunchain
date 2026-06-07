use serde::{Deserialize, Serialize};

/// Health status of the AmunChain node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeHealth {
    pub node_id: [u8; 32],
    pub is_synced: bool,
    pub current_height: u64,
    pub peer_count: usize,
    pub state_root: [u8; 32],
    pub last_block_time: u64,
    pub uptime_seconds: u64,
    pub memory_usage_mb: u64,
    pub storage_usage_mb: u64,
}

impl NodeHealth {
    pub fn new(node_id: [u8; 32]) -> Self {
        Self {
            node_id,
            is_synced: false,
            current_height: 0,
            peer_count: 0,
            state_root: [0u8; 32],
            last_block_time: 0,
            uptime_seconds: 0,
            memory_usage_mb: 0,
            storage_usage_mb: 0,
        }
    }

    /// Returns true if the node is healthy and ready to participate in consensus.
    pub fn is_healthy(&self) -> bool {
        self.is_synced && self.peer_count > 0
    }

    /// Returns a summary status string.
    pub fn status(&self) -> &str {
        if !self.is_synced { "syncing" }
        else if self.peer_count == 0 { "isolated" }
        else { "healthy" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n62_health_syncing_when_not_synced() {
        let health = NodeHealth::new([1u8; 32]);
        assert_eq!(health.status(), "syncing");
        assert!(!health.is_healthy());
    }

    #[test]
    fn n62_health_healthy_when_synced_with_peers() {
        let mut health = NodeHealth::new([1u8; 32]);
        health.is_synced = true;
        health.peer_count = 5;
        assert_eq!(health.status(), "healthy");
        assert!(health.is_healthy());
    }
}
