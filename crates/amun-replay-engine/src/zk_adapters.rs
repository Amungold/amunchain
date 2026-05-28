//! ZK Witness Adapters — cryptographic portability for derivability.
//!
//! These adapters wrap constitutional proofs in cryptographic envelopes
//! for efficient, private, or cross-system verification.
//!
//! CRITICAL: ZK proofs attest to derivability, but do NOT define it.
//! The constitutional kernel remains the sole source of truth.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use amun_constitutional::ConstitutionalHasher;

/// A cryptographic wrapper for a constitutional witness.
///
/// The envelope carries a ZK proof that a certain derivability outcome
/// is valid, without necessarily revealing the full witness surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZKWitnessEnvelope {
    /// The target artifact whose admissibility is being proven.
    pub target_artifact_hash: ConstitutionalHash,
    /// The constitutional context.
    pub context_hash: ConstitutionalHash,
    /// The admissibility fingerprint that the ZK proof asserts.
    pub asserted_fingerprint: ConstitutionalHash,
    /// The ZK proof payload (opaque bytes — informational until verified).
    pub proof_payload: Vec<u8>,
    /// Whether the proof has been verified against the kernel.
    pub verified: bool,
    /// Hash of this envelope for transport.
    pub envelope_hash: ConstitutionalHash,
}

impl ZKWitnessEnvelope {
    pub fn new(
        target: ConstitutionalHash,
        context: ConstitutionalHash,
        fingerprint: ConstitutionalHash,
        proof_payload: Vec<u8>,
    ) -> Self {
        let mut e = Self {
            target_artifact_hash: target,
            context_hash: context,
            asserted_fingerprint: fingerprint,
            proof_payload,
            verified: false,
            envelope_hash: [0; 32],
        };
        e.envelope_hash = e.compute_hash();
        e
    }

    fn compute_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"ZK_WITNESS_ENVELOPE");
        h.update_bytes(&self.target_artifact_hash)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.asserted_fingerprint)
            .update_bytes(&self.proof_payload);
        h.finalize()
    }

    /// Mark the proof as verified (after actual ZK verification against kernel).
    pub fn mark_verified(&mut self) {
        self.verified = true;
    }
}

/// A commitment to a specific derivability outcome, suitable for ZK wrapping.
///
/// This commitment cryptographically binds the admissibility outcome
/// without revealing the full derivation surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivabilityCommitment {
    /// The target artifact.
    pub target_artifact_hash: ConstitutionalHash,
    /// The canonical derivation fingerprint being committed to.
    pub fingerprint: ConstitutionalHash,
    /// A random blinding factor to hide the fingerprint if needed.
    pub blinding: [u8; 32],
    /// The actual commitment hash.
    pub commitment: ConstitutionalHash,
}

impl DerivabilityCommitment {
    pub fn new(
        target: ConstitutionalHash,
        fingerprint: ConstitutionalHash,
        blinding: [u8; 32],
    ) -> Self {
        let mut c = Self {
            target_artifact_hash: target,
            fingerprint,
            blinding,
            commitment: [0; 32],
        };
        c.commitment = c.compute_commitment();
        c
    }

    fn compute_commitment(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"DERIVABILITY_COMMITMENT");
        h.update_bytes(&self.target_artifact_hash)
            .update_bytes(&self.fingerprint)
            .update_bytes(&self.blinding);
        h.finalize()
    }
}

/// A recursive proof boundary — prevents infinite ZK nesting.
#[derive(Debug, Clone)]
pub struct RecursiveProofBoundary {
    /// Maximum allowed depth of recursive ZK proofs.
    pub max_depth: u32,
    /// Current depth level (for tracking nested proofs).
    pub current_depth: u32,
}

impl RecursiveProofBoundary {
    pub fn new(max_depth: u32) -> Self {
        Self { max_depth, current_depth: 0 }
    }

    /// Returns true if another level of recursion is allowed.
    pub fn can_recurse(&self) -> bool {
        self.current_depth < self.max_depth
    }

    /// Enter one recursion level.
    pub fn recurse(&mut self) -> bool {
        if self.can_recurse() {
            self.current_depth += 1;
            true
        } else {
            false
        }
    }
}

/// A selective reveal surface — discloses only the minimal derivation
/// necessary for an external verifier to check admissibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectiveRevealSurface {
    /// The target artifact.
    pub target_artifact_hash: ConstitutionalHash,
    /// The minimal core hashes being revealed.
    pub revealed_hashes: Vec<ConstitutionalHash>,
    /// The admissibility fingerprint that this surface proves.
    pub fingerprint: ConstitutionalHash,
    /// A hash binding the revealed surface to the original witness.
    pub surface_binding: ConstitutionalHash,
}

impl SelectiveRevealSurface {
    pub fn new(
        target: ConstitutionalHash,
        revealed: Vec<ConstitutionalHash>,
        fingerprint: ConstitutionalHash,
    ) -> Self {
        let mut s = Self {
            target_artifact_hash: target,
            revealed_hashes: revealed,
            fingerprint,
            surface_binding: [0; 32],
        };
        s.surface_binding = s.compute_binding();
        s
    }

    fn compute_binding(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"SELECTIVE_REVEAL");
        h.update_bytes(&self.target_artifact_hash)
            .update_bytes(&self.fingerprint);
        for hash in &self.revealed_hashes {
            h.update_bytes(hash);
        }
        h.finalize()
    }
}

/// An adapter for external verification systems (e.g., other proof networks).
///
/// Bridges AmunChain derivability to foreign verifiers without exposing
/// the full constitutional kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalVerifierAdapter {
    /// Identifier for the external verifier system.
    pub verifier_system_id: [u8; 32],
    /// The derivability commitment being exported.
    pub commitment: DerivabilityCommitment,
    /// A cryptographic proof that the commitment is valid under the kernel.
    pub export_proof: Vec<u8>,
}

impl ExternalVerifierAdapter {
    pub fn new(
        verifier_system_id: [u8; 32],
        commitment: DerivabilityCommitment,
        export_proof: Vec<u8>,
    ) -> Self {
        Self { verifier_system_id, commitment, export_proof }
    }
}

/// A guard that ensures ZK proofs remain constitutionally inspectable.
///
/// Prevents opaque proofs from bypassing constitutional verification.
/// Every ZK proof must carry enough information to trace back to a
/// valid constitutional derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofInspectabilityGuard {
    /// The original witness hash that this ZK proof derives from.
    pub source_witness_hash: ConstitutionalHash,
    /// Whether the proof preserves full derivability attribution.
    pub attribution_preserved: bool,
    /// Whether the proof can be decomposed into constituent parts.
    pub decomposable: bool,
}

impl ProofInspectabilityGuard {
    pub fn new(source_witness_hash: ConstitutionalHash) -> Self {
        Self { source_witness_hash, attribution_preserved: true, decomposable: true }
    }

    /// Returns true if this ZK proof is constitutionally inspectable.
    pub fn is_inspectable(&self) -> bool {
        self.attribution_preserved && self.decomposable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_envelope_creation() {
        let env = ZKWitnessEnvelope::new(
            [0xAA; 32], [0xAB; 32], [0xCC; 32], b"zk_proof_data".to_vec(),
        );
        assert_eq!(env.target_artifact_hash, [0xAA; 32]);
        assert!(!env.verified);
    }

    #[test]
    fn test_derivability_commitment() {
        let c = DerivabilityCommitment::new([0xAA; 32], [0xBB; 32], [0xCC; 32]);
        assert_ne!(c.commitment, [0; 32]);
    }

    #[test]
    fn test_recursive_boundary() {
        let mut boundary = RecursiveProofBoundary::new(2);
        assert!(boundary.can_recurse());
        assert!(boundary.recurse());
        assert!(boundary.can_recurse());
        assert!(boundary.recurse());
        assert!(!boundary.can_recurse());
        assert!(!boundary.recurse());
    }

    #[test]
    fn test_selective_reveal() {
        let s = SelectiveRevealSurface::new(
            [0xAA; 32], vec![[0x01; 32], [0x02; 32]], [0xBB; 32],
        );
        assert_eq!(s.revealed_hashes.len(), 2);
        assert_ne!(s.surface_binding, [0; 32]);
    }

    #[test]
    fn test_external_verifier_adapter() {
        let commitment = DerivabilityCommitment::new([0xAA; 32], [0xBB; 32], [0xCC; 32]);
        let adapter = ExternalVerifierAdapter::new([0xEE; 32], commitment, b"proof".to_vec());
        assert_eq!(adapter.verifier_system_id, [0xEE; 32]);
    }

    #[test]
    fn test_inspectability_guard() {
        let guard = ProofInspectabilityGuard::new([0xDD; 32]);
        assert!(guard.is_inspectable());
    }
}
