use amun_rpc::provider::LiveRpcProvider;
use amun_wallet_api::server;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Inject real RPC provider
    let provider = Arc::new(LiveRpcProvider::new("127.0.0.1", 9070));
    amun_wallet_api::services::account_service::set_provider(provider.clone());

    let app = server::build_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9071").await.unwrap();
    println!("Wallet API listening on 9071");
    axum::serve(listener, app).await.unwrap();
}
