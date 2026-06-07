use axum::{routing::get, Router};
use crate::services::network_service::NetworkService;

async fn get_network_info() -> crate::errors::ApiResult<crate::types::NetworkInfoResponse> {
    NetworkService::get_info()
}
pub fn network_routes() -> Router {
    Router::new().route("/info", get(get_network_info))
}
