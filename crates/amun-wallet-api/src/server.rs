use crate::routes::{accounts, chain, network, transactions};
use axum::Router;

pub fn build_app() -> Router {
    Router::new()
        .nest("/accounts", accounts::account_routes())
        .nest("/transactions", transactions::transaction_routes())
        .nest("/chain", chain::chain_routes())
        .nest("/network", network::network_routes())
}
