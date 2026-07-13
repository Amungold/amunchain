#![allow(dead_code)]
pub mod config_gen;
pub mod keygen;
pub mod service_manager;
pub mod validator;

use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::event::EventBus;
use amun_orchestrator_core::traits::{CertificateProvider, GenesisProvider, ProcessManager};
use amun_orchestrator_core::types::{PeerId, PublicKey, ValidatorId};
use std::path::PathBuf;
use std::sync::Arc;
use validator::ValidatorConfig;

/// Result of creating a new validator.
#[derive(Debug, Clone)]
pub struct ValidatorResult {
    pub name: String,
    pub peer_id: PeerId,
    pub validator_id: ValidatorId,
    pub public_key: PublicKey,
    pub listen_port: u16,
    pub rpc_port: u16,
    pub config_path: PathBuf,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

/// The Validator Factory handles the full lifecycle:
/// create, start, stop, restart, remove.
pub struct ValidatorFactory {
    event_bus: Arc<EventBus>,
    cert_provider: Arc<dyn CertificateProvider>,
    genesis_provider: Arc<dyn GenesisProvider>,
    process_manager: Arc<dyn ProcessManager>,
    base_dir: PathBuf,
}

impl ValidatorFactory {
    pub fn new(
        event_bus: Arc<EventBus>,
        cert_provider: Arc<dyn CertificateProvider>,
        genesis_provider: Arc<dyn GenesisProvider>,
        process_manager: Arc<dyn ProcessManager>,
        base_dir: PathBuf,
    ) -> Self {
        Self {
            event_bus,
            cert_provider,
            genesis_provider,
            process_manager,
            base_dir,
        }
    }

    /// Create a new validator from scratch:
    /// 1. Generate keypair
    /// 2. Generate certificate
    /// 3. Generate config.toml
    /// 4. Create systemd service (or process definition)
    pub async fn create_validator(
        &self,
        name: &str,
        listen_port: u16,
        rpc_port: u16,
        voting_power: u64,
    ) -> Result<ValidatorResult, OrchestratorError> {
        tracing::info!(%name, listen_port, rpc_port, voting_power, "Creating validator");

        // 1. Generate keypair
        let keypair = keygen::generate_keypair();
        let peer_id = keygen::derive_peer_id(&keypair);
        let validator_id = keygen::derive_validator_id(&keypair);
        let public_key = keygen::derive_public_key(&keypair);

        // 2. Generate certificate
        let cert_path = self
            .cert_provider
            .generate_validator_certificate(name, &public_key)
            .await?;

        // 3. Generate config
        let config = ValidatorConfig {
            name: name.to_string(),
            peer_id,
            validator_id,
            public_key,
            listen_port,
            rpc_port,
            p2p_port: listen_port,
            metrics_port: rpc_port + 100,
            voting_power,
            base_dir: self.base_dir.join("validators").join(name),
        };

        let config_path = config_gen::write_config(&config, &self.base_dir)?;

        // Rebuild genesis to include all validators
        self.rebuild_genesis().await?;

        let result = ValidatorResult {
            name: name.to_string(),
            peer_id,
            validator_id,
            public_key,
            listen_port,
            rpc_port,
            config_path,
            cert_path,
            key_path: config.base_dir.join("key.bin"),
        };

        Ok(result)
    }

    /// Start a validator by name.
    pub async fn start_validator(&self, name: &str) -> Result<u32, OrchestratorError> {
        let service_name = format!("amun-{}", name);
        let config_path = self
            .base_dir
            .join("validators")
            .join(name)
            .join("config.toml");

        let pid = service_manager::start_service(
            &self.process_manager,
            &service_name,
            config_path.to_str().unwrap(),
        )
        .await?;

        self.event_bus.emit(
            "validator-factory",
            amun_orchestrator_core::event::OrchestratorEvent::ValidatorStarted {
                name: name.to_string(),
                pid,
                listen_port: 0, // populated by health check later
                rpc_port: 0,
            },
        );

        Ok(pid)
    }

    /// Stop a validator by name.
    pub async fn stop_validator(&self, name: &str, reason: &str) -> Result<(), OrchestratorError> {
        let service_name = format!("amun-{}", name);
        self.process_manager.stop(&service_name).await?;

        self.event_bus.emit(
            "validator-factory",
            amun_orchestrator_core::event::OrchestratorEvent::ValidatorStopped {
                name: name.to_string(),
                reason: reason.to_string(),
                exit_code: None,
            },
        );

        Ok(())
    }

    /// Restart a validator.
    pub async fn restart_validator(&self, name: &str) -> Result<u32, OrchestratorError> {
        let service_name = format!("amun-{}", name);
        let pid = self.process_manager.restart(&service_name).await?;

        self.event_bus.emit(
            "validator-factory",
            amun_orchestrator_core::event::OrchestratorEvent::ValidatorRecovered {
                name: name.to_string(),
            },
        );

        Ok(pid)
    }

    /// Remove a validator completely.
    pub async fn remove_validator(&self, name: &str) -> Result<(), OrchestratorError> {
        let service_name = format!("amun-{}", name);

        // Stop first
        let _ = self.process_manager.stop(&service_name).await;

        // Remove data directory
        let validator_dir = self.base_dir.join("validators").join(name);
        if validator_dir.exists() {
            tokio::fs::remove_dir_all(&validator_dir)
                .await
                .map_err(|e| OrchestratorError::Io {
                    path: validator_dir,
                    source: e,
                })?;
        }

        self.event_bus.emit(
            "validator-factory",
            amun_orchestrator_core::event::OrchestratorEvent::ValidatorRemoved {
                name: name.to_string(),
            },
        );

        Ok(())
    }

    /// Check if a validator is running.
    pub async fn is_running(&self, name: &str) -> Result<bool, OrchestratorError> {
        let service_name = format!("amun-{}", name);
        self.process_manager.is_running(&service_name).await
    }
}

/// Collect all validators and rebuild genesis.json.
impl ValidatorFactory {
    pub async fn rebuild_genesis(&self) -> Result<(), OrchestratorError> {
        let validators_dir = self.base_dir.join("validators");
        if !validators_dir.exists() {
            return Ok(());
        }

        let mut entries: Vec<(ValidatorId, PublicKey, u64)> = Vec::new();

        let mut dir =
            tokio::fs::read_dir(&validators_dir)
                .await
                .map_err(|e| OrchestratorError::Io {
                    path: validators_dir.clone(),
                    source: e,
                })?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| OrchestratorError::Io {
            path: validators_dir.clone(),
            source: e,
        })? {
            if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
                let config_path = entry.path().join("config.toml");
                if let Ok(content) = tokio::fs::read_to_string(&config_path).await {
                    if let (Some(vid), Some(pk)) = (
                        extract_toml_value(&content, "validator_id"),
                        extract_toml_value(&content, "public_key"),
                    ) {
                        if let (Ok(vid_bytes), Ok(pk_bytes)) = (hex::decode(&vid), hex::decode(&pk))
                        {
                            if vid_bytes.len() == 32 && pk_bytes.len() == 32 {
                                let mut vid_arr = [0u8; 32];
                                let mut pk_arr = [0u8; 32];
                                vid_arr.copy_from_slice(&vid_bytes);
                                pk_arr.copy_from_slice(&pk_bytes);
                                entries.push((ValidatorId(vid_arr), PublicKey(pk_arr), 100));
                            }
                        }
                    }
                }
            }
        }

        if entries.is_empty() {
            return Ok(());
        }

        self.genesis_provider
            .generate_genesis("amun-chain", &entries)
            .await?;

        // Copy genesis.json to each validator directory
        let genesis_src = self.base_dir.join("genesis.json");
        let mut dir =
            tokio::fs::read_dir(&validators_dir)
                .await
                .map_err(|e| OrchestratorError::Io {
                    path: validators_dir.clone(),
                    source: e,
                })?;
        while let Some(entry) = dir.next_entry().await.map_err(|e| OrchestratorError::Io {
            path: validators_dir.clone(),
            source: e,
        })? {
            if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
                let dest = entry.path().join("genesis.json");
                let _ = tokio::fs::copy(&genesis_src, &dest).await;
            }
        }

        Ok(())
    }
}

/// Extract a value from a TOML config line.
fn extract_toml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(pos) = trimmed.find(&format!("{} = ", key)) {
            return Some(
                trimmed[pos + key.len() + 3..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        if let Some(pos) = trimmed.find(&format!("{} =", key)) {
            return Some(
                trimmed[pos + key.len() + 2..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    None
}
