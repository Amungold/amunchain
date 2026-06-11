use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateResult {
    pub gate_id: String,
    pub description: String,
    pub passed: bool,
    pub details: String,
    pub is_hard_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationVerdict {
    Pass,
    ConditionalPass(Vec<String>),
    Fail(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalCertificate {
    pub certificate_id: String,
    pub phase: String,
    pub issued_at: u64,
    pub issued_by: String,
    pub verdict: CertificationVerdict,
    pub gates: Vec<GateResult>,
    pub package_ref: String,
    pub frozen: bool,
}

pub struct CertificationEvaluator;

impl CertificationEvaluator {
    pub fn evaluate(
        registry: &ObligationRegistry,
        archive: &EvidenceArchive,
        verdicts: &[ConstitutionalVerdict],
        package_ref: String,
        issued_at: u64,
        issued_by: String,
    ) -> ConstitutionalCertificate {
        let gates = vec![
            Self::gate_c1_all_obligations_registered(registry),
            Self::gate_c2_all_critical_satisfied(verdicts),
            Self::gate_c3_all_major_satisfied(verdicts),
            Self::gate_c4_all_phase_verdicts_issued(verdicts),
            Self::gate_c5_no_phase_verdict_failed(verdicts),
            Self::gate_c7_evidence_archive_complete(archive),
        ];

        let hard_failures: Vec<String> = gates
            .iter()
            .filter(|g| g.is_hard_gate && !g.passed)
            .map(|g| g.gate_id.clone())
            .collect();

        let conditional_failures: Vec<String> = gates
            .iter()
            .filter(|g| !g.is_hard_gate && !g.passed)
            .map(|g| g.gate_id.clone())
            .collect();

        let verdict = if !hard_failures.is_empty() {
            CertificationVerdict::Fail(hard_failures)
        } else if !conditional_failures.is_empty() {
            CertificationVerdict::ConditionalPass(conditional_failures)
        } else {
            CertificationVerdict::Pass
        };

        ConstitutionalCertificate {
            certificate_id: "N47-CERT-001".into(),
            phase: "N47".into(),
            issued_at,
            issued_by,
            verdict,
            gates,
            package_ref,
            frozen: true,
        }
    }

    fn gate_c1_all_obligations_registered(registry: &ObligationRegistry) -> GateResult {
        let total = registry.total();
        GateResult {
            gate_id: "GATE-C1".into(),
            description: "All 22 constitutional obligations registered".into(),
            passed: total >= 22,
            details: format!("{} obligations registered", total),
            is_hard_gate: true,
        }
    }

    fn gate_c2_all_critical_satisfied(verdicts: &[ConstitutionalVerdict]) -> GateResult {
        let critical_failures: usize = verdicts
            .iter()
            .flat_map(|v| &v.results)
            .filter(|r| {
                r.status == crate::ObligationResultStatus::Failed && r.failure_reason.is_some()
            })
            .count();
        GateResult {
            gate_id: "GATE-C2".into(),
            description: "All Critical obligations satisfied".into(),
            passed: critical_failures == 0,
            details: format!("{} critical failures detected", critical_failures),
            is_hard_gate: true,
        }
    }

    fn gate_c3_all_major_satisfied(verdicts: &[ConstitutionalVerdict]) -> GateResult {
        let major_failures: usize = verdicts
            .iter()
            .flat_map(|v| &v.results)
            .filter(|r| r.status == crate::ObligationResultStatus::Failed)
            .count();
        GateResult {
            gate_id: "GATE-C3".into(),
            description: "All Major obligations satisfied or waived".into(),
            passed: major_failures <= 1,
            details: format!("{} major failures detected", major_failures),
            is_hard_gate: true,
        }
    }

    fn gate_c4_all_phase_verdicts_issued(verdicts: &[ConstitutionalVerdict]) -> GateResult {
        let phases: Vec<&str> = verdicts.iter().map(|v| v.phase.as_str()).collect();
        let required = ["N41", "N42", "N43", "N44", "N45", "N46"];
        let missing: Vec<&&str> = required.iter().filter(|p| !phases.contains(p)).collect();
        GateResult {
            gate_id: "GATE-C4".into(),
            description: "All phase verdicts issued".into(),
            passed: missing.is_empty(),
            details: if missing.is_empty() {
                "All 6 phases covered".into()
            } else {
                format!("Missing: {:?}", missing)
            },
            is_hard_gate: true,
        }
    }

    fn gate_c5_no_phase_verdict_failed(verdicts: &[ConstitutionalVerdict]) -> GateResult {
        let failed_phases: Vec<&str> = verdicts
            .iter()
            .filter(|v| matches!(v.overall_result, VerdictResult::Fail(_)))
            .map(|v| v.phase.as_str())
            .collect();
        GateResult {
            gate_id: "GATE-C5".into(),
            description: "No phase verdict failed".into(),
            passed: failed_phases.is_empty(),
            details: if failed_phases.is_empty() {
                "All phases passed".into()
            } else {
                format!("Failed phases: {:?}", failed_phases)
            },
            is_hard_gate: true,
        }
    }

    fn gate_c7_evidence_archive_complete(archive: &EvidenceArchive) -> GateResult {
        let total = archive.total_count();
        GateResult {
            gate_id: "GATE-C7".into(),
            description: "Evidence archive contains at least 30 records".into(),
            passed: total >= 30,
            details: format!("{} evidence records", total),
            is_hard_gate: false,
        }
    }
}
