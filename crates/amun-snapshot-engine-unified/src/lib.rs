use amun_persistence::PersistentState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// A constitutional snapshot capturing chain state at a given height.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub height: u64,
    pub state_root: String,
    pub evidence_root: String,
    pub block_hash: String,
    pub snapshot_version: u32,
}

impl Snapshot {
    pub fn from_persistent_state(state: &PersistentState) -> Self {
        Self {
            height: state.height,
            state_root: state.state_root.clone(),
            evidence_root: state.evidence_root.clone(),
            block_hash: state.block_hash.clone(),
            snapshot_version: 1,
        }
    }
}

/// Manages snapshot creation and restoration.
pub struct SnapshotManager {
    pub path: String,
}

impl SnapshotManager {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Create a snapshot from a persistent state.
    pub fn create(&self, state: &PersistentState) -> Result<(), String> {
        let snapshot = Snapshot::from_persistent_state(state);
        let json = serde_json::to_string_pretty(&snapshot).map_err(|e| e.to_string())?;
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    /// Load a snapshot from disk.
    pub fn load(&self) -> Result<Snapshot, String> {
        if !Path::new(&self.path).exists() {
            return Err("Snapshot file not found".into());
        }
        let json = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        let snapshot: Snapshot = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        // Version check
        if snapshot.snapshot_version != 1 {
            return Err(format!(
                "Unsupported snapshot version: {}",
                snapshot.snapshot_version
            ));
        }
        Ok(snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_state() -> PersistentState {
        PersistentState {
            height: 100,
            state_root: "state100".into(),
            evidence_root: "evidence100".into(),
            block_hash: "block100".into(),
            last_commit_hash: "commit100".into(),
        }
    }

    #[test]
    fn n43_snapshot_roundtrip() {
        let path = "/tmp/n43_snapshot_test.json";
        let _ = fs::remove_file(path);

        let manager = SnapshotManager::new(path);
        let state = create_test_state();

        manager.create(&state).unwrap();
        let snapshot = manager.load().unwrap();

        assert_eq!(snapshot.height, 100);
        assert_eq!(snapshot.state_root, "state100");
        assert_eq!(snapshot.evidence_root, "evidence100");
        assert_eq!(snapshot.block_hash, "block100");
        assert_eq!(snapshot.snapshot_version, 1);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n43_snapshot_restores_height() {
        let path = "/tmp/n43_height_test.json";
        let _ = fs::remove_file(path);

        let manager = SnapshotManager::new(path);
        let state = PersistentState {
            height: 1000,
            state_root: "root1000".into(),
            evidence_root: "ev1000".into(),
            block_hash: "block1000".into(),
            last_commit_hash: "commit1000".into(),
        };

        manager.create(&state).unwrap();
        let snapshot = manager.load().unwrap();

        assert_eq!(snapshot.height, 1000);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n43_snapshot_restores_evidence() {
        let path = "/tmp/n43_evidence_test.json";
        let _ = fs::remove_file(path);

        let manager = SnapshotManager::new(path);
        let state = PersistentState {
            height: 42,
            state_root: "sr42".into(),
            evidence_root: "evidence_root_42".into(),
            block_hash: "bh42".into(),
            last_commit_hash: "ch42".into(),
        };

        manager.create(&state).unwrap();
        let snapshot = manager.load().unwrap();

        assert_eq!(snapshot.evidence_root, "evidence_root_42");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n43_version_validation() {
        let path = "/tmp/n43_version_test.json";
        let _ = fs::remove_file(path);

        // Manually write a snapshot with unsupported version
        let bad_snapshot = r#"{"height":1,"state_root":"x","evidence_root":"y","block_hash":"z","snapshot_version":99}"#;
        fs::write(path, bad_snapshot).unwrap();

        let manager = SnapshotManager::new(path);
        let result = manager.load();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported snapshot version"));

        let _ = fs::remove_file(path);
    }
}
