use hex;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct SourceFingerprinter {
    workspace_root: PathBuf,
    fingerprint_file: PathBuf,
    fingerprints: HashMap<String, String>,
}

impl SourceFingerprinter {
    pub fn new(workspace_root: &Path, workspace_members: &HashSet<String>) -> Self {
        let fp_file = workspace_root
            .join("target")
            .join(".amun_fingerprints.json");
        let previous = Self::load_previous(&fp_file);
        // Seed in-memory fingerprints from previous build
        let mut fps = previous;
        // Ensure all current members have an entry
        for m in workspace_members {
            fps.entry(m.clone()).or_default();
        }
        Self {
            workspace_root: workspace_root.to_path_buf(),
            fingerprint_file: fp_file,
            fingerprints: fps,
        }
    }

    fn load_previous(path: &Path) -> HashMap<String, String> {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn hash_dir(&self, dir: &Path) -> Result<String, amun_orchestrator_core::OrchestratorError> {
        let mut hasher = Sha256::new();
        if dir.exists() {
            for entry in WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext == "rs" || ext == "toml")
                })
            {
                let content = std::fs::read(entry.path()).map_err(|e| {
                    amun_orchestrator_core::OrchestratorError::Io {
                        path: entry.path().to_path_buf(),
                        source: e,
                    }
                })?;
                hasher.update(&content);
                hasher.update(entry.path().to_string_lossy().as_bytes());
            }
        }
        Ok(hex::encode(hasher.finalize()))
    }

    /// Detect changed crates compared to stored fingerprints.
    pub async fn find_changed(
        &self,
    ) -> Result<HashSet<String>, amun_orchestrator_core::OrchestratorError> {
        let mut changed = HashSet::new();
        let crates_dir = self.workspace_root.join("crates");

        if crates_dir.exists() {
            for entry in std::fs::read_dir(&crates_dir).map_err(|e| {
                amun_orchestrator_core::OrchestratorError::Io {
                    path: crates_dir.clone(),
                    source: e,
                }
            })? {
                let entry = entry.map_err(|e| amun_orchestrator_core::OrchestratorError::Io {
                    path: crates_dir.clone(),
                    source: e,
                })?;
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let hash = self.hash_dir(&entry.path())?;
                    match self.fingerprints.get(&name) {
                        Some(prev) if prev == &hash => {}
                        _ => {
                            changed.insert(name);
                        }
                    }
                }
            }
        }

        // Also check root-level members (fuzz/, tests/, etc.)
        for extra_dir in &["fuzz", "tests", "apps"] {
            let dir = self.workspace_root.join(extra_dir);
            if dir.exists() {
                for entry in std::fs::read_dir(&dir).map_err(|e| {
                    amun_orchestrator_core::OrchestratorError::Io {
                        path: dir.clone(),
                        source: e,
                    }
                })? {
                    let entry =
                        entry.map_err(|e| amun_orchestrator_core::OrchestratorError::Io {
                            path: dir.clone(),
                            source: e,
                        })?;
                    if entry.file_type().is_ok_and(|t| t.is_dir()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let hash = self.hash_dir(&entry.path())?;
                        match self.fingerprints.get(&name) {
                            Some(prev) if prev == &hash => {}
                            _ => {
                                changed.insert(name);
                            }
                        }
                    }
                }
            }
        }

        Ok(changed)
    }

    /// Update fingerprints in memory and on disk after a successful build.
    pub async fn persist(&mut self) -> Result<(), amun_orchestrator_core::OrchestratorError> {
        let crates_dir = self.workspace_root.join("crates");
        if crates_dir.exists() {
            for entry in std::fs::read_dir(&crates_dir).map_err(|e| {
                amun_orchestrator_core::OrchestratorError::Io {
                    path: crates_dir.clone(),
                    source: e,
                }
            })? {
                let entry = entry.map_err(|e| amun_orchestrator_core::OrchestratorError::Io {
                    path: crates_dir.clone(),
                    source: e,
                })?;
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if self.fingerprints.contains_key(&name) {
                        let hash = self.hash_dir(&entry.path())?;
                        self.fingerprints.insert(name, hash);
                    }
                }
            }
        }

        let parent = self.fingerprint_file.parent().unwrap();
        std::fs::create_dir_all(parent).map_err(|e| {
            amun_orchestrator_core::OrchestratorError::Io {
                path: parent.to_path_buf(),
                source: e,
            }
        })?;

        let json = serde_json::to_string_pretty(&self.fingerprints)
            .map_err(|e| amun_orchestrator_core::OrchestratorError::Serialization(e.to_string()))?;
        std::fs::write(&self.fingerprint_file, json).map_err(|e| {
            amun_orchestrator_core::OrchestratorError::Io {
                path: self.fingerprint_file.clone(),
                source: e,
            }
        })?;

        Ok(())
    }
}
