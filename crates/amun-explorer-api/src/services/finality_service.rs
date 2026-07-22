use crate::error::RpcError;
use crate::types::FinalityCertificateSummary;

pub struct FinalityService;

impl FinalityService {
    pub fn list_certificates() -> Result<Vec<FinalityCertificateSummary>, RpcError> {
        Ok(vec![FinalityCertificateSummary {
            certificate_id: "N41-CERT-001".into(),
            block_hash: "0xblock0000034f".into(),
            block_height: 847,
            quorum_size: 5,
            signed_at: 1700000000,
        }])
    }

    pub fn get_certificate(id: &str) -> Result<FinalityCertificateSummary, RpcError> {
        if id.is_empty() {
            return Err(RpcError::not_found("Finality Certificate", id));
        }
        Ok(FinalityCertificateSummary {
            certificate_id: id.to_string(),
            block_hash: "0xblock0000034f".into(),
            block_height: 847,
            quorum_size: 5,
            signed_at: 1700000000,
        })
    }
}
