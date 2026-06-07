use axum::{extract::Path, routing::get, Router};
use crate::services::finality_service::FinalityService;

async fn list_certificates() -> crate::errors::ApiResult<Vec<crate::types::FinalityCertificateSummary>> {
    FinalityService::list_certificates()
}
async fn get_certificate(Path(id): Path<String>) -> crate::errors::ApiResult<crate::types::FinalityCertificateSummary> {
    FinalityService::get_certificate(&id)
}

pub fn finality_routes() -> Router {
    Router::new()
        .route("/certificates", get(list_certificates))
        .route("/certificates/{id}", get(get_certificate))
}
