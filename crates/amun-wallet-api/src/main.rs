use amun_wallet_api::server;

#[tokio::main]
async fn main() {
    // Inject real RPC provider
    amun_wallet_api::services::account_service::AccountService::set_provider(
        "http://127.0.0.1:9070",
    );

    let app = server::build_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9081").await.unwrap();
    println!("Wallet API listening on 9081");
    axum::serve(listener, app).await.unwrap();
}
