use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::state::NodeConfig;
use crate::types::{PeerId, PublicKey, ValidatorId};

pub const CURRENT_CONFIG_VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OrchestratorConfig {
    pub config_version: u32,
    pub network: NetworkConfig,
    pub validators: Vec<ValidatorEntry>,
    pub services: ServicesConfig,
    pub health: HealthConfig,
    pub build: BuildConfig,
    pub deployment: DeploymentConfig,
    pub node: NodeConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NetworkConfig {
    pub name: String,
    pub chain_id: String,
    pub base_dir: PathBuf,
    pub genesis_file: PathBuf,
    pub bootnodes: Vec<String>,
    pub protocol_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorEntry {
    pub name: String,
    pub peer_id: PeerId,
    pub validator_id: ValidatorId,
    pub public_key: PublicKey,
    pub listen_port: u16,
    pub rpc_port: u16,
    pub p2p_port: u16,
    pub metrics_port: u16,
    pub voting_power: u64,
    pub service_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ServicesConfig {
    pub rpc_enabled: bool,
    pub rpc_port: u16,
    pub explorer_api_enabled: bool,
    pub explorer_api_port: u16,
    pub explorer_ui_enabled: bool,
    pub explorer_ui_port: u16,
    pub websocket_enabled: bool,
    pub websocket_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HealthConfig {
    pub check_interval_secs: u64,
    pub max_restart_attempts: u32,
    pub restart_cooldown_secs: u64,
    pub auto_rejoin: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BuildConfig {
    pub auto_build: bool,
    pub incremental: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Testnet,
    Mainnet,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeploymentConfig {
    pub environment: Environment,
    pub auto_genesis: bool,
    pub auto_certs: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub prometheus_port: u16,
    pub grafana_enabled: bool,
    pub telemetry_enabled: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prometheus_port: 9090,
            grafana_enabled: true,
            telemetry_enabled: true,
        }
    }
}

impl OrchestratorConfig {
    pub fn load(path: &str) -> Result<Self, crate::OrchestratorError> {
        let content = std::fs::read_to_string(path).map_err(|e| crate::OrchestratorError::Io {
            path: PathBuf::from(path),
            source: e,
        })?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| crate::OrchestratorError::Config(e.to_string()))?;
        if config.config_version != CURRENT_CONFIG_VERSION {
            tracing::warn!(
                file_version = config.config_version,
                current_version = CURRENT_CONFIG_VERSION,
                "Config version mismatch"
            );
        }
        Ok(config)
    }
}
