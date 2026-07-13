use amun_orchestrator_core::config::OrchestratorConfig;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_load_valid_config() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("config.toml");
    let toml = r#"
config_version = 1
[network]
name = "test"
chain_id = "test-1"
base_dir = "./data"
genesis_file = "./data/genesis.json"
bootnodes = []
protocol_version = 1
[services]
rpc_enabled = true
rpc_port = 8545
explorer_api_enabled = false
explorer_api_port = 3000
explorer_ui_enabled = false
explorer_ui_port = 3001
websocket_enabled = false
websocket_port = 8546
[health]
check_interval_secs = 10
max_restart_attempts = 3
restart_cooldown_secs = 30
auto_rejoin = true
[build]
auto_build = true
incremental = true
[deployment]
environment = "dev"
auto_genesis = true
auto_certs = true
[node]
auto_restart = true
auto_snapshot = true
auto_backup = true
auto_prune = true
snapshot_interval_blocks = 1000
backup_interval_secs = 3600
prune_keep_blocks = 100000
[metrics]
enabled = true
prometheus_port = 9090
grafana_enabled = true
telemetry_enabled = true
[[validators]]
name = "validator-1"
peer_id = "0000000000000000000000000000000000000000000000000000000000000001"
validator_id = "0000000000000000000000000000000000000000000000000000000000000001"
public_key = "0000000000000000000000000000000000000000000000000000000000000065"
listen_port = 9071
rpc_port = 9546
p2p_port = 9071
metrics_port = 9601
voting_power = 100
service_name = "amun-validator-1"
"#;
    std::fs::File::create(&p)
        .unwrap()
        .write_all(toml.as_bytes())
        .unwrap();
    let c = OrchestratorConfig::load(p.to_str().unwrap()).unwrap();
    assert_eq!(c.network.name, "test");
    assert_eq!(c.validators.len(), 1);
    assert_eq!(c.validators[0].name, "validator-1");
    assert_eq!(c.metrics.prometheus_port, 9090);
}

#[test]
fn test_load_missing_file_fails() {
    assert!(OrchestratorConfig::load("/nonexistent/config.toml").is_err());
}

#[test]
fn test_invalid_hex_length_rejected() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("bad.toml");
    let toml = r#"
config_version = 1
[network]
name = "t"
chain_id = "t"
base_dir = "."
genesis_file = "g.json"
bootnodes = []
protocol_version = 1
[services]
rpc_enabled = false
rpc_port = 0
explorer_api_enabled = false
explorer_api_port = 0
explorer_ui_enabled = false
explorer_ui_port = 0
websocket_enabled = false
websocket_port = 0
[health]
check_interval_secs = 10
max_restart_attempts = 3
restart_cooldown_secs = 30
auto_rejoin = true
[build]
auto_build = true
incremental = true
[deployment]
environment = "dev"
auto_genesis = true
auto_certs = true
[node]
auto_restart = true
auto_snapshot = true
auto_backup = true
auto_prune = true
snapshot_interval_blocks = 1000
backup_interval_secs = 3600
prune_keep_blocks = 100000
[metrics]
enabled = true
prometheus_port = 9090
grafana_enabled = true
telemetry_enabled = true
[[validators]]
name = "v"
peer_id = "too-short"
validator_id = "0000000000000000000000000000000000000000000000000000000000000001"
public_key = "0000000000000000000000000000000000000000000000000000000000000001"
listen_port = 1
rpc_port = 1
p2p_port = 1
metrics_port = 1
voting_power = 1
service_name = "s"
"#;
    std::fs::File::create(&p)
        .unwrap()
        .write_all(toml.as_bytes())
        .unwrap();
    assert!(OrchestratorConfig::load(p.to_str().unwrap()).is_err());
}
