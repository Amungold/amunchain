use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use amun_resource_core::ResourceId;

use crate::invariant_types::{InvariantDeclaration, InvariantResult, InvariantSeverity};

/// Evaluates contract invariants after commit (Phase 5 in N48.5-D).
pub struct InvariantEngine;

impl InvariantEngine {
    /// Evaluate all declared invariants against the post-state.
    /// Returns results and any violation evidence for Critical failures.
    pub fn evaluate<F>(
        invariants: &[InvariantDeclaration],
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
        state_root: [u8; 32],
        check_fn: F,
    ) -> (Vec<InvariantResult>, Vec<ConstitutionalEvidence>)
    where
        F: Fn(&InvariantDeclaration) -> bool,
    {
        let mut results = Vec::new();
        let mut evidence = Vec::new();

        for inv in invariants {
            let passed = check_fn(inv);
            results.push(InvariantResult {
                obligation_id: inv.obligation_id.clone(),
                passed,
                severity: inv.severity,
            });

            if !passed {
                evidence.push(ConstitutionalEvidence::InvariantViolation {
                    obligation_id: inv.obligation_id.clone(),
                    contract_id,
                    block_height,
                    transaction_hash,
                    state_root,
                });
            }
        }

        (results, evidence)
    }

    /// Count results by severity.
    pub fn count_by_severity(results: &[InvariantResult], severity: InvariantSeverity) -> usize {
        results
            .iter()
            .filter(|r| r.severity == severity && !r.passed)
            .count()
    }

    /// Returns true if any Critical invariant failed.
    pub fn has_critical_failure(results: &[InvariantResult]) -> bool {
        results
            .iter()
            .any(|r| r.severity == InvariantSeverity::Critical && !r.passed)
    }

    /// Returns true if all invariants passed.
    pub fn all_passed(results: &[InvariantResult]) -> bool {
        results.iter().all(|r| r.passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invariant_types::{InvariantDeclaration, InvariantScope};
    use amun_resource_core::ResourceId;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    fn sample_invariants() -> Vec<InvariantDeclaration> {
        vec![
            InvariantDeclaration {
                obligation_id: "SAFETY-001".into(),
                description: "Total supply non-negative".into(),
                severity: InvariantSeverity::Critical,
                scope: InvariantScope::State,
            },
            InvariantDeclaration {
                obligation_id: "SAFETY-002".into(),
                description: "Accounts balanced".into(),
                severity: InvariantSeverity::Critical,
                scope: InvariantScope::State,
            },
            InvariantDeclaration {
                obligation_id: "PERF-001".into(),
                description: "TPS threshold".into(),
                severity: InvariantSeverity::Minor,
                scope: InvariantScope::Local,
            },
        ]
    }

    #[test]
    fn w8_all_invariants_pass() {
        let invs = sample_invariants();
        let (results, evidence) = InvariantEngine::evaluate(
            &invs,
            make_id(1),
            1,
            [0xaa; 32],
            [0x01; 32],
            |_| true, // all pass
        );
        assert!(InvariantEngine::all_passed(&results));
        assert!(!InvariantEngine::has_critical_failure(&results));
        assert_eq!(evidence.len(), 0);
    }

    #[test]
    fn w8_critical_failure_produces_evidence() {
        let invs = sample_invariants();
        let (results, evidence) = InvariantEngine::evaluate(
            &invs,
            make_id(2),
            42,
            [0xbb; 32],
            [0x02; 32],
            |inv| inv.obligation_id != "SAFETY-001", // SAFETY-001 fails
        );
        assert!(!InvariantEngine::all_passed(&results));
        assert!(InvariantEngine::has_critical_failure(&results));
        assert_eq!(evidence.len(), 1);
        match &evidence[0] {
            ConstitutionalEvidence::InvariantViolation { obligation_id, .. } => {
                assert_eq!(obligation_id, "SAFETY-001");
            }
            _ => panic!("Expected InvariantViolation"),
        }
    }

    #[test]
    fn w8_minor_failure_produces_evidence_but_not_critical() {
        let invs = sample_invariants();
        let (results, evidence) = InvariantEngine::evaluate(
            &invs,
            make_id(3),
            10,
            [0xcc; 32],
            [0x03; 32],
            |inv| inv.obligation_id != "PERF-001", // PERF-001 fails (Minor)
        );
        assert!(!InvariantEngine::all_passed(&results));
        assert!(!InvariantEngine::has_critical_failure(&results)); // No Critical failed
        assert_eq!(evidence.len(), 1); // Still produces evidence
    }

    #[test]
    fn w8_count_by_severity() {
        let invs = sample_invariants();
        let (results, _) = InvariantEngine::evaluate(
            &invs,
            make_id(4),
            5,
            [0xdd; 32],
            [0x04; 32],
            |inv| inv.obligation_id == "PERF-001", // only PERF-001 passes
        );
        assert_eq!(
            InvariantEngine::count_by_severity(&results, InvariantSeverity::Critical),
            2
        );
        assert_eq!(
            InvariantEngine::count_by_severity(&results, InvariantSeverity::Minor),
            0
        );
    }

    #[test]
    fn w8_evidence_contains_state_root() {
        let invs = sample_invariants();
        let state_root = [0xab; 32];
        let (_, evidence) = InvariantEngine::evaluate(
            &invs,
            make_id(5),
            99,
            [0xee; 32],
            state_root,
            |_| false, // all fail
        );
        assert_eq!(evidence.len(), 3);
        for ev in &evidence {
            match ev {
                ConstitutionalEvidence::InvariantViolation { state_root: sr, .. } => {
                    assert_eq!(*sr, state_root);
                }
                _ => panic!("Expected InvariantViolation"),
            }
        }
    }
}
