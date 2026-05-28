//! Hash domains — constitutional domain separation constants.
//!
//! Every constitutional object type has a unique domain tag.
//! These constants are the SINGLE source of truth for domain identifiers.
//!
//! RATIONALE:
//!   - Prevents domain drift (typos, inconsistencies across modules)
//!   - Enables future formal verification of domain separation
//!   - Centralizes hashing governance
//!
//! ADDING A NEW DOMAIN:
//!   This is a constitutional change. All domain tags MUST be unique.
//!   Changing an existing tag invalidates ALL historical artifacts.

/// Domain: ConstitutionalFailure objects
pub const DOMAIN_CONSTITUTIONAL_FAILURE: &[u8] = b"CONSTITUTIONAL_FAILURE";

/// Domain: ExecutionLimits objects
pub const DOMAIN_EXECUTION_LIMITS: &[u8] = b"EXECUTION_LIMITS";

/// Domain: ExecutionContext objects
pub const DOMAIN_EXECUTION_CONTEXT: &[u8] = b"EXECUTION_CONTEXT";

/// Domain: ExecutionBoundary objects
pub const DOMAIN_EXECUTION_BOUNDARY: &[u8] = b"EXECUTION_BOUNDARY";

/// Domain: JournalEntry objects
pub const DOMAIN_JOURNAL_ENTRY: &[u8] = b"JOURNAL_ENTRY";

/// Domain: ExecutionJournal objects
pub const DOMAIN_EXECUTION_JOURNAL: &[u8] = b"EXECUTION_JOURNAL";

/// Domain: TransitionEvidence objects
pub const DOMAIN_TRANSITION_EVIDENCE: &[u8] = b"TRANSITION_EVIDENCE";

/// Domain: TransitionCommitment objects
pub const DOMAIN_TRANSITION_COMMITMENT: &[u8] = b"TRANSITION_COMMITMENT";

/// Domain: ReplayOutcome objects
pub const DOMAIN_REPLAY_OUTCOME: &[u8] = b"REPLAY_OUTCOME";

/// Domain: ExecutionReceipt objects
pub const DOMAIN_EXECUTION_RECEIPT: &[u8] = b"EXECUTION_RECEIPT";

/// Domain: ReplayCertificate objects
pub const DOMAIN_REPLAY_CERTIFICATE: &[u8] = b"REPLAY_CERTIFICATE";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_all_domains_unique() {
        let domains: Vec<&[u8]> = vec![
            DOMAIN_CONSTITUTIONAL_FAILURE,
            DOMAIN_EXECUTION_LIMITS,
            DOMAIN_EXECUTION_CONTEXT,
            DOMAIN_EXECUTION_BOUNDARY,
            DOMAIN_JOURNAL_ENTRY,
            DOMAIN_EXECUTION_JOURNAL,
            DOMAIN_TRANSITION_EVIDENCE,
            DOMAIN_TRANSITION_COMMITMENT,
            DOMAIN_REPLAY_OUTCOME,
            DOMAIN_EXECUTION_RECEIPT,
            DOMAIN_REPLAY_CERTIFICATE,
        ];
        for i in 0..domains.len() {
            for j in (i + 1)..domains.len() {
                assert_ne!(
                    domains[i],
                    domains[j],
                    "Hash domain collision: {:?} == {:?}",
                    String::from_utf8_lossy(domains[i]),
                    String::from_utf8_lossy(domains[j])
                );
            }
        }
    }
}
