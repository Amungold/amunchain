// Constitutional Compatibility Matrix
// Defines the possible compatibility relationships between
// different constitutional identities. This is the foundation
// for Phase 85 migration, fork legitimacy, and evolution mechanics.

use super::constitutional_identity::ConstitutionalIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityLevel {
    /// Full compatibility - same identity hash
    FullyCompatible,
    /// State replay is identical, proofs are identical
    ReplayCompatible,
    /// Snapshots can be imported, but WAL may differ
    SnapshotCompatible,
    /// State can be read but not validated
    ReadOnlyCompatible,
    /// No compatibility - different civilizations
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityMatrix {
    pub local_identity: ConstitutionalIdentity,
    pub remote_identity: ConstitutionalIdentity,
    pub level: CompatibilityLevel,
    pub reasons: Vec<CompatibilityReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityReason {
    IdenticalIdentity,
    SameConstitutionalHash,
    DifferentConstitutionalHash,
    SameEmptyRoot,
    DifferentEmptyRoot,
    SameMaxDepth,
    DifferentMaxDepth,
    SameProofVersion,
    DifferentProofVersion,
    SameProtocolVersion,
    DifferentProtocolVersion,
    SameSnapshotVersion,
    DifferentSnapshotVersion,
    SameCodecVersion,
    DifferentCodecVersion,
}

pub struct CompatibilityEngine;

impl CompatibilityEngine {
    /// Compute the compatibility matrix between two constitutional identities.
    /// This determines what operations are safe between the two civilizations.
    pub fn compute(
        local: &ConstitutionalIdentity,
        remote: &ConstitutionalIdentity,
    ) -> CompatibilityMatrix {
        let mut reasons = Vec::new();

        // Identity check
        if local.identity_hash == remote.identity_hash {
            reasons.push(CompatibilityReason::IdenticalIdentity);
            return CompatibilityMatrix {
                local_identity: local.clone(),
                remote_identity: remote.clone(),
                level: CompatibilityLevel::FullyCompatible,
                reasons,
            };
        }

        // Constitutional foundation checks
        let same_constitution = local.constitutional_hash == remote.constitutional_hash;
        if same_constitution {
            reasons.push(CompatibilityReason::SameConstitutionalHash);
        } else {
            reasons.push(CompatibilityReason::DifferentConstitutionalHash);
        }

        let same_empty_root = local.canonical_empty_root == remote.canonical_empty_root;
        if same_empty_root {
            reasons.push(CompatibilityReason::SameEmptyRoot);
        } else {
            reasons.push(CompatibilityReason::DifferentEmptyRoot);
        }

        let same_max_depth = local.max_depth == remote.max_depth;
        if same_max_depth {
            reasons.push(CompatibilityReason::SameMaxDepth);
        } else {
            reasons.push(CompatibilityReason::DifferentMaxDepth);
        }

        let same_proof = local.proof_version == remote.proof_version;
        if same_proof {
            reasons.push(CompatibilityReason::SameProofVersion);
        } else {
            reasons.push(CompatibilityReason::DifferentProofVersion);
        }

        let same_protocol = local.protocol_version == remote.protocol_version;
        if same_protocol {
            reasons.push(CompatibilityReason::SameProtocolVersion);
        } else {
            reasons.push(CompatibilityReason::DifferentProtocolVersion);
        }

        let same_snapshot = local.snapshot_version == remote.snapshot_version;
        if same_snapshot {
            reasons.push(CompatibilityReason::SameSnapshotVersion);
        } else {
            reasons.push(CompatibilityReason::DifferentSnapshotVersion);
        }

        let same_codec = local.codec_version == remote.codec_version;
        if same_codec {
            reasons.push(CompatibilityReason::SameCodecVersion);
        } else {
            reasons.push(CompatibilityReason::DifferentCodecVersion);
        }

        // Determine compatibility level
        let level = if same_constitution && same_empty_root && same_max_depth && same_proof {
            CompatibilityLevel::ReplayCompatible
        } else if same_constitution && same_empty_root && same_max_depth {
            CompatibilityLevel::SnapshotCompatible
        } else if same_empty_root && same_max_depth {
            CompatibilityLevel::ReadOnlyCompatible
        } else {
            CompatibilityLevel::Incompatible
        };

        CompatibilityMatrix {
            local_identity: local.clone(),
            remote_identity: remote.clone(),
            level,
            reasons,
        }
    }

    /// Can these two identities safely synchronize state?
    pub fn can_sync(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> bool {
        let matrix = Self::compute(local, remote);
        matches!(
            matrix.level,
            CompatibilityLevel::FullyCompatible | CompatibilityLevel::ReplayCompatible
        )
    }

    /// Can state be migrated from remote to local?
    pub fn can_migrate(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> bool {
        let matrix = Self::compute(local, remote);
        matches!(
            matrix.level,
            CompatibilityLevel::FullyCompatible
                | CompatibilityLevel::ReplayCompatible
                | CompatibilityLevel::SnapshotCompatible
        )
    }

    /// Can state be read from remote (without full validation)?
    pub fn can_read(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> bool {
        let matrix = Self::compute(local, remote);
        matches!(
            matrix.level,
            CompatibilityLevel::FullyCompatible
                | CompatibilityLevel::ReplayCompatible
                | CompatibilityLevel::SnapshotCompatible
                | CompatibilityLevel::ReadOnlyCompatible
        )
    }
}
