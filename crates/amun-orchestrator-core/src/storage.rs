use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::error::OrchestratorError;
use crate::state::OrchestratorState;
use crate::OrchestratorConfig;

const CURRENT_STATE_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct PersistedState {
    version: u32,
    state: OrchestratorState,
}

pub struct StateStore {
    state_path: PathBuf,
    backup_path: PathBuf,
    config_path: PathBuf,
}

impl StateStore {
    pub fn new(base_dir: &PathBuf) -> Self {
        fs::create_dir_all(base_dir).ok();
        Self {
            state_path: base_dir.join("orchestrator_state.json"),
            backup_path: base_dir.join("orchestrator_state.json.bak"),
            config_path: base_dir.join("orchestrator_config.json"),
        }
    }

    /// Atomic write with parent directory fsync for crash safety.
    fn atomic_write(&self, path: &PathBuf, data: &[u8]) -> Result<(), OrchestratorError> {
        // Backup existing file — log failure but continue
        if path.exists() {
            if let Err(e) = fs::copy(path, &self.backup_path) {
                tracing::warn!(?path, ?e, "Failed to create backup before write");
            }
        }

        let tmp_path = path.with_extension("json.tmp");
        let parent = path.parent().unwrap();

        // Write to temp file
        let mut file = fs::File::create(&tmp_path).map_err(|e| OrchestratorError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
        file.write_all(data).map_err(|e| OrchestratorError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
        file.sync_all().map_err(|e| OrchestratorError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;

        // Atomic rename
        fs::rename(&tmp_path, path).map_err(|e| OrchestratorError::Io {
            path: path.clone(),
            source: e,
        })?;

        // fsync parent directory to ensure rename is durable
        let dir = fs::File::open(parent).map_err(|e| OrchestratorError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
        dir.sync_all().map_err(|e| OrchestratorError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;

        Ok(())
    }

    pub fn load_state(&self) -> Result<OrchestratorState, OrchestratorError> {
        let path = if self.state_path.exists() {
            &self.state_path
        } else if self.backup_path.exists() {
            tracing::warn!("Primary state missing, loading from backup");
            &self.backup_path
        } else {
            return Ok(OrchestratorState::new());
        };

        let data = fs::read(path).map_err(|e| OrchestratorError::Io {
            path: path.clone(),
            source: e,
        })?;
        let persisted: PersistedState = serde_json::from_slice(&data)
            .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;
        if persisted.version != CURRENT_STATE_VERSION {
            tracing::warn!(
                stored = persisted.version,
                current = CURRENT_STATE_VERSION,
                "State version mismatch"
            );
        }
        Ok(persisted.state)
    }

    pub fn save_state(&self, state: &OrchestratorState) -> Result<(), OrchestratorError> {
        let persisted = PersistedState {
            version: CURRENT_STATE_VERSION,
            state: state.clone(),
        };
        let data = serde_json::to_vec_pretty(&persisted)
            .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;
        self.atomic_write(&self.state_path, &data)
    }

    pub fn save_config_snapshot(
        &self,
        config: &OrchestratorConfig,
    ) -> Result<(), OrchestratorError> {
        let data = serde_json::to_vec_pretty(config)
            .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;
        self.atomic_write(&self.config_path, &data)
    }
}
