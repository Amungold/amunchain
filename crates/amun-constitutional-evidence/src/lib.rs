//! # Constitutional Evidence Module
//!
//! This module bridges CCS theory with AmunChain runtime by defining
//! the constitutional evidence types and the core verification function
//! that answers: "Is this evidence valid within this constitutional context?"
//!
//! ## CCS Theory Mapping
//! - Context `C = (E, V, H)` → `ConstitutionalContext`
//! - Evidence `E` → `QuorumCertificate`
//! - Validity check → `is_evidence_valid_for_context()`
//!
//! This is the single function that embodies derivability (`⊢_C`) and
//! exclusion (`⇍_C`) in the runtime.

use amun_kernel_types::epoch::Epoch;
use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::validator_set::ValidatorSet;

/// Represents the constitutional context `C = (E, V, H)` from CCS theory.
/// 
/// - `epoch`: The current constitutional epoch `E`.
/// - `validator_set`: The set of validators authorized in this context `V`.
/// - `state_root`: The finalized state hash `H` that this context is anchored to.
#[derive(Debug, Clone)]
pub struct ConstitutionalContext {
    pub epoch: Epoch,
    pub validator_set: ValidatorSet,
    pub state_root: [u8; 32],
}

/// Represents constitutional evidence submitted to authorize a state transition.
/// In the current implementation, this is the Quorum Certificate (QC).
#[derive(Debug, Clone)]
pub struct ConstitutionalEvidence {
    pub qc: QuorumCertificate,
}

/// The core constitutional function: determines whether a given piece of
/// evidence is valid within a specific constitutional context.
///
/// This function embodies the constitutional rules of CCS:
/// 1. **Epoch Supremacy (Exclusion):** The QC must belong to the current epoch.
/// 2. **Constitutional Membership (Exclusion):** The QC must be attested by a
///    quorum of validators from the *current* constitutional validator set.
/// 3. **Evidence Strength (Comparability):** The QC must meet the quorum
///    threshold for the current validator set.
///
/// # Returns
/// `true` if the evidence is constitutionally valid, meaning the state
/// transition it authorizes is legitimate. `false` otherwise.
pub fn is_evidence_valid_for_context(
    evidence: &ConstitutionalEvidence,
    context: &ConstitutionalContext,
) -> bool {
    // Rule 1: Epoch Check (Exclusion ⇍_C)
    // Stale evidence from previous epochs is rejected.
    if evidence.qc.position.epoch != context.epoch.value() {
        return false;
    }

    // Rule 2: Constitutional Membership (Exclusion ⇍_C)
    // The QC's signers must be validators in the current constitutional set.
    // This rejects foreign or outdated validator attestations.
    let signers = evidence.qc.voter_ids();
    let validators = context.validator_set.validator_ids();
    for signer in &signers {
        if !validators.contains(signer) {
            return false;
        }
    }

    // Rule 3: Quorum Check (Comparability CC → SH1)
    // The evidence must carry sufficient weight to meet the constitutional
    // quorum threshold. This ensures that no conflicting evidence of equal
    // or greater weight can exist.
    let evidence_weight = evidence.qc.total_weight();
    if !context.validator_set.has_quorum(evidence_weight) {
        return false;
    }

    // If all checks pass, the evidence is constitutionally valid.
    // This single `true` return value is the runtime embodiment of the
    // CCS derivability relation (`⊢_C`).
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_fails_for_wrong_epoch() {
        // TODO: Add test once QuorumCertificate construction is available
    }

    #[test]
    fn test_evidence_fails_for_foreign_validator() {
        // TODO: Add test once QuorumCertificate construction is available
    }

    #[test]
    fn test_evidence_fails_for_insufficient_weight() {
        // TODO: Add test once QuorumCertificate construction is available
    }

    #[test]
    fn test_evidence_passes_for_valid_context() {
        // TODO: Add test once QuorumCertificate construction is available
    }
}
