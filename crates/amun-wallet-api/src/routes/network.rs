use crate::services::network_service::NetworkService;
use axum::{routing::get, Router};

async fn get_network_info() -> crate::errors::ApiResult<crate::types::NetworkInfoResponse> {
    NetworkService::get_info()
}
pub fn network_routes() -> Router {
    Router::new().route("/info", get(get_network_info))
}
