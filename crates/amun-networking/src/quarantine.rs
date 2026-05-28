use amun_snapshot_engine::ConstitutionalIdentity;

/// Constitutional Quarantine Zone.
/// Foreign snapshots never enter runtime directly - they are isolated
/// in a verification space similar to biosafety containment levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuarantineLevel {
    /// No quarantine needed - compatible civilization
    None,
    /// Observation only - no state integration
    Observation,
    /// Isolated verification - snapshots verified but not applied
    IsolatedVerification,
    /// Full quarantine - no interaction permitted
    FullQuarantine,
}

/// A quarantined snapshot from a foreign or untrusted source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantinedSnapshot {
    pub source_identity: ConstitutionalIdentity,
    pub snapshot_root: [u8; 32],
    pub quarantine_level: QuarantineLevel,
    pub quarantine_reason: String,
    pub verification_status: QuarantineVerificationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuarantineVerificationStatus {
    Pending,
    PhysicsVerified,
    ReplayVerified,
    LineageVerified,
    Released { authorized_by: [u8; 32] },
    Rejected { reason: String },
    PermanentlyQuarantined,
}

/// Constitutional Quarantine Engine.
pub struct QuarantineEngine;

impl QuarantineEngine {
    pub fn classify_snapshot(
        local: &ConstitutionalIdentity,
        remote: &ConstitutionalIdentity,
    ) -> QuarantineLevel {
        if local.matches(remote) {
            return QuarantineLevel::None;
        }

        if local.canonical_empty_root == remote.canonical_empty_root
            && local.max_depth == remote.max_depth
        {
            return QuarantineLevel::IsolatedVerification;
        }

        if local.canonical_empty_root == remote.canonical_empty_root {
            return QuarantineLevel::Observation;
        }

        QuarantineLevel::FullQuarantine
    }
}
