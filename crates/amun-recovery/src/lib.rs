use amun_persistence::{PersistentState, FileBackend, PersistenceBackend};
use amun_snapshot_engine_unified::{SnapshotManager};
use amun_block_store::{BlockStore, StoredBlock};
use amun_replay_store::{ReplayStore, ReplayRecord};

/// The complete recovered state of a node after restart.
#[derive(Debug, Clone, PartialEq)]
pub struct RecoveredState {
    pub height: u64,
    pub state_root: String,
    pub evidence_root: String,
    pub block_hash: String,
    pub last_commit_hash: String,
    pub block_count: usize,
    pub replay_count: usize,
}

/// Orchestrates full node recovery from all persistence layers.
pub struct RecoveryEngine {
    persistence: FileBackend,
    snapshots: SnapshotManager,
    blocks: BlockStore,
    replay: ReplayStore,
}

impl RecoveryEngine {
    pub fn new(data_dir: &str) -> Self {
        Self {
            persistence: FileBackend::new(&format!("{}/state.json", data_dir)),
            snapshots: SnapshotManager::new(&format!("{}/snapshot.json", data_dir)),
            blocks: BlockStore::new(&format!("{}/blocks.json", data_dir)),
            replay: ReplayStore::new(&format!("{}/replay.json", data_dir)),
        }
    }

    /// Save current state to all persistence layers.
    pub fn save_state(
        &self,
        state: &PersistentState,
        block: &StoredBlock,
        record: &ReplayRecord,
    ) -> Result<(), String> {
        self.persistence.save(state)?;
        self.snapshots.create(state)?;
        self.blocks.append(block)?;
        self.replay.append(record)?;
        Ok(())
    }

    /// Recover the full node state from all persistence layers.
    pub fn recover(&self) -> Result<RecoveredState, String> {
        let state = self.persistence.load()?;
        let snapshot = self.snapshots.load().ok();
        let blocks = self.blocks.load_all().unwrap_or_default();
        let replay_records = self.replay.load_all().unwrap_or_default();

        // Verify snapshot matches persisted state if available
        if let Some(ref snap) = snapshot {
            if snap.height != state.height {
                return Err(format!(
                    "Snapshot height mismatch: snapshot={}, state={}",
                    snap.height, state.height
                ));
            }
        }

        // Verify replay chain integrity
        if !self.replay.verify_chain().unwrap_or(false) {
            return Err("Replay chain verification failed".into());
        }

        Ok(RecoveredState {
            height: state.height,
            state_root: state.state_root.clone(),
            evidence_root: state.evidence_root.clone(),
            block_hash: state.block_hash.clone(),
            last_commit_hash: state.last_commit_hash.clone(),
            block_count: blocks.len(),
            replay_count: replay_records.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn clean_dir(dir: &str) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn n46_save_and_recover() {
        let dir = "/tmp/n46_recovery_test";
        clean_dir(dir);
        
        let engine = RecoveryEngine::new(dir);
        
        let state = PersistentState {
            height: 42,
            state_root: "state42".into(),
            evidence_root: "evidence42".into(),
            block_hash: "block42".into(),
            last_commit_hash: "commit42".into(),
        };
        
        let block = StoredBlock {
            height: 42,
            hash: "block42".into(),
            parent_hash: "block41".into(),
            state_root: "state42".into(),
            evidence_root: "evidence42".into(),
        };
        
        let record = ReplayRecord {
            height: 42,
            tx_hash: "tx42".into(),
            state_root_before: "state41".into(),
            state_root_after: "state42".into(),
            commit_hash: "commit42".into(),
        };
        
        engine.save_state(&state, &block, &record).unwrap();
        
        let recovered = engine.recover().unwrap();
        assert_eq!(recovered.height, 42);
        assert_eq!(recovered.state_root, "state42");
        assert_eq!(recovered.evidence_root, "evidence42");
        assert_eq!(recovered.block_count, 1);
        assert_eq!(recovered.replay_count, 1);
        
        clean_dir(dir);
    }

    #[test]
    fn n46_recover_from_genesis() {
        let dir = "/tmp/n46_genesis_test";
        clean_dir(dir);
        
        let engine = RecoveryEngine::new(dir);
        let recovered = engine.recover().unwrap();
        assert_eq!(recovered.height, 0);
        assert_eq!(recovered.block_count, 0);
        assert_eq!(recovered.replay_count, 0);
        
        clean_dir(dir);
    }

    #[test]
    fn n46_full_persistence_pipeline() {
        let dir = "/tmp/n46_pipeline_test";
        clean_dir(dir);
        
        let engine = RecoveryEngine::new(dir);
        
        // Simulate multiple blocks
        for i in 1..=3 {
            let state = PersistentState {
                height: i,
                state_root: format!("state{}", i),
                evidence_root: format!("evidence{}", i),
                block_hash: format!("block{}", i),
                last_commit_hash: format!("commit{}", i),
            };
            let block = StoredBlock {
                height: i,
                hash: format!("block{}", i),
                parent_hash: format!("block{}", i.saturating_sub(1)),
                state_root: format!("state{}", i),
                evidence_root: format!("evidence{}", i),
            };
            let record = ReplayRecord {
                height: i,
                tx_hash: format!("tx{}", i),
                state_root_before: format!("state{}", i.saturating_sub(1)),
                state_root_after: format!("state{}", i),
                commit_hash: format!("commit{}", i),
            };
            engine.save_state(&state, &block, &record).unwrap();
        }
        
        let recovered = engine.recover().unwrap();
        assert_eq!(recovered.height, 3);
        assert_eq!(recovered.block_count, 3);
        assert_eq!(recovered.replay_count, 3);
        assert_eq!(recovered.state_root, "state3");
        
        clean_dir(dir);
    }
}
