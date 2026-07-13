//! ExecutionReceipt — terminal constitutional witness.
//!
//! ARCHITECTURAL CONSTRAINTS:
//!   1. Receipt is TERMINAL — it NEVER participates in execution.
//!   2. Receipt contains NO execution payload — only hashes, references, positions.
//!   3. Receipt validity MUST be derivable entirely from prior artifacts.
//!   4. Receipt is immutable after creation.
//!   5. receipt_sequence is PER-CONTEXT monotonic, NOT global.
//!      This preserves replay locality and prevents global sequencing bottlenecks.
//!   6. parent_receipt_hash is AUDIT TOPOLOGY ONLY.
//!      Receipt validity does NOT depend on receipt ancestry.
//!      Ancestry is for auditability, not authorization.
//!
//! CERTIFICATE SEMANTICS (documented for PHASE 52/53):
//!   The certificate_hash links to a ReplayCertificate that attests
//!   replay ADMISSIBILITY, not execution finality.
//!   This is an admissibility-first certificate model.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains::DOMAIN_EXECUTION_RECEIPT;
use crate::kernel_types::ConstitutionalHash;
use crate::replay_outcome::ReplayOutcome;

/// A terminal constitutional witness attesting to replay admissibility.
///
/// # Per-Context Sequencing
/// `receipt_sequence` is monotonic within a single context (same `context_hash`).
/// It is NOT a global sequence number. Two receipts in different contexts
/// may have the same `receipt_sequence` — they are in different lineages.
///
/// # Parent Lineage (Audit Only)
/// `parent_receipt_hash` provides audit topology — it links receipts
/// within the same context for traceability. Receipt validity does NOT
/// depend on parent receipt validity. A receipt is valid if its own
/// constitutional fields are valid, regardless of its ancestry.
///
/// # Certificate Semantics
/// `certificate_hash` links to a ReplayCertificate. The certificate model
/// is ADMISSIBILITY-FIRST: it attests that the execution was constitutionally
/// admissible, not that it achieved finality. Finality is a consensus concern
/// layered above the constitutional kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub receipt_id: u64,
    pub receipt_hash: ConstitutionalHash,

    /// Transcript position at time of receipt creation.
    pub transcript_position: u64,

    /// Per-context monotonic sequence number.
    /// Unique within the scope of `context_hash`.
    pub receipt_sequence: u64,

    /// Context this receipt belongs to.
    pub context_hash: ConstitutionalHash,

    /// Active boundary at time of receipt.
    pub boundary_hash: ConstitutionalHash,

    /// Commitment this receipt witnesses.
    pub commitment_hash: ConstitutionalHash,

    /// Certificate attesting replay admissibility (admissibility-first model).
    pub certificate_hash: ConstitutionalHash,

    /// Previous receipt in this context (AUDIT TOPOLOGY ONLY).
    /// Does NOT affect receipt validity.
    pub parent_receipt_hash: Option<ConstitutionalHash>,

    /// Constitutional admissibility outcome.
    pub replay_outcome: ReplayOutcome,

    /// Reference to a ConstitutionalFailure if outcome is failure.
    /// INFORMATIONAL ONLY (CIR-001) — not in constitutional hash.
    pub failure_reference: Option<ConstitutionalHash>,
}

impl ConstitutionalIdentity for ExecutionReceipt {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for ExecutionReceipt {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(DOMAIN_EXECUTION_RECEIPT);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.receipt_id)
            .update_u64(self.transcript_position)
            .update_u64(self.receipt_sequence)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.boundary_hash)
            .update_bytes(&self.commitment_hash)
            .update_bytes(&self.certificate_hash)
            .update_optional_hash(self.parent_receipt_hash.as_ref())
            .update_u8(self.replay_outcome as u8);
        // failure_reference is INFORMATIONAL ONLY (CIR-001)
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000C || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.receipt_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid receipt schema",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.receipt_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.receipt_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Receipt hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.receipt_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage for receipt",
            ));
        }
        Ok(())
    }

    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        // NOTE: We do NOT verify parent_receipt_hash validity.
        // Parent lineage is audit topology only.
        // A receipt with a broken parent link is still constitutionally valid;
        // the break is an audit concern, not a validity concern.
        Ok(())
    }
}

impl ExecutionReceipt {
    pub fn new(
        receipt_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        transcript_position: u64,
        receipt_sequence: u64,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        commitment_hash: ConstitutionalHash,
        certificate_hash: ConstitutionalHash,
        parent_receipt_hash: Option<ConstitutionalHash>,
        replay_outcome: ReplayOutcome,
        failure_reference: Option<ConstitutionalHash>,
    ) -> Self {
        let mut r = Self {
            schema_id: 0x000C,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            receipt_id,
            receipt_hash: [0; 32],
            transcript_position,
            receipt_sequence,
            context_hash,
            boundary_hash,
            commitment_hash,
            certificate_hash,
            parent_receipt_hash,
            replay_outcome,
            failure_reference,
        };
        r.receipt_hash = r.constitutional_hash();
        r
    }

    pub fn is_admitted(&self) -> bool {
        self.replay_outcome.is_admitted()
    }
    pub fn is_failure(&self) -> bool {
        self.replay_outcome.is_failure()
    }
    pub fn has_parent(&self) -> bool {
        self.parent_receipt_hash.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_receipt(
        id: u64,
        seq: u64,
        context: [u8; 32],
        parent: Option<ConstitutionalHash>,
    ) -> ExecutionReceipt {
        ExecutionReceipt::new(
            id,
            1,
            1,
            seq * 100,
            seq,
            context,
            [0xBC; 32],
            [0xCD; 32],
            [0xDE; 32],
            parent,
            ReplayOutcome::Admitted,
            None,
        )
    }

    #[test]
    fn test_admitted_verifies() {
        assert!(make_receipt(1, 0, [0xAB; 32], None).verify().is_ok());
    }
    #[test]
    fn test_hash_deterministic() {
        assert_eq!(
            make_receipt(1, 0, [0xAB; 32], None).receipt_hash,
            make_receipt(1, 0, [0xAB; 32], None).receipt_hash
        );
    }
    #[test]
    fn test_parent_does_not_affect_validity() {
        let r = make_receipt(1, 1, [0xAB; 32], Some([0xDE; 32]));
        assert!(r.verify().is_ok());
        assert!(r.has_parent());
    }
    #[test]
    fn test_different_contexts_can_share_sequence() {
        let r1 = make_receipt(1, 0, [0xAA; 32], None);
        let r2 = make_receipt(1, 0, [0xBB; 32], None);
        assert_ne!(r1.context_hash, r2.context_hash);
        assert_eq!(r1.receipt_sequence, r2.receipt_sequence);
    }
    #[test]
    fn test_failure_reference_not_in_hash() {
        let r1 = ExecutionReceipt::new(
            1,
            1,
            1,
            0,
            0,
            [0xAB; 32],
            [0xBC; 32],
            [0xCD; 32],
            [0xDE; 32],
            None,
            ReplayOutcome::ConstitutionalFailure,
            Some([0x11; 32]),
        );
        let r2 = ExecutionReceipt::new(
            1,
            1,
            1,
            0,
            0,
            [0xAB; 32],
            [0xBC; 32],
            [0xCD; 32],
            [0xDE; 32],
            None,
            ReplayOutcome::ConstitutionalFailure,
            Some([0x22; 32]),
        );
        assert_eq!(r1.receipt_hash, r2.receipt_hash);
    }
}
