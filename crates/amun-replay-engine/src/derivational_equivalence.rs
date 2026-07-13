//! Derivational Equivalence Classes — portable admissibility identity.
//!
//! Different derivation surfaces may produce identical constitutional
//! admissibility. This module defines equivalence classes that capture
//! "same admissibility outcome" independent of derivation topology.
//!
//! CRITICAL: Canonicalization is compression, not authority.
//! The canonical form is a transport convenience, not a privileged path.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use amun_constitutional::ConstitutionalHasher;

/// A fingerprint that identifies an admissibility outcome independently
/// of the derivation surface that produced it.
///
/// Two surfaces with the same fingerprint are derivationally equivalent:
/// they prove the same constitutional admissibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalDerivationFingerprint(pub [u8; 32]);

impl CanonicalDerivationFingerprint {
    /// Compute a fingerprint from a target artifact and its admissibility outcome.
    pub fn compute(target_artifact: ConstitutionalHash, can_derive: bool, context_hash: ConstitutionalHash) -> Self {
        let mut h = ConstitutionalHasher::new(b"DERIVATION_FINGERPRINT");
        h.update_bytes(&target_artifact)
            .update_u8(if can_derive { 1 } else { 0 })
            .update_bytes(&context_hash);
        Self(h.finalize())
    }
}

/// A derivational equivalence class: a set of surfaces that prove
/// the same constitutional admissibility for the same target artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationalEquivalenceClass {
    /// The fingerprint identifying this admissibility outcome.
    pub fingerprint: CanonicalDerivationFingerprint,

    /// The target artifact whose admissibility is being proven.
    pub target_artifact_hash: ConstitutionalHash,

    /// The context this equivalence class belongs to.
    pub context_hash: ConstitutionalHash,

    /// Whether this class represents "admissibility derivable".
    pub is_admissible: bool,

    /// Number of known surfaces in this equivalence class.
    pub surface_count: u64,

    /// The minimal derivation core — the smallest known surface
    /// that still proves the same admissibility.
    pub minimal_core_hashes: Vec<ConstitutionalHash>,

    /// Hash of this equivalence class for transport.
    pub class_hash: [u8; 32],
}

impl DerivationalEquivalenceClass {
    pub fn new(
        target_artifact_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        is_admissible: bool,
        minimal_core_hashes: Vec<ConstitutionalHash>,
    ) -> Self {
        let fingerprint = CanonicalDerivationFingerprint::compute(
            target_artifact_hash, is_admissible, context_hash,
        );
        let mut c = Self {
            fingerprint,
            target_artifact_hash,
            context_hash,
            is_admissible,
            surface_count: 1,
            minimal_core_hashes,
            class_hash: [0; 32],
        };
        c.class_hash = c.compute_class_hash();
        c
    }

    fn compute_class_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(b"EQUIVALENCE_CLASS");
        h.update_bytes(&self.fingerprint.0)
            .update_bytes(&self.target_artifact_hash)
            .update_bytes(&self.context_hash)
            .update_u8(if self.is_admissible { 1 } else { 0 })
            .update_u64(self.surface_count);
        for core_hash in &self.minimal_core_hashes {
            h.update_bytes(core_hash);
        }
        h.finalize()
    }

    /// Add another surface to this equivalence class.
    pub fn add_surface(&mut self, core_hashes: Vec<ConstitutionalHash>) {
        self.surface_count += 1;
        // Keep the smaller minimal core
        if core_hashes.len() < self.minimal_core_hashes.len() || self.minimal_core_hashes.is_empty() {
            self.minimal_core_hashes = core_hashes;
        }
        self.class_hash = self.compute_class_hash();
    }

    /// Returns true if two equivalence classes represent the same admissibility.
    pub fn is_equivalent_to(&self, other: &DerivationalEquivalenceClass) -> bool {
        self.fingerprint == other.fingerprint
            && self.target_artifact_hash == other.target_artifact_hash
            && self.is_admissible == other.is_admissible
    }
}

/// Normalize a set of artifact hashes into canonical order.
/// This is for transport efficiency only — canonical order is not privileged.
pub fn canonical_order_for_transport(hashes: &mut [ConstitutionalHash]) {
    hashes.sort();
}

/// Reduce a proof surface to its minimal derivation core.
/// Keeps only hard dependency hashes that are strictly necessary.
pub fn reduce_to_minimal_core(
    all_hashes: &[ConstitutionalHash],
    hard_dependency_hashes: &[ConstitutionalHash],
) -> Vec<ConstitutionalHash> {
    let mut core: Vec<ConstitutionalHash> = Vec::new();
    for hash in hard_dependency_hashes {
        if all_hashes.contains(hash) && !core.contains(hash) {
            core.push(*hash);
        }
    }
    canonical_order_for_transport(&mut core);
    core
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_deterministic() {
        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_different_outcome_different_fingerprint() {
        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], false, [0xAB; 32]);
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn test_equivalence_class_creation() {
        let class = DerivationalEquivalenceClass::new(
            [0xAA; 32], [0xAB; 32], true,
            vec![[0x01; 32], [0x02; 32]],
        );
        assert!(class.is_admissible);
        assert_eq!(class.surface_count, 1);
        assert_eq!(class.minimal_core_hashes.len(), 2);
    }

    #[test]
    fn test_add_surface_keeps_smaller_core() {
        let mut class = DerivationalEquivalenceClass::new(
            [0xAA; 32], [0xAB; 32], true,
            vec![[0x01; 32], [0x02; 32], [0x03; 32]],
        );
        class.add_surface(vec![[0x01; 32], [0x02; 32]]);
        assert_eq!(class.surface_count, 2);
        assert_eq!(class.minimal_core_hashes.len(), 2); // smaller core kept
    }

    #[test]
    fn test_reduce_to_minimal_core() {
        let all = vec![[0x01; 32], [0x02; 32], [0x03; 32], [0x04; 32]];
        let hard = vec![[0x02; 32], [0x04; 32]];
        let core = reduce_to_minimal_core(&all, &hard);
        assert_eq!(core.len(), 2);
        assert!(core.contains(&[0x02; 32]));
        assert!(core.contains(&[0x04; 32]));
        // Canonical order: sorted
        assert!(core[0] < core[1]);
    }

    #[test]
    fn test_equivalence_detection() {
        let c1 = DerivationalEquivalenceClass::new([0xAA; 32], [0xAB; 32], true, vec![[0x01; 32]]);
        let c2 = DerivationalEquivalenceClass::new([0xAA; 32], [0xAB; 32], true, vec![[0x02; 32]]);
        assert!(c1.is_equivalent_to(&c2));
    }

    #[test]
    fn test_non_equivalence() {
        let c1 = DerivationalEquivalenceClass::new([0xAA; 32], [0xAB; 32], true, vec![[0x01; 32]]);
        let c2 = DerivationalEquivalenceClass::new([0xAA; 32], [0xAB; 32], false, vec![[0x01; 32]]);
        assert!(!c1.is_equivalent_to(&c2));
    }
}
