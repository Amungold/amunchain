use super::compatibility::{CompatibilityEngine, CompatibilityLevel, CompatibilityMatrix};
use super::constitutional_identity::ConstitutionalIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalRelationship {
    Identical,
    SameConstitution {
        local_version: u32,
        remote_version: u32,
    },
    ForeignCivilization {
        compatibility: CompatibilityMatrix,
    },
    Incompatible {
        reason: String,
    },
}

pub struct TransitionClassifier;

impl TransitionClassifier {
    pub fn classify(
        local: &ConstitutionalIdentity,
        remote: &ConstitutionalIdentity,
    ) -> ConstitutionalRelationship {
        let matrix = CompatibilityEngine::compute(local, remote);

        match matrix.level {
            CompatibilityLevel::FullyCompatible => ConstitutionalRelationship::Identical,
            CompatibilityLevel::ReplayCompatible | CompatibilityLevel::SnapshotCompatible => {
                if local.constitutional_hash == remote.constitutional_hash {
                    ConstitutionalRelationship::SameConstitution {
                        local_version: local.protocol_version,
                        remote_version: remote.protocol_version,
                    }
                } else {
                    ConstitutionalRelationship::ForeignCivilization {
                        compatibility: matrix,
                    }
                }
            }
            CompatibilityLevel::ReadOnlyCompatible => {
                ConstitutionalRelationship::ForeignCivilization {
                    compatibility: matrix,
                }
            }
            CompatibilityLevel::Incompatible => ConstitutionalRelationship::Incompatible {
                reason: format!("Incompatible civilizations: different constitutional foundations"),
            },
        }
    }

    pub fn can_sync(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> bool {
        CompatibilityEngine::can_sync(local, remote)
    }

    pub fn can_migrate(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> bool {
        CompatibilityEngine::can_migrate(local, remote)
    }
}
