use crate::error::RpcError;
use crate::types::{ConstitutionalDashboard, ConstitutionalVerdictSummary, EvidenceRecordSummary};

pub struct ConstitutionalService;

impl ConstitutionalService {
    pub fn get_dashboard() -> Result<ConstitutionalDashboard, RpcError> {
        Ok(ConstitutionalDashboard {
            total_obligations: 25,
            total_verdicts: 6,
            total_evidence: 12,
            total_phases: 7,
            active_phases: 1,
            completed_phases: 6,
            failed_obligations: 3,
            pending_obligations: 4,
            met_obligations: 18,
            overall_status: "PASS".into(),
        })
    }

    pub fn list_verdicts() -> Result<Vec<ConstitutionalVerdictSummary>, RpcError> {
        Ok(vec![ConstitutionalVerdictSummary {
            verdict_id: "N47-V-N41-001".into(),
            phase: "N41".into(),
            oblig: "N41-REPLAY-001".into(),
            oblig_type: "REPLAY".into(),
            status: "Pass".into(),
            validator: "node-1".into(),
            validator_alt: "node-2".into(),
            timestamp: 1700000000,
        }])
    }

    pub fn list_obligations() -> Result<Vec<String>, RpcError> {
        Ok(vec![
            "SAFETY-001".into(),
            "SAFETY-002".into(),
            "REPLAY-001".into(),
            "CONSENSUS-001".into(),
            "CLUSTER-001".into(),
        ])
    }

    pub fn list_evidence() -> Result<Vec<EvidenceRecordSummary>, RpcError> {
        Ok(vec![EvidenceRecordSummary {
            evidence_id: "EV-N41-CERT-001".into(),
            evidence_type: "CertificateEvidence".into(),
            phase: "N41".into(),
            status: "Archived".into(),
            timestamp: 1700000000,
        }])
    }

    pub fn get_evidence(id: &str) -> Result<EvidenceRecordSummary, RpcError> {
        if id.is_empty() {
            return Err(RpcError::not_found("Evidence", id));
        }
        Ok(EvidenceRecordSummary {
            evidence_id: id.to_string(),
            evidence_type: "CertificateEvidence".into(),
            phase: "N41".into(),
            status: "Archived".into(),
            timestamp: 1700000000,
        })
    }
}
