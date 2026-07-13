#![allow(clippy::too_many_arguments)]
use crate::{
    ConstitutionalVerdict, ObligationResult, ObligationResultStatus, ObligationSeverity,
    ProofObligation, VerdictResult,
};

/// Applies Article II evaluation rules to a set of obligation results and
/// produces a `ConstitutionalVerdict`.
///
/// Rules (Article II, Section 2):
/// - Any Critical failure => Fail
/// - Two or more Major failures => Fail
/// - Exactly one Major failure => ConditionalPass
/// - Minor or Advisory failures do not affect Pass/Fail
pub struct VerdictEvaluator;

impl VerdictEvaluator {
    /// Evaluate a collection of obligation results against their definitions.
    ///
    /// Returns a fully-formed `ConstitutionalVerdict` with the overall result
    /// computed according to the constitutional rules.
    pub fn evaluate(
        verdict_id: String,
        subject_id: String,
        subject_type: String,
        phase: String,
        obligations: &[ProofObligation],
        results: Vec<ObligationResult>,
        issued_at: u64,
        verifier: String,
    ) -> ConstitutionalVerdict {
        let overall = Self::compute_overall_result(obligations, &results);
        ConstitutionalVerdict::new(
            verdict_id,
            subject_id,
            subject_type,
            phase,
            results,
            overall,
            issued_at,
            verifier,
        )
    }

    /// Compute the overall `VerdictResult` given the obligation definitions
    /// and their evaluation results.
    pub fn compute_overall_result(
        obligations: &[ProofObligation],
        results: &[ObligationResult],
    ) -> VerdictResult {
        let mut critical_failures = Vec::new();
        let mut major_failures = Vec::new();
        let mut minor_failures = Vec::new();
        let mut advisory_failures = Vec::new();

        for result in results {
            if result.status == ObligationResultStatus::Satisfied
                || result.status == ObligationResultStatus::Waived
                || result.status == ObligationResultStatus::NotApplicable
            {
                continue;
            }

            // Find the matching obligation definition to get severity
            let severity = obligations
                .iter()
                .find(|o| o.id == result.obligation_id)
                .map(|o| o.severity);

            let description = result
                .failure_reason
                .as_ref()
                .map(|r| r.description.clone())
                .unwrap_or_else(|| format!("{} failed", result.obligation_id));

            match severity {
                Some(ObligationSeverity::Critical) => critical_failures.push(description),
                Some(ObligationSeverity::Major) => major_failures.push(description),
                Some(ObligationSeverity::Minor) => minor_failures.push(description),
                Some(ObligationSeverity::Advisory) => advisory_failures.push(description),
                None => {
                    // If no obligation definition found, treat as Major by default
                    major_failures.push(description);
                }
            }
        }

        // Rule: Critical failure => Fail
        if !critical_failures.is_empty() {
            let mut reasons = critical_failures;
            reasons.extend(major_failures);
            reasons.extend(minor_failures);
            reasons.extend(advisory_failures);
            return VerdictResult::Fail(reasons);
        }

        // Rule: 2+ Major failures => Fail
        if major_failures.len() >= 2 {
            let mut reasons = major_failures;
            reasons.extend(minor_failures);
            reasons.extend(advisory_failures);
            return VerdictResult::Fail(reasons);
        }

        // Rule: 1 Major failure => ConditionalPass
        if major_failures.len() == 1 {
            let mut conditions = major_failures;
            conditions.extend(minor_failures);
            conditions.extend(advisory_failures);
            return VerdictResult::ConditionalPass(conditions);
        }

        // Only Minor/Advisory failures remain (or none): Pass
        if minor_failures.is_empty() && advisory_failures.is_empty() {
            VerdictResult::Pass
        } else {
            // Minor/Advisory failures exist but don't block Pass
            // Include them as informational conditions
            let mut conditions = minor_failures;
            conditions.extend(advisory_failures);
            if conditions.is_empty() {
                VerdictResult::Pass
            } else {
                VerdictResult::ConditionalPass(conditions)
            }
        }
    }
}
