//! Constitutional Identity & Attribution — identity without privilege.
//!
//! Identities may participate in the constitutional derivation fabric,
//! but identity persistence, reputation, or institutional trust must
//! NEVER become constitutional privilege.
//!
//! CRITICAL: A proof from a new participant is constitutionally identical
//! to one from an ancient institution. Identity ≠ authority.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use amun_constitutional::ConstitutionalHasher;

/// An identity surface — identity as operational attribute, NOT semantic privilege.
///
/// Records WHO produced an artifact for operational traceability.
/// Does NOT confer constitutional authority, admissibility preference,
/// or semantic weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentitySurface {
    /// Unique identity identifier.
    pub identity_id: u64,
    /// Cryptographic identifier for this identity.
    pub public_key_hash: ConstitutionalHash,
    /// How long this identity has been active (operational age).
    /// This is an OPERATIONAL metric, not a trust metric.
    pub operational_age: u64,
    /// Number of artifacts produced by this identity.
    /// This is an OPERATIONAL count, not a quality measure.
    pub artifacts_produced: u64,
    /// Whether this identity has EVER been granted constitutional authority.
    /// CRITICAL: This must ALWAYS be false for any compliant implementation.
    pub has_constitutional_authority: bool,
}

impl IdentitySurface {
    pub fn new(identity_id: u64, public_key_hash: ConstitutionalHash) -> Self {
        Self { identity_id, public_key_hash, operational_age: 0, artifacts_produced: 0, has_constitutional_authority: false }
    }

    /// Record artifact production — operational bookkeeping only.
    pub fn record_artifact(&mut self) {
        self.artifacts_produced += 1;
    }

    /// CRITICAL: This method always returns false.
    /// No identity may acquire constitutional authority through
    /// any operational mechanism.
    pub fn check_constitutional_authority(&self) -> bool {
        self.has_constitutional_authority
    }

    /// Compute identity fingerprint — operational, not constitutional.
    pub fn fingerprint(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"IDENTITY_SURFACE");
        h.update_u64(self.identity_id).update_bytes(&self.public_key_hash);
        h.finalize()
    }
}

/// An attribution boundary — who produced what, without who becoming authority.
///
/// Attribution provides operational provenance: "artifact X was produced by Y."
/// It does NOT provide: "artifact X is more valid because Y produced it."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributionBoundary {
    /// The artifact being attributed.
    pub artifact_hash: ConstitutionalHash,
    /// The identity that produced the artifact.
    pub producer_identity_id: u64,
    /// The identity that verified the artifact (if any).
    pub verifier_identity_id: Option<u64>,
    /// Whether this attribution implies ANY semantic weight (ALWAYS false).
    pub implies_semantic_weight: bool,
}

impl AttributionBoundary {
    pub fn new(artifact_hash: ConstitutionalHash, producer_id: u64) -> Self {
        Self { artifact_hash, producer_identity_id: producer_id, verifier_identity_id: None, implies_semantic_weight: false }
    }

    /// CRITICAL: Attribution NEVER implies semantic weight.
    /// Knowing who produced an artifact does not make it more valid.
    pub fn has_semantic_weight(&self) -> bool {
        self.implies_semantic_weight
    }
}

/// A reputation neutrality guard — prevents reputation→legitimacy conversion.
///
/// Reputation is an operational observation: "this participant has been
/// active and responsive." It is NOT a constitutional signal:
/// "this participant produces more valid proofs."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReputationNeutralityGuard {
    /// Participants whose reputation metrics are being tracked.
    pub tracked_identities: Vec<u64>,
    /// Maximum reputation score before neutrality review.
    /// This is a WARNING threshold, not a blocking threshold.
    pub max_neutral_reputation: u64,
    /// Whether any identity has exceeded the neutrality threshold.
    pub neutrality_breach_detected: bool,
}

impl ReputationNeutralityGuard {
    pub fn new(max_reputation: u64) -> Self {
        Self { tracked_identities: Vec::new(), max_neutral_reputation: max_reputation, neutrality_breach_detected: false }
    }

    /// Check if an identity's reputation exceeds the neutrality threshold.
    /// High reputation is an OPERATIONAL observation, not a privilege.
    /// But excessive reputation concentration may indicate capture dynamics.
    pub fn check_reputation(&mut self, identity_id: u64, reputation_score: u64) -> bool {
        if !self.tracked_identities.contains(&identity_id) {
            self.tracked_identities.push(identity_id);
        }
        if reputation_score > self.max_neutral_reputation {
            self.neutrality_breach_detected = true;
            false
        } else {
            true
        }
    }
}

/// An identity containment zone — prevents identity-based capture dynamics.
///
/// Detects when identities accumulate disproportionate operational influence
/// that could drift toward semantic authority. Contains the identity
/// operationally without constitutional invalidation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityContainmentZone {
    /// Identities that have been operationally contained.
    pub contained_identities: Vec<u64>,
    /// Maximum fraction of total artifacts an identity may produce
    /// before containment warning.
    pub max_production_share_percent: u8,
    /// Whether identity-based capture is suspected.
    pub capture_suspected: bool,
}

impl IdentityContainmentZone {
    pub fn new(max_share: u8) -> Self {
        Self { contained_identities: Vec::new(), max_production_share_percent: max_share, capture_suspected: false }
    }

    /// Check if an identity's production share exceeds the threshold.
    pub fn check_production_share(&mut self, identity_id: u64, share_percent: u8) {
        if share_percent > self.max_production_share_percent {
            if !self.contained_identities.contains(&identity_id) {
                self.contained_identities.push(identity_id);
            }
            self.capture_suspected = true;
        }
    }

    /// Returns true if identity-based capture is suspected.
    /// This is a WARNING — not a constitutional invalidation.
    pub fn is_capture_suspected(&self) -> bool { self.capture_suspected }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_never_has_authority() {
        let identity = IdentitySurface::new(1, [0xAA; 32]);
        assert!(!identity.check_constitutional_authority());
    }

    #[test]
    fn test_identity_fingerprint_deterministic() {
        let i1 = IdentitySurface::new(1, [0xAA; 32]);
        let i2 = IdentitySurface::new(1, [0xAA; 32]);
        assert_eq!(i1.fingerprint(), i2.fingerprint());
    }

    #[test]
    fn test_attribution_never_has_semantic_weight() {
        let attr = AttributionBoundary::new([0xBB; 32], 100);
        assert!(!attr.has_semantic_weight());
    }

    #[test]
    fn test_reputation_neutrality() {
        let mut guard = ReputationNeutralityGuard::new(1000);
        assert!(guard.check_reputation(1, 500)); // within bound
        assert!(!guard.check_reputation(2, 1500)); // exceeds bound
        assert!(guard.neutrality_breach_detected);
    }

    #[test]
    fn test_identity_containment() {
        let mut zone = IdentityContainmentZone::new(40);
        zone.check_production_share(100, 45); // exceeds 40%
        assert!(zone.is_capture_suspected());
        assert!(zone.contained_identities.contains(&100));
    }

    #[test]
    fn test_identity_is_not_authority() {
        // No matter how many artifacts, identity never becomes authority
        let mut identity = IdentitySurface::new(1, [0xAA; 32]);
        for _ in 0..1000 { identity.record_artifact(); }
        assert!(!identity.check_constitutional_authority());
        assert_eq!(identity.artifacts_produced, 1000);
    }
}
