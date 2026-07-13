use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::types::{PeerId, PublicKey, ValidatorId};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Validator configuration that gets written to config.toml.
#[derive(Debug, Clone, Serialize)]
pub struct ValidatorConfig {
    pub name: String,
    pub peer_id: PeerId,
    pub validator_id: ValidatorId,
    pub public_key: PublicKey,
    pub listen_port: u16,
    pub rpc_port: u16,
    pub p2p_port: u16,
    pub metrics_port: u16,
    pub voting_power: u64,
    pub base_dir: PathBuf,
}

/// Write the validator config to disk.
pub fn write_config(
    config: &ValidatorConfig,
    base_dir: &Path,
) -> Result<PathBuf, OrchestratorError> {
    let validator_dir = base_dir.join("validators").join(&config.name);
    std::fs::create_dir_all(&validator_dir).map_err(|e| OrchestratorError::Io {
        path: validator_dir.clone(),
        source: e,
    })?;

    let config_path = validator_dir.join("config.toml");

    // Build TOML
    let toml_content = format!(
        r#"[node]
name = "{}"
listen_host = "0.0.0.0"
listen_port = {}

[identity]
key_file = "key.bin"
peer_id = "{}"
validator_id = "{}"
public_key = "{}"

[peers]
seed_peers = []

[genesis]
file = "../../genesis.json"

[rpc]
enabled = true
port = {}

[metrics]
enabled = true
port = {}

[voting]
power = {}
"#,
        config.name,
        config.listen_port,
        hex::encode(config.peer_id.0),
        hex::encode(config.validator_id.0),
        hex::encode(config.public_key.0),
        config.rpc_port,
        config.metrics_port,
        config.voting_power,
    );

    std::fs::write(&config_path, toml_content).map_err(|e| OrchestratorError::Io {
        path: config_path.clone(),
        source: e,
    })?;

    tracing::info!(path = %config_path.display(), "Validator config written");
    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_config() {
        let dir = TempDir::new().unwrap();
        let config = ValidatorConfig {
            name: "test-validator".into(),
            peer_id: PeerId([1u8; 32]),
            validator_id: ValidatorId([2u8; 32]),
            public_key: PublicKey([3u8; 32]),
            listen_port: 9071,
            rpc_port: 9546,
            p2p_port: 9071,
            metrics_port: 9601,
            voting_power: 100,
            base_dir: PathBuf::from(dir.path())
                .join("validators")
                .join("test-validator"),
        };
        let path = write_config(&config, &PathBuf::from(dir.path())).unwrap();
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("test-validator"));
        assert!(content.contains("9071"));
    }
}
