use super::{OrchestratorRole, PeerInstance};

/// Snapshot of the HA cluster status.
#[derive(Debug, Clone)]
pub struct ClusterStatus {
    pub instance_id: String,
    pub role: OrchestratorRole,
    pub peer_count: usize,
    pub healthy_peers: usize,
    pub peers: Vec<PeerInstance>,
}

impl ClusterStatus {
    pub fn has_quorum(&self) -> bool {
        let total = self.peer_count + 1;
        let healthy = self.healthy_peers + 1;
        healthy > total / 2
    }

    pub fn summary(&self) -> String {
        format!(
            "Cluster: {} role={:?} peers={}/{} healthy quorum={}",
            self.instance_id,
            self.role,
            self.healthy_peers,
            self.peer_count,
            self.has_quorum()
        )
    }
}
