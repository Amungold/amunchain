// ============================================================================
// ADR-022: Integration test - amun-node starts with unified AmunConfig
// ============================================================================

use amun_bootstrap::AmunConfig;
use std::path::PathBuf;

#[test]
fn test_node_config_roundtrip() {
    // Create a unified config (simulating what main.rs would receive)
    let config = AmunConfig {
        validator_id: [1u8; 32],
        key_file: PathBuf::from("/tmp/amun-test/key.bin"),
        certificate_path: None,
        listen_addr: "127.0.0.1:9070".parse().unwrap(),
        peer_addresses: vec![
            "127.0.0.1:9071".parse().unwrap(),
            "127.0.0.1:9072".parse().unwrap(),
        ],
        data_dir: PathBuf::from("/tmp/amun-test"),
        genesis_file: PathBuf::from("../genesis/test_genesis.json"),
        quorum_size: 3,
    };
    
    // Validate
    assert!(config.validate().is_ok());
    assert_eq!(config.total_validators(), 3);
    
    // Serialize to TOML (simulating config file)
    let toml_str = toml::to_string_pretty(&config).expect("serialize");
    
    // Parse back (simulating main.rs loading)
    let loaded: AmunConfig = toml::from_str(&toml_str).expect("deserialize");
    assert_eq!(loaded.validator_id, config.validator_id);
    assert_eq!(loaded.listen_addr, config.listen_addr);
    assert_eq!(loaded.quorum_size, config.quorum_size);
}

#[test]
fn test_bootstrap_error_on_invalid_config() {
    let config = AmunConfig {
        quorum_size: 0,  // Invalid
        ..make_test_config()
    };
    assert!(config.validate().is_err());
}

fn make_test_config() -> AmunConfig {
    AmunConfig {
        validator_id: [1u8; 32],
        key_file: PathBuf::from("/tmp/key.bin"),
        certificate_path: None,
        listen_addr: "127.0.0.1:9000".parse().unwrap(),
        peer_addresses: vec!["127.0.0.1:9001".parse().unwrap()],
        data_dir: PathBuf::from("/tmp/amun-test"),
        genesis_file: PathBuf::from("genesis.bin"),
        quorum_size: 3,
    }
}
