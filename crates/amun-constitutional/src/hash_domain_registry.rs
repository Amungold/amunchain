//! Hash Domain Registry — formal domain governance.
//!
//! Every hash domain is a constitutional namespace boundary.
//! This registry governs domain assignment and prevents:
//!   - Cross-object collision surfaces
//!   - Semantic domain inflation
//!   - Runtime/constitutional contamination
//!
//! INVARIANT: No constitutional object may share a hash domain
//! with an operational object. Constitutional and operational
//! hashing are separate namespaces.

/// Constitutional hash domains — IMMUTABLE.
///
/// These domains define the constitutional namespace.
/// Changing a domain invalidates ALL historical artifacts
/// hashed under the old domain.
pub mod constitutional_domains {
    /// Core constitutional object domains
    pub const CONSTITUTIONAL_FAILURE: &[u8] = b"CONSTITUTIONAL_FAILURE";
    pub const EXECUTION_LIMITS: &[u8] = b"EXECUTION_LIMITS";
    pub const EXECUTION_CONTEXT: &[u8] = b"EXECUTION_CONTEXT";
    pub const EXECUTION_BOUNDARY: &[u8] = b"EXECUTION_BOUNDARY";
    pub const JOURNAL_ENTRY: &[u8] = b"JOURNAL_ENTRY";
    pub const EXECUTION_JOURNAL: &[u8] = b"EXECUTION_JOURNAL";
    pub const TRANSITION_EVIDENCE: &[u8] = b"TRANSITION_EVIDENCE";
    pub const TRANSITION_COMMITMENT: &[u8] = b"TRANSITION_COMMITMENT";

    /// Admissibility + proof domains
    pub const REPLAY_OUTCOME: &[u8] = b"REPLAY_OUTCOME";
    pub const EXECUTION_RECEIPT: &[u8] = b"EXECUTION_RECEIPT";
    pub const REPLAY_CERTIFICATE: &[u8] = b"REPLAY_CERTIFICATE";
    pub const STATE_ANCHOR: &[u8] = b"STATE_ANCHOR";
    pub const SNAPSHOT: &[u8] = b"SNAPSHOT";

    /// Restoration + continuity domains
    pub const RESTORATION_POINT: &[u8] = b"RESTORATION_POINT";
    pub const CONTINUATION_CHAIN: &[u8] = b"CONTINUATION_CHAIN";

    /// Divergence domains
    pub const DIVERGENCE_POINT: &[u8] = b"DIVERGENCE_POINT";
    pub const DIVERGENCE_RESOLUTION: &[u8] = b"DIVERGENCE_RESOLUTION";

    /// Causality domains
    pub const CAUSAL_EDGE: &[u8] = b"CAUSAL_EDGE";
    pub const CAUSALITY_CHAIN: &[u8] = b"CAUSALITY_CHAIN";

    /// Witness + proof domains
    pub const WITNESS: &[u8] = b"CONSTITUTIONAL_WITNESS";

    /// Sub-domains for scoped artifacts (used with .update_bytes prefix)
    pub const SCOPE_PREFIX: &[u8] = b"SCOPE";
    pub const STATE_ANCHOR_SCOPE: &[u8] = b"STATE_ANCHOR_SCOPE";
    pub const SNAPSHOT_SCOPE: &[u8] = b"SNAPSHOT_SCOPE";
    pub const DIVERGENCE_TYPE: &[u8] = b"DIVERGENCE_TYPE";
    pub const RESOLUTION_TYPE: &[u8] = b"RESOLUTION_TYPE";
    pub const WITNESS_TYPE: &[u8] = b"WITNESS_TYPE";
    pub const SNAPSHOT_OUTCOME: &[u8] = b"SNAPSHOT_OUTCOME";
}

/// Operational hash domains — for runtime use only.
///
/// These are SEPARATE from constitutional domains.
/// No constitutional object may use an operational domain.
pub mod operational_domains {
    /// Runtime execution domains
    pub const RUNTIME_RECEIPT: &[u8] = b"RUNTIME_RECEIPT";
    pub const EXECUTION_TASK: &[u8] = b"EXECUTION_TASK";
    pub const SCHEDULER: &[u8] = b"EXECUTION_SCHEDULER";
}

/// Domain separation verification.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constitutional_domains_unique() {
        let domains = [
            constitutional_domains::CONSTITUTIONAL_FAILURE,
            constitutional_domains::EXECUTION_LIMITS,
            constitutional_domains::EXECUTION_CONTEXT,
            constitutional_domains::EXECUTION_BOUNDARY,
            constitutional_domains::JOURNAL_ENTRY,
            constitutional_domains::EXECUTION_JOURNAL,
            constitutional_domains::TRANSITION_EVIDENCE,
            constitutional_domains::TRANSITION_COMMITMENT,
            constitutional_domains::REPLAY_OUTCOME,
            constitutional_domains::EXECUTION_RECEIPT,
            constitutional_domains::REPLAY_CERTIFICATE,
            constitutional_domains::STATE_ANCHOR,
            constitutional_domains::SNAPSHOT,
            constitutional_domains::RESTORATION_POINT,
            constitutional_domains::CONTINUATION_CHAIN,
            constitutional_domains::DIVERGENCE_POINT,
            constitutional_domains::DIVERGENCE_RESOLUTION,
            constitutional_domains::CAUSAL_EDGE,
            constitutional_domains::CAUSALITY_CHAIN,
            constitutional_domains::WITNESS,
        ];
        for i in 0..domains.len() {
            for j in (i + 1)..domains.len() {
                assert_ne!(domains[i], domains[j]);
            }
        }
    }

    #[test]
    fn test_no_constitutional_operational_overlap() {
        let constitutional = [
            constitutional_domains::CONSTITUTIONAL_FAILURE,
            constitutional_domains::WITNESS,
            constitutional_domains::REPLAY_CERTIFICATE,
        ];
        let operational = [
            operational_domains::RUNTIME_RECEIPT,
            operational_domains::EXECUTION_TASK,
            operational_domains::SCHEDULER,
        ];
        for cd in &constitutional {
            for od in &operational {
                assert_ne!(cd, od);
            }
        }
    }
}
