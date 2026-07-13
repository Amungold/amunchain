use amun_orchestrator_core::error::OrchestratorError;

/// Handles state replication between leader and followers.
#[derive(Default)]
pub struct StateReplicator {
    replication_enabled: bool,
}

impl StateReplicator {
    pub fn new() -> Self {
        Self {
            replication_enabled: true,
        }
    }

    /// Leader: replicate state to all followers.
    pub async fn replicate_to_followers(&self) {
        if self.replication_enabled {
            // TODO: replicate state to followers.
        }
    }

    /// Follower: sync state from the leader.
    pub async fn sync_from_leader(&self, leader_id: &str) -> Result<(), OrchestratorError> {
        if !self.replication_enabled {
            return Ok(());
        }
        tracing::debug!(%leader_id, "Syncing state from leader");
        Ok(())
    }

    /// Enable or disable replication.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.replication_enabled = enabled;
    }
}
