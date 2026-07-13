//! Cross-Constitution Federation — interoperability without imperialism.
//!
//! Federation enables derivational exchange between constitutions
//! without establishing any constitution as canonical authority.
//!
//! CRITICAL: Federation ≠ unification. Interoperability ≠ supremacy.
//! Bridging transports derivations, not sovereignty.

use amun_constitutional::kernel_types::ConstitutionalHash;
use amun_constitutional::ConstitutionalHasher;

/// A federation boundary — constitutional sovereignty firewall.
///
/// Prevents any external constitution from overriding local invariants,
/// importing governance authority, or mutating constitutional semantics.
/// Allows only: derivational exchange, inspectable translation,
/// bounded interoperability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FederationBoundary {
    /// Unique boundary identifier.
    pub boundary_id: u64,
    /// Hash of the local constitution this boundary protects.
    pub local_constitution_hash: ConstitutionalHash,
    /// Hash of the remote constitution this boundary connects to.
    pub remote_constitution_hash: ConstitutionalHash,
    /// Whether the remote constitution may influence local admissibility.
    /// CRITICAL: This must ALWAYS be false for any compliant implementation.
    pub allows_remote_admissibility_override: bool,
    /// Whether the remote constitution may modify local invariants.
    /// CRITICAL: This must ALWAYS be false.
    pub allows_remote_invariant_mutation: bool,
    /// Whether this boundary is active.
    pub active: bool,
}

impl FederationBoundary {
    pub fn new(
        boundary_id: u64, local_hash: ConstitutionalHash, remote_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            boundary_id, local_constitution_hash: local_hash, remote_constitution_hash: remote_hash,
            allows_remote_admissibility_override: false,
            allows_remote_invariant_mutation: false,
            active: true,
        }
    }

    /// CRITICAL: Remote admissibility override is NEVER allowed.
    /// The local constitution always governs its own admissibility.
    pub fn check_remote_override_allowed(&self) -> bool {
        self.allows_remote_admissibility_override
    }

    /// CRITICAL: Remote invariant mutation is NEVER allowed.
    /// Each constitution is sovereign over its own invariants.
    pub fn check_remote_mutation_allowed(&self) -> bool {
        self.allows_remote_invariant_mutation
    }
}

/// A constitutional translation surface — interprets derivations
/// between constitutions without semantic replacement.
///
/// Translation is interpretation, not authority. When a derivation
/// crosses a federation boundary, the translation surface records
/// what was received, how it was interpreted, and preserves the
/// source constitution's sovereignty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalTranslationSurface {
    /// Unique translation identifier.
    pub translation_id: u64,
    /// The source constitution this derivation originated from.
    pub source_constitution_hash: ConstitutionalHash,
    /// The target constitution receiving the translation.
    pub target_constitution_hash: ConstitutionalHash,
    /// The derivation hash in the source constitution.
    pub source_derivation_hash: ConstitutionalHash,
    /// The interpreted derivation hash in the target constitution.
    pub interpreted_derivation_hash: ConstitutionalHash,
    /// Whether this translation preserves sovereignty attribution.
    pub sovereignty_preserved: bool,
    /// Whether the target claims authority over the source (MUST be false).
    pub target_claims_source_authority: bool,
}

impl ConstitutionalTranslationSurface {
    pub fn new(
        translation_id: u64, source_hash: ConstitutionalHash,
        target_hash: ConstitutionalHash, source_derivation: ConstitutionalHash,
        interpreted_derivation: ConstitutionalHash,
    ) -> Self {
        Self {
            translation_id, source_constitution_hash: source_hash,
            target_constitution_hash: target_hash,
            source_derivation_hash: source_derivation,
            interpreted_derivation_hash: interpreted_derivation,
            sovereignty_preserved: true,
            target_claims_source_authority: false,
        }
    }

    /// CRITICAL: The target never claims authority over the source.
    /// Each constitution is sovereign.
    pub fn verify_sovereignty(&self) -> bool {
        self.sovereignty_preserved && !self.target_claims_source_authority
    }

    /// Compute translation fingerprint.
    pub fn fingerprint(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"TRANSLATION_SURFACE");
        h.update_u64(self.translation_id)
            .update_bytes(&self.source_constitution_hash)
            .update_bytes(&self.target_constitution_hash)
            .update_bytes(&self.source_derivation_hash)
            .update_bytes(&self.interpreted_derivation_hash);
        h.finalize()
    }
}

/// A sovereignty-preserving bridge — bridges without semantic equivalence.
///
/// Bridges transport derivations between constitutions without implying
/// that the constitutions are semantically equivalent. Federation is
/// coexistence, not assimilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereigntyPreservingBridge {
    /// Unique bridge identifier.
    pub bridge_id: u64,
    /// The federation boundary governing this bridge.
    pub boundary: FederationBoundary,
    /// Whether this bridge enforces sovereignty preservation.
    pub sovereignty_enforced: bool,
    /// Whether this bridge treats constitutions as equivalent (MUST be false).
    pub assumes_constitutional_equivalence: bool,
    /// Number of successful translations through this bridge.
    pub translation_count: u64,
}

impl SovereigntyPreservingBridge {
    pub fn new(bridge_id: u64, boundary: FederationBoundary) -> Self {
        Self {
            bridge_id, boundary, sovereignty_enforced: true,
            assumes_constitutional_equivalence: false, translation_count: 0,
        }
    }

    /// CRITICAL: The bridge does NOT assume constitutional equivalence.
    /// Two constitutions may interoperate without being the same.
    pub fn check_equivalence_assumed(&self) -> bool {
        self.assumes_constitutional_equivalence
    }

    /// Record a successful translation.
    pub fn record_translation(&mut self) {
        self.translation_count += 1;
    }
}

/// Federation status for a constitution pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FederationStatus {
    /// Federations are actively exchanging derivations.
    Active,
    /// Federation is suspended (operational, not constitutional).
    Suspended,
    /// No federation relationship exists.
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_never_allows_override() {
        let boundary = FederationBoundary::new(1, [0xAA; 32], [0xBB; 32]);
        assert!(!boundary.check_remote_override_allowed());
        assert!(!boundary.check_remote_mutation_allowed());
    }

    #[test]
    fn test_translation_preserves_sovereignty() {
        let translation = ConstitutionalTranslationSurface::new(
            1, [0xAA; 32], [0xBB; 32], [0xCC; 32], [0xDD; 32],
        );
        assert!(translation.verify_sovereignty());
        assert!(!translation.target_claims_source_authority);
    }

    #[test]
    fn test_bridge_never_assumes_equivalence() {
        let boundary = FederationBoundary::new(1, [0xAA; 32], [0xBB; 32]);
        let bridge = SovereigntyPreservingBridge::new(1, boundary);
        assert!(!bridge.check_equivalence_assumed());
        assert!(bridge.sovereignty_enforced);
    }

    #[test]
    fn test_federation_is_not_unification() {
        // Two different constitutions can federate without becoming one.
        let boundary = FederationBoundary::new(1, [0xAA; 32], [0xBB; 32]);
        assert_ne!(boundary.local_constitution_hash, boundary.remote_constitution_hash);
        // They remain distinct — federation ≠ unification
    }

    #[test]
    fn test_translation_fingerprint_deterministic() {
        let t1 = ConstitutionalTranslationSurface::new(1, [0xAA;32], [0xBB;32], [0xCC;32], [0xDD;32]);
        let t2 = ConstitutionalTranslationSurface::new(1, [0xAA;32], [0xBB;32], [0xCC;32], [0xDD;32]);
        assert_eq!(t1.fingerprint(), t2.fingerprint());
    }

    #[test]
    fn test_bridge_records_translations() {
        let boundary = FederationBoundary::new(1, [0xAA; 32], [0xBB; 32]);
        let mut bridge = SovereigntyPreservingBridge::new(1, boundary);
        bridge.record_translation();
        bridge.record_translation();
        assert_eq!(bridge.translation_count, 2);
    }
}
