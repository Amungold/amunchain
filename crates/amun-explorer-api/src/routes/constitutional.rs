use axum::{extract::Path, routing::get, Router};
use crate::services::constitutional_service::ConstitutionalService;

async fn get_dashboard() -> crate::errors::ApiResult<crate::types::ConstitutionalDashboard> {
    ConstitutionalService::get_dashboard()
}
async fn list_verdicts() -> crate::errors::ApiResult<Vec<crate::types::ConstitutionalVerdictSummary>> {
    ConstitutionalService::list_verdicts()
}
async fn list_obligations() -> crate::errors::ApiResult<Vec<String>> {
    ConstitutionalService::list_obligations()
}
async fn list_evidence() -> crate::errors::ApiResult<Vec<crate::types::EvidenceRecordSummary>> {
    ConstitutionalService::list_evidence()
}
async fn get_evidence(Path(id): Path<String>) -> crate::errors::ApiResult<crate::types::EvidenceRecordSummary> {
    ConstitutionalService::get_evidence(&id)
}

pub fn constitutional_routes() -> Router {
    Router::new()
        .route("/dashboard", get(get_dashboard))
        .route("/verdicts", get(list_verdicts))
        .route("/obligations", get(list_obligations))
        .route("/evidence", get(list_evidence))
        .route("/evidence/{id}", get(get_evidence))
}
