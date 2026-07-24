use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_rpc::{serve, AppState};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let index: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let base_port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(9900);
    let rpc_port: u16 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9070);
    let quorum: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(4);

    let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
    let config = ValidatorConfig::test_cluster(index, &ports).with_quorum(quorum);

    eprintln!(
        "Validator {}: static peers = {:?}",
        index,
        config.all_peer_addresses()
    );

    let validator = LiveValidator::new(config);
    validator.start().unwrap();

    let state = AppState {
        store: validator.store.clone(),
        engine: validator.engine.clone(),
        block_store: validator.block_store.clone(),
        mempool: validator.mempool.clone(),
        faucet: Arc::new(Mutex::new(amun_rpc::faucet::FaucetState::default())),
        account_store: Arc::new(Mutex::new(amun_accounts::AccountStore::new())),
    };

    eprintln!("RPC server on port {}", rpc_port);
    serve(state, rpc_port).await;
}
