use amun_canonical_codec::CanonicalHasher;

/// Formal constitutional delta types.
/// Each delta represents a specific semantic change to the constitution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalDelta {
    GovernanceDelta {
        old_quorum: u64,
        new_quorum: u64,
    },
    ReplayDelta {
        old_guarantee_rank: u8,
        new_guarantee_rank: u8,
    },
    ProofSystemDelta {
        old_proof_version: u8,
        new_proof_version: u8,
    },
    FreezeBoundaryDelta {
        field_name: String,
        old_mutability: u8,
        new_mutability: u8,
    },
    IdentityDelta {
        old_constitution_hash: [u8; 32],
        new_constitution_hash: [u8; 32],
    },
    CompatibilityDelta {
        old_class_rank: u8,
        new_class_rank: u8,
    },
    SnapshotFormatDelta {
        old_snapshot_version: u32,
        new_snapshot_version: u32,
    },
    Noop,
}

impl ConstitutionalDelta {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            Self::GovernanceDelta { .. } => 0x01,
            Self::ReplayDelta { .. } => 0x02,
            Self::ProofSystemDelta { .. } => 0x03,
            Self::FreezeBoundaryDelta { .. } => 0x04,
            Self::IdentityDelta { .. } => 0x05,
            Self::CompatibilityDelta { .. } => 0x06,
            Self::SnapshotFormatDelta { .. } => 0x07,
            Self::Noop => 0x00,
        }
    }

    pub fn impact_level(&self) -> u8 {
        match self {
            Self::GovernanceDelta { .. } => 1,
            Self::CompatibilityDelta { .. } => 1,
            Self::ReplayDelta { .. } => 3,
            Self::ProofSystemDelta { .. } => 3,
            Self::FreezeBoundaryDelta { .. } => 3,
            Self::IdentityDelta { .. } => 3,
            Self::SnapshotFormatDelta { .. } => 2,
            Self::Noop => 0,
        }
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_DELTA_V1");
        h.update(&[self.canonical_tag()]);
        match self {
            Self::GovernanceDelta {
                old_quorum,
                new_quorum,
            } => {
                h.update(&old_quorum.to_le_bytes());
                h.update(&new_quorum.to_le_bytes());
            }
            Self::ReplayDelta {
                old_guarantee_rank,
                new_guarantee_rank,
            } => {
                h.update(&[*old_guarantee_rank]);
                h.update(&[*new_guarantee_rank]);
            }
            Self::ProofSystemDelta {
                old_proof_version,
                new_proof_version,
            } => {
                h.update(&[*old_proof_version]);
                h.update(&[*new_proof_version]);
            }
            Self::FreezeBoundaryDelta {
                field_name,
                old_mutability,
                new_mutability,
            } => {
                h.update(field_name.as_bytes());
                h.update(&[*old_mutability]);
                h.update(&[*new_mutability]);
            }
            Self::IdentityDelta {
                old_constitution_hash,
                new_constitution_hash,
            } => {
                h.update(old_constitution_hash);
                h.update(new_constitution_hash);
            }
            Self::CompatibilityDelta {
                old_class_rank,
                new_class_rank,
            } => {
                h.update(&[*old_class_rank]);
                h.update(&[*new_class_rank]);
            }
            Self::SnapshotFormatDelta {
                old_snapshot_version,
                new_snapshot_version,
            } => {
                h.update(&old_snapshot_version.to_le_bytes());
                h.update(&new_snapshot_version.to_le_bytes());
            }
            Self::Noop => {}
        }
        h.finalize()
    }
}
