use crate::errors::{ApiError, ApiResult};
use crate::types::FinalityCertificateSummary;
use axum::Json;

pub struct FinalityService;

impl FinalityService {
    pub fn list_certificates() -> ApiResult<Vec<FinalityCertificateSummary>> {
        Ok(Json(vec![
            FinalityCertificateSummary {
                certificate_id: "N41-CERT-001".into(),
                block_hash: "0xblock0000034f".into(),
                block_height: 847,
                quorum_size: 5,
                signed_at: 1700000000,
            },
            FinalityCertificateSummary {
                certificate_id: "N41-CERT-002".into(),
                block_hash: "0xblock0000034e".into(),
                block_height: 846,
                quorum_size: 5,
                signed_at: 1699999990,
            },
        ]))
    }

    pub fn get_certificate(id: &str) -> ApiResult<FinalityCertificateSummary> {
        if id.is_empty() {
            return Err(ApiError::not_found("Finality Certificate", id));
        }
        Ok(Json(FinalityCertificateSummary {
            certificate_id: id.to_string(),
            block_hash: "0xblock0000034f".into(),
            block_height: 847,
            quorum_size: 5,
            signed_at: 1700000000,
        }))
    }
}
