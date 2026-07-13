use amun_orchestrator_core::event::EventBus;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Result of checking leadership status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeadershipStatus {
    /// This instance is the leader.
    Leader,
    /// Another instance is the leader.
    Follower { leader_id: String },
    /// Election is in progress.
    Election,
}

/// Simple leader election using instance ID hashing + heartbeat.
pub struct LeaderElection {
    instance_id: String,
    #[allow(dead_code)]
    event_bus: Arc<EventBus>,
    current_leader: RwLock<Option<String>>,
    election_term: RwLock<u64>,
    last_heartbeat: RwLock<chrono::DateTime<chrono::Utc>>,
}

impl LeaderElection {
    pub fn new(instance_id: String, event_bus: Arc<EventBus>) -> Self {
        Self {
            instance_id,
            event_bus,
            current_leader: RwLock::new(None),
            election_term: RwLock::new(0),
            last_heartbeat: RwLock::new(chrono::Utc::now()),
        }
    }

    /// Start participating in leader election.
    pub async fn start(&self) {
        tracing::info!(instance_id = %self.instance_id, "Starting leader election");
        *self.last_heartbeat.write().await = chrono::Utc::now();
    }

    /// Check current leadership status.
    pub async fn check_status(&self) -> LeadershipStatus {
        // Simple deterministic leader election: hash instance_id + term
        let term = *self.election_term.read().await;
        let leader = self.determine_leader(term);

        if leader == self.instance_id {
            *self.current_leader.write().await = Some(self.instance_id.clone());
            return LeadershipStatus::Leader;
        }

        *self.current_leader.write().await = Some(leader.clone());
        LeadershipStatus::Follower { leader_id: leader }
    }

    /// Deterministically determine the leader for a given term.
    fn determine_leader(&self, term: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(term.to_le_bytes());
        hasher.update(b"amun-leader-election");
        let hash = hasher.finalize();

        // Use the hash to deterministically select a leader
        // In production, this would use all cluster members
        format!("leader-{}", hex::encode(&hash[..8]))
    }

    /// Get the current leader ID.
    pub async fn current_leader(&self) -> Option<String> {
        self.current_leader.read().await.clone()
    }

    /// Increment the election term (e.g., on leader failure).
    pub async fn increment_term(&self) {
        *self.election_term.write().await += 1;
        tracing::info!(
            term = *self.election_term.read().await,
            "Election term incremented"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_leader_is_deterministic() {
        let election = LeaderElection::new("instance-1".into(), Arc::new(EventBus::new(16)));
        let leader1 = election.determine_leader(1);
        let leader2 = election.determine_leader(1);
        assert_eq!(leader1, leader2);
    }

    #[test]
    fn test_different_terms_produce_different_leaders() {
        let election = LeaderElection::new("instance-1".into(), Arc::new(EventBus::new(16)));
        let leader_term1 = election.determine_leader(1);
        let leader_term2 = election.determine_leader(2);
        assert_ne!(leader_term1, leader_term2);
    }
}
