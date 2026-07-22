use amun_explorer_api::config::ExplorerConfig;
use amun_explorer_api::rpc::client::RpcClient;
use amun_explorer_api::state::AppState;
use reqwest::Url;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = Arc::new(ExplorerConfig::from_env());

    // Build reqwest client (timeouts, etc. configured here — not in RpcClient)
    let http_client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("failed to build HTTP client");

    // Build RPC client
    let rpc_url = Url::parse(&config.rpc_base_url).expect("invalid AMUN_RPC_URL");
    let rpc = Arc::new(RpcClient::new(rpc_url, http_client));

    // Store the base_url before moving into state
    let rpc_endpoint = rpc.base_url().clone();

    // Build application state
    let _state = AppState { config, rpc };

    println!("Amun Explorer API starting...");
    println!("RPC endpoint: {}", rpc_endpoint);
    println!("Ready (server not yet wired — Phase 5+)");

    // TODO: Phase 5 — wire up axum router with new handlers
    // TODO: Phase 6 — bind to listen_addr and serve
}
