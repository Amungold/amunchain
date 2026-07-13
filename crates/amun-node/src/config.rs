use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::{io_err, NodeError};

/// Root configuration structure representing the complete config.toml file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub node: NodeSection,
    pub peers: PeersSection,
    pub identity: IdentitySection,
    pub genesis: GenesisSection,

    #[serde(default)]
    pub validator: Option<ValidatorSection>,

    #[serde(default)]
    pub cluster: Vec<ClusterPeerConfig>,
}

/// Node-specific configuration section.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeSection {
    pub name: String,
    pub listen_host: String,
    pub listen_port: u16,
}

/// Peer discovery and networking configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PeersSection {
    pub seed_peers: Vec<String>,
}

/// Identity and key management configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdentitySection {
    pub key_file: String,
}

/// Genesis block configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenesisSection {
    pub file: String,
}

/// Validator runtime configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ValidatorSection {
    pub validator_id: [u8; 32],
    pub authority_public_key: [u8; 32],
    pub data_dir: String,
}

/// Cluster peer configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClusterPeerConfig {
    pub validator_id: [u8; 32],
    pub address: String,
    pub certificate_path: String,
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn load(path: &str) -> Result<Self, NodeError> {
        let config_str = fs::read_to_string(path).map_err(|e| io_err(path, e))?;
        toml::from_str(&config_str).map_err(|e| NodeError::Toml {
            path: Path::new(path).to_path_buf(),
            source: e,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_valid_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let config_content = r#"
[node]
name = "test-node"
listen_host = "0.0.0.0"
listen_port = 9070

[peers]
seed_peers = ["seed1.example.com:9070", "seed2.example.com:9070"]

[identity]
key_file = "node_key.json"

[genesis]
file = "genesis.json"
"#;
        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        let config = Config::load(config_path.to_str().unwrap()).unwrap();

        assert_eq!(config.node.name, "test-node");
        assert_eq!(config.node.listen_host, "0.0.0.0");
        assert_eq!(config.node.listen_port, 9070);
        assert_eq!(config.peers.seed_peers.len(), 2);
        assert_eq!(config.identity.key_file, "node_key.json");
        assert_eq!(config.genesis.file, "genesis.json");
    }

    #[test]
    fn test_load_missing_file() {
        let err = Config::load("/nonexistent/path/config.toml").unwrap_err();
        assert!(matches!(err, NodeError::Io { .. }));
    }

    #[test]
    fn test_load_invalid_toml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("invalid.toml");
        std::fs::write(&config_path, "this is not valid toml {{{").unwrap();
        let err = Config::load(config_path.to_str().unwrap()).unwrap_err();
        assert!(matches!(err, NodeError::Toml { .. }));
    }

    #[test]
    fn config_roundtrip() {
        let original = Config {
            node: NodeSection {
                name: "roundtrip-node".to_string(),
                listen_host: "127.0.0.1".to_string(),
                listen_port: 8080,
            },
            peers: PeersSection {
                seed_peers: vec!["peer1:9070".to_string()],
            },
            identity: IdentitySection {
                key_file: "test_key.json".to_string(),
            },
            genesis: GenesisSection {
                file: "test_genesis.json".to_string(),
            },
            validator: None,
            cluster: Vec::new(),
        };

        let serialized = toml::to_string_pretty(&original).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(original.node.name, deserialized.node.name);
        assert_eq!(original.node.listen_host, deserialized.node.listen_host);
        assert_eq!(original.node.listen_port, deserialized.node.listen_port);
        assert_eq!(original.peers.seed_peers, deserialized.peers.seed_peers);
        assert_eq!(original.identity.key_file, deserialized.identity.key_file);
        assert_eq!(original.genesis.file, deserialized.genesis.file);
    }
}
