use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataPaths {
    pub base: PathBuf,
    pub genesis: PathBuf,
    pub validators: PathBuf,
    pub logs: PathBuf,
    pub backups: PathBuf,
    pub snapshots: PathBuf,
    pub certificates: PathBuf,
    pub configs: PathBuf,
}

impl DataPaths {
    pub fn new(base_dir: &Path) -> Self {
        Self {
            genesis: base_dir.join("genesis.json"),
            validators: base_dir.join("validators"),
            logs: base_dir.join("logs"),
            backups: base_dir.join("backups"),
            snapshots: base_dir.join("snapshots"),
            certificates: base_dir.join("certificates"),
            configs: base_dir.join("configs"),
            base: base_dir.to_path_buf(),
        }
    }

    pub fn ensure_all(&self) -> std::io::Result<()> {
        for dir in &[
            &self.validators,
            &self.logs,
            &self.backups,
            &self.snapshots,
            &self.certificates,
            &self.configs,
        ] {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorPaths {
    pub root: PathBuf,
    pub config: PathBuf,
    pub key: PathBuf,
    pub certificate: PathBuf,
    pub logs: PathBuf,
    pub snapshots: PathBuf,
    pub wal: PathBuf,
    pub data: PathBuf,
}

impl ValidatorPaths {
    pub fn new(validators_base: &Path, name: &str) -> Self {
        let root = validators_base.join(name);
        Self {
            config: root.join("config.toml"),
            key: root.join("key.bin"),
            certificate: root.join("validator.crt"),
            logs: root.join("logs"),
            snapshots: root.join("snapshots"),
            wal: root.join("wal"),
            data: root.join("data"),
            root,
        }
    }

    pub fn ensure_all(&self) -> std::io::Result<()> {
        for dir in &[&self.logs, &self.snapshots, &self.wal, &self.data] {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}
