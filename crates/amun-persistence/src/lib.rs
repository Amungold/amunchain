use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Constitutional state that persists across node restarts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistentState {
    pub height: u64,
    pub state_root: String,
    pub evidence_root: String,
    pub block_hash: String,
    pub last_commit_hash: String,
}

impl PersistentState {
    pub fn genesis() -> Self {
        Self {
            height: 0,
            state_root: String::new(),
            evidence_root: String::new(),
            block_hash: String::new(),
            last_commit_hash: String::new(),
        }
    }
}

/// Backend trait for persistence implementations.
pub trait PersistenceBackend {
    fn save(&self, state: &PersistentState) -> Result<(), String>;
    fn load(&self) -> Result<PersistentState, String>;
}

/// File-based persistence backend.
pub struct FileBackend {
    pub path: String,
}

impl FileBackend {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl PersistenceBackend for FileBackend {
    fn save(&self, state: &PersistentState) -> Result<(), String> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| e.to_string())?;
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    fn load(&self) -> Result<PersistentState, String> {
        if !Path::new(&self.path).exists() {
            return Ok(PersistentState::genesis());
        }
        let json = fs::read_to_string(&self.path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn n42_persistence_roundtrip() {
        let path = "/tmp/n42_test_state.json";
        let _ = fs::remove_file(path);
        
        let backend = FileBackend::new(path);
        
        let original = PersistentState {
            height: 41,
            state_root: "abc123".into(),
            evidence_root: "def456".into(),
            block_hash: "ghi789".into(),
            last_commit_hash: "jkl012".into(),
        };
        
        backend.save(&original).unwrap();
        let restored = backend.load().unwrap();
        
        assert_eq!(restored.height, 41);
        assert_eq!(restored.state_root, "abc123");
        assert_eq!(restored.evidence_root, "def456");
        assert_eq!(restored.block_hash, "ghi789");
        assert_eq!(restored.last_commit_hash, "jkl012");
        
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n42_genesis_on_missing_file() {
        let backend = FileBackend::new("/tmp/n42_nonexistent.json");
        let _ = fs::remove_file("/tmp/n42_nonexistent.json");
        
        let state = backend.load().unwrap();
        assert_eq!(state.height, 0);
        assert!(state.state_root.is_empty());
        assert!(state.evidence_root.is_empty());
    }

    #[test]
    fn n42_persistence_survives_reload() {
        let path = "/tmp/n42_reload_test.json";
        let _ = fs::remove_file(path);
        
        let backend = FileBackend::new(path);
        
        let state = PersistentState {
            height: 100,
            state_root: "state100".into(),
            evidence_root: "evidence100".into(),
            block_hash: "block100".into(),
            last_commit_hash: "commit100".into(),
        };
        
        backend.save(&state).unwrap();
        let loaded = backend.load().unwrap();
        assert_eq!(loaded.height, 100);
        
        // Reload without saving again
        let reloaded = backend.load().unwrap();
        assert_eq!(reloaded.height, 100);
        
        let _ = fs::remove_file(path);
    }
}
