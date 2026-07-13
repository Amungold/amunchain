//! WitnessEnvelope — transport container for witness propagation.
//!
//! CRITICAL: The envelope is a TRANSPORT ARTIFACT only.
//! It does NOT carry constitutional authority, validity judgments,
//! or truth status. The constitutional kernel judges the witness
//! AFTER it arrives, not the envelope.
//!
//! The envelope answers: "Here is a proof surface for you to verify."
//! It does NOT answer: "This proof surface is constitutionally valid."

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::operational_hasher::OperationalHasher;

/// A transport envelope for a constitutional witness.
///
/// The envelope wraps a witness for network propagation.
/// The witness itself remains a constitutional artifact.
/// The envelope adds operational metadata for routing and filtering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessEnvelope {
    /// Unique envelope identifier (operational).
    pub envelope_id: u64,

    /// Hash of the witness being transported.
    pub witness_hash: ConstitutionalHash,

    /// The worker that produced this witness (operational provenance).
    pub source_worker_id: u64,

    /// The context this witness belongs to.
    pub context_hash: ConstitutionalHash,

    /// The target artifact this witness proves admissibility for.
    pub target_artifact_hash: ConstitutionalHash,

    /// Propagation scope — how far this witness should propagate.
    pub propagation_scope: PropagationScope,

    /// Envelope hash for operational traceability.
    pub envelope_hash: [u8; 32],

    /// Informational note (CIR-001).
    pub propagation_note: Option<Vec<u8>>,
}

/// How far a witness should propagate through the network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationScope {
    /// Only workers in the same context need this witness.
    ContextLocal,
    /// All workers in the same boundary need this witness.
    BoundaryScoped,
    /// Any worker that depends on the target artifact needs this witness.
    DependencyScoped,
    /// Broadcast to all reachable workers (use sparingly).
    FullBroadcast,
}

impl WitnessEnvelope {
    pub fn new(
        envelope_id: u64,
        witness_hash: ConstitutionalHash,
        source_worker_id: u64,
        context_hash: ConstitutionalHash,
        target_artifact_hash: ConstitutionalHash,
        propagation_scope: PropagationScope,
    ) -> Self {
        let mut e = Self {
            envelope_id,
            witness_hash,
            source_worker_id,
            context_hash,
            target_artifact_hash,
            propagation_scope,
            envelope_hash: [0; 32],
            propagation_note: None,
        };
        e.envelope_hash = e.compute_hash();
        e
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = OperationalHasher::new(b"WITNESS_ENVELOPE");
        h.update_u64(self.envelope_id)
            .update_bytes(&self.witness_hash)
            .update_u64(self.source_worker_id)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.target_artifact_hash)
            .update_u8(self.propagation_scope as u8);
        h.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_creation() {
        let env = WitnessEnvelope::new(
            1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32],
            PropagationScope::ContextLocal,
        );
        assert_eq!(env.envelope_id, 1);
        assert_eq!(env.source_worker_id, 100);
    }

    #[test]
    fn test_envelope_hash_deterministic() {
        let e1 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
        let e2 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
        assert_eq!(e1.envelope_hash, e2.envelope_hash);
    }

    #[test]
    fn test_envelope_is_not_witness() {
        // Different propagation scopes = different envelopes
        let e1 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
        let e2 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::FullBroadcast);
        assert_ne!(e1.envelope_hash, e2.envelope_hash);
        // Same witness hash though — the envelope changes, not the witness
        assert_eq!(e1.witness_hash, e2.witness_hash);
    }
}
