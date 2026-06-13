use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_rpc::{serve, AppState};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let index: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let base_port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(9900);
    let rpc_port: u16 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(9070);
    let quorum: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);

    let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
    let config = ValidatorConfig::test_cluster(index, &ports).with_quorum(quorum);

    eprintln!("Starting validator {} with quorum={} on port {}...", index, quorum, base_port + index as u16);
    let validator = LiveValidator::new(config);
    validator.start().unwrap();

    let state = AppState {
        store: validator.store.clone(),
        engine: validator.engine.clone(),
    };

    eprintln!("RPC server on port {}", rpc_port);
    serve(state, rpc_port).await;
}
