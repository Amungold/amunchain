use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_rpc::client::RpcClient;
use amun_rpc::{serve, AppState};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let ports = [10001, 10002, 10003, 10004];
    let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
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
    tokio::spawn(async move { serve(state, 9070).await });

    // Wait for RPC to be ready
    let rpc = RpcClient::new("127.0.0.1", 9070);
    for _ in 0..30 {
        if rpc.get_status().is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Poll until store has at least 3 blocks
    println!("Waiting for blocks in store...");
    for _ in 0..60 {
        if let Ok(h) = rpc.get_head() {
            if h.height >= 3 {
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    let head = rpc.get_head().expect("/head");
    println!("Store has {} blocks", head.height);

    let status = rpc.get_status().expect("/status");
    println!("[PASS] /status: height={}", status.height);

    println!("[PASS] /head: height={}", head.height);

    let _block = rpc.get_block(head.height).expect("/block");
    println!("[PASS] /block/{}: ok", head.height);

    let range = rpc.get_block_range(1, head.height).expect("/blocks range");
    println!(
        "[PASS] /blocks/1/{}: {} blocks",
        head.height,
        range.blocks.len()
    );

    let metrics = rpc.get_metrics().expect("/metrics");
    println!("[PASS] /metrics: height={}", metrics.height);

    validator.stop();

    println!("\n============================================");
    println!("  N102.6 — E2E INTEGRATION TEST — PASS");
    println!("============================================");
}
