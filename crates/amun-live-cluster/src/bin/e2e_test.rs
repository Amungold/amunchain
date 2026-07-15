use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_rpc::client::RpcClient;
use amun_rpc::{build_app, AppState};
use axum::serve;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let ports = [10001, 10002, 10003, 10004];
    let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
    let validator = LiveValidator::new(config).unwrap();
    validator.start().unwrap();

    let state = AppState {
        store: validator.store.clone(),
        engine: validator.engine.clone(),
        mempool: validator.mempool.clone(),
        faucet: Arc::new(Mutex::new(amun_rpc::faucet::FaucetState::default())),
        account_store: Arc::new(Mutex::new(amun_accounts::AccountStore::new())),
        governance: Arc::new(Mutex::new(
            amun_authority_registry::transaction::GovernanceState::new(),
        )),
        authority_registry: Arc::new(Mutex::new(
            amun_authority_registry::AuthorityRegistry::from_genesis(
                amun_authority_registry::ConstitutionalAuthority::new([0u8; 32], 1, 0),
            ),
        )),
        constitutional_kernel: Arc::new(Mutex::new(
            amun_constitutional_enforcement::ConstitutionalEnforcementKernel::new(),
        )),
        certificate_gossip: Arc::new(Mutex::new(amun_consensus_network::CertificateGossip::new())),
        slashing_ledger: Arc::new(Mutex::new(amun_consensus_network::SlashingLedger::new())),
        economic_ledger: Arc::new(Mutex::new(amun_tokenomics_ledger::EconomicLedger::new())),
        previous_evidence_root: Arc::new(Mutex::new([0u8; 32])),
    };
    let app = build_app(state);
    let listener = TcpListener::bind("0.0.0.0:9070").await.unwrap();
    tokio::spawn(async move { serve(listener, app).await.unwrap() });

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
