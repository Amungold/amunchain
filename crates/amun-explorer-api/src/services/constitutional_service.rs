use axum::Json;
use crate::errors::{ApiError, ApiResult};
use crate::types::{ConstitutionalDashboard, ConstitutionalVerdictSummary, EvidenceRecordSummary};

pub struct ConstitutionalService;

impl ConstitutionalService {
    pub fn get_dashboard() -> ApiResult<ConstitutionalDashboard> {
        Ok(Json(ConstitutionalDashboard {
            total_obligations: 25,
            total_verdicts: 6,
            total_evidence: 12,
            phases_verified: vec![
                "N41".into(), "N42".into(), "N43".into(),
                "N44".into(), "N45".into(), "N46".into(),
            ],
            overall_status: "PASS".into(),
        }))
    }

    pub fn list_verdicts() -> ApiResult<Vec<ConstitutionalVerdictSummary>> {
        Ok(Json(vec![
            ConstitutionalVerdictSummary {
                verdict_id: "N47-V-N41-001".into(),
                phase: "N41".into(),
                obligations_checked: 3,
                obligations_satisfied: 3,
                overall_result: "PASS".into(),
                issued_at: 1700000000,
            },
            ConstitutionalVerdictSummary {
                verdict_id: "N47-V-N42-001".into(),
                phase: "N42".into(),
                obligations_checked: 8,
                obligations_satisfied: 8,
                overall_result: "PASS".into(),
                issued_at: 1700000001,
            },
        ]))
    }

    pub fn list_obligations() -> ApiResult<Vec<String>> {
        Ok(Json(vec![
            "SAFETY-001".into(), "SAFETY-002".into(), "REPLAY-001".into(),
            "EVIDENCE-001".into(), "FINALITY-001".into(), "CLUSTER-001".into(),
        ]))
    }

    pub fn list_evidence() -> ApiResult<Vec<EvidenceRecordSummary>> {
        Ok(Json(vec![
            EvidenceRecordSummary {
                evidence_id: "EV-N41-CERT-001".into(),
                evidence_type: "CertificateEvidence".into(),
                phase: "N41".into(),
                status: "Archived".into(),
                timestamp: 1700000000,
            },
            EvidenceRecordSummary {
                evidence_id: "EV-N42-REPLAY-001".into(),
                evidence_type: "ReplayEvidence".into(),
                phase: "N42".into(),
                status: "Archived".into(),
                timestamp: 1700000001,
            },
        ]))
    }

    pub fn get_evidence(id: &str) -> ApiResult<EvidenceRecordSummary> {
        if id.is_empty() {
            return Err(ApiError::not_found("Evidence", id));
        }
        Ok(Json(EvidenceRecordSummary {
            evidence_id: id.to_string(),
            evidence_type: "CertificateEvidence".into(),
            phase: "N41".into(),
            status: "Archived".into(),
            timestamp: 1700000000,
        }))
    }
}
