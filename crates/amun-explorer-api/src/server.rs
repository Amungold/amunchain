use axum::Router;
use crate::routes::{accounts, chain, constitutional, finality};

pub fn build_app() -> Router {
    Router::new()
        .nest("/explorer/chain", chain::chain_routes())
        .nest("/explorer/account", accounts::account_routes())
        .nest("/explorer/finality", finality::finality_routes())
        .nest("/explorer/constitutional", constitutional::constitutional_routes())
}
