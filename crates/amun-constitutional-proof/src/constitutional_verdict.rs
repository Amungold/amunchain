#![allow(clippy::too_many_arguments)]
use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{ObligationResult, ObligationResultStatus, VerdictResult};

/// A constitutional verdict issued after evaluating a set of proof obligations.
///
/// The verdict binds a particular subject (a phase, an experiment, a cluster run)
/// to a collection of obligation results and an overall pass/fail outcome.
/// The `verdict_hash` covers the identity fields and results so that the verdict
/// can be referenced immutably in certificates and publication packages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalVerdict {
    pub verdict_id: String,
    pub subject_id: String,
    pub subject_type: String,
    pub phase: String,

    pub obligations_checked: u64,
    pub obligations_satisfied: u64,

    pub results: Vec<ObligationResult>,
    pub overall_result: VerdictResult,

    pub evidence_refs: Vec<String>,

    pub issued_at: u64,
    pub verifier: String,

    /// Hash computed over the immutable identity and results of the verdict.
    /// Excludes `evidence_refs` so that the same verdict can be validated
    /// independently of the evidence storage layer.
    pub verdict_hash: String,
}

impl ConstitutionalVerdict {
    /// Create a new verdict, computing the hash automatically.
    pub fn new(
        verdict_id: String,
        subject_id: String,
        subject_type: String,
        phase: String,
        results: Vec<ObligationResult>,
        overall_result: VerdictResult,
        issued_at: u64,
        verifier: String,
    ) -> Self {
        let obligations_checked = results.len() as u64;
        let obligations_satisfied = results
            .iter()
            .filter(|r| r.status == ObligationResultStatus::Satisfied)
            .count() as u64;

        let evidence_refs = Self::collect_evidence_refs(&results);

        let mut verdict = Self {
            verdict_id,
            subject_id,
            subject_type,
            phase,
            obligations_checked,
            obligations_satisfied,
            results,
            overall_result,
            evidence_refs,
            issued_at,
            verifier,
            verdict_hash: String::new(),
        };

        verdict.verdict_hash = verdict.compute_hash();
        verdict
    }

    /// Verify the integrity of the verdict by recomputing the hash.
    pub fn verify(&self) -> bool {
        self.verdict_hash == self.compute_hash()
    }

    /// Number of obligations that were not satisfied (Failed, Inconclusive, etc.).
    pub fn failed_count(&self) -> u64 {
        self.obligations_checked - self.obligations_satisfied
    }

    // --- private helpers ---

    fn compute_hash(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CONSTITUTIONAL_VERDICT_V1");
        hasher.update(self.verdict_id.as_bytes());
        hasher.update(self.subject_id.as_bytes());
        hasher.update(self.phase.as_bytes());
        hasher.update(&self.issued_at.to_le_bytes());
        hasher.update(self.verifier.as_bytes());
        hasher.update(&self.obligations_checked.to_le_bytes());
        hasher.update(&self.obligations_satisfied.to_le_bytes());

        // Hash the overall result in a stable way
        let overall_str = match &self.overall_result {
            VerdictResult::Pass => "PASS".to_string(),
            VerdictResult::ConditionalPass(conds) => format!("CONDITIONAL:{}", conds.join(",")),
            VerdictResult::Fail(reasons) => format!("FAIL:{}", reasons.join(",")),
        };
        hasher.update(overall_str.as_bytes());

        // Hash each result in order
        for result in &self.results {
            hasher.update(result.obligation_id.to_string().as_bytes());
            let status_str = match result.status {
                ObligationResultStatus::Satisfied => "SATISFIED",
                ObligationResultStatus::Failed => "FAILED",
                ObligationResultStatus::Inconclusive => "INCONCLUSIVE",
                ObligationResultStatus::Waived => "WAIVED",
                ObligationResultStatus::NotApplicable => "NOT_APPLICABLE",
            };
            hasher.update(status_str.as_bytes());
            if let Some(ref reason) = result.failure_reason {
                hasher.update(reason.code.as_bytes());
                hasher.update(reason.description.as_bytes());
            }
        }

        hex::encode(hasher.finalize().as_bytes())
    }

    fn collect_evidence_refs(results: &[ObligationResult]) -> Vec<String> {
        let mut refs: Vec<String> = results
            .iter()
            .flat_map(|r| r.evidence_refs.iter().cloned())
            .collect();
        refs.sort();
        refs.dedup();
        refs
    }
}
