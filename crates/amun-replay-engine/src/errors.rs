// ============================================================================
// REPLAY ERRORS — DETERMINISTIC, NO EXTERNAL DEPENDENCIES
// ============================================================================

use amun_constitutional::ConstitutionalHash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayFailure {
    OrderingViolation {
        expected_sequence: u64,
        actual_sequence: u64,
    },
    StateDivergence {
        expected_root: ConstitutionalHash,
        actual_root: ConstitutionalHash,
    },
    EquivalenceMismatch {
        transcript_hash: ConstitutionalHash,
        expected_root: ConstitutionalHash,
        actual_root: ConstitutionalHash,
    },
    CausalityViolation {
        reason: &'static str,
    },
    CheckpointMismatch {
        expected: ConstitutionalHash,
        actual: ConstitutionalHash,
    },
    AuthorityViolation {
        authority_id: ConstitutionalHash,
    },
    CanonicalError {
        reason: &'static str,
    },
}
