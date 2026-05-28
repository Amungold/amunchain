#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};
use crate::state::ConstitutionalState;

/// Deterministic state snapshot
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    pub height: u64,
    pub state_hash: [u8; 32],
    pub parent_snapshot_hash: [u8; 32],
    pub event_count: u64,
}

impl StateSnapshot {
    pub fn new(state: &ConstitutionalState, parent_hash: [u8; 32], event_count: u64) -> Self {
        Self {
            height: state.height,
            state_hash: state.hash(),
            parent_snapshot_hash: parent_hash,
            event_count,
        }
    }
    
    pub fn snapshot_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.height.to_be_bytes());
        hasher.update(self.state_hash);
        hasher.update(self.parent_snapshot_hash);
        hasher.update(self.event_count.to_be_bytes());
        hasher.finalize().into()
    }
    
    /// Verify snapshot chain integrity
    pub fn verify_chain(&self, previous: &StateSnapshot) -> bool {
        self.parent_snapshot_hash == previous.snapshot_hash()
    }
}

/// Snapshot manager for deterministic checkpointing
pub struct SnapshotManager {
    snapshots: Vec<StateSnapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }
    
    pub fn take_snapshot(&mut self, state: &ConstitutionalState, event_count: u64) -> StateSnapshot {
        let parent_hash = self.snapshots.last().map_or([0; 32], |s| s.snapshot_hash());
        let snapshot = StateSnapshot::new(state, parent_hash, event_count);
        self.snapshots.push(snapshot.clone());
        snapshot
    }
    
    pub fn get_snapshot(&self, height: u64) -> Option<&StateSnapshot> {
        self.snapshots.iter().find(|s| s.height == height)
    }
    
    pub fn latest_snapshot(&self) -> Option<&StateSnapshot> {
        self.snapshots.last()
    }
    
    pub fn verify_all(&self) -> bool {
        for i in 1..self.snapshots.len() {
            if !self.snapshots[i].verify_chain(&self.snapshots[i - 1]) {
                return false;
            }
        }
        true
    }
    
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    
    pub fn clear(&mut self) {
        self.snapshots.clear();
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConstitutionalState;
    
    #[test]
    fn test_snapshot_chain() {
        let mut manager = SnapshotManager::new();
        let state = ConstitutionalState::new();
        
        let snap1 = manager.take_snapshot(&state, 0);
        let snap2 = manager.take_snapshot(&state, 10);
        
        assert!(snap2.verify_chain(&snap1));
        assert!(manager.verify_all());
    }
}
