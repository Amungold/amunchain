mod peer_handshake;
mod peer_registry;

use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_rpc::{serve, AppState};
use std::sync::{Arc, Mutex};

fn main() {
    println!("AmunChain Node v0.2 (Unified RPC)");

    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/node_unified.toml".to_string());
    let amun_config =
        amun_bootstrap::AmunConfig::from_file(&config_path).expect("Failed to load config");

    let vconfig = ValidatorConfig {
        validator_id: amun_config.validator_id,
        listen_addr: amun_config.listen_addr,
        cluster: amun_config
            .peer_addresses
            .iter()
            .copied()
            .map(|addr| amun_live_cluster::config::ClusterPeer {
                validator_id: [0u8; 32],
                certificate_path: None,
                address: addr,
            })
            .collect(),
        data_dir: amun_config.data_dir.to_str().unwrap().to_string(),
        quorum_size: Some(amun_config.quorum_size),
        authority_public_key: [0u8; 32],
    };

    let validator = Arc::new(LiveValidator::new(vconfig));
    validator.start().expect("Failed to start validator");
    println!("Validator started. Height: {}", validator.current_height());

    let gossip_peers: Vec<std::net::SocketAddr> = amun_config.peer_addresses.to_vec();

    let state = AppState {
        store: validator.store.clone(),
        block_store: validator.block_store.clone(),
        engine: validator.engine.clone(),
        mempool: validator.mempool.clone(),
        faucet: Arc::new(Mutex::new(amun_rpc::faucet::FaucetState::default())),
        account_store: Arc::new(Mutex::new(amun_accounts::AccountStore::new())),
        peers: gossip_peers,
    };

    let rpc_port = amun_config.listen_addr.port() + 70;
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    println!("Unified RPC on 0.0.0.0:{}", rpc_port);
    rt.block_on(serve(state, rpc_port));
}
