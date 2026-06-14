use amun_explorer_api::server;
use amun_rpc::provider::LiveRpcProvider;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Inject real RPC provider
    let provider = Arc::new(LiveRpcProvider::new("127.0.0.1", 9070));
    amun_explorer_api::services::chain_service::set_provider(provider.clone());
    
    let app = server::build_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9072")
        .await
        .unwrap();
    println!("Explorer API listening on 9072");
    axum::serve(listener, app).await.unwrap();
}
