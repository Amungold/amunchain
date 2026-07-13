use super::freeze_map::{ConstitutionalFreezeMap, FreezeBoundary, MutabilityClass};
use amun_lineage::record::EvolutionProof;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreezeViolation {
    ImmutableViolation { field: String, description: String },
    InsufficientQuorum { field: String, required: String },
    ReplayPreservationViolation { field: String },
    SnapshotCompatibilityViolation { field: String },
    PhysicsProofRequired { field: String },
    PhysicsProofFailed { field: String, reason: String },
}

#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub evolution_proof: Option<EvolutionProof>,
    pub preserves_replay: bool,
    pub preserves_snapshot: bool,
    pub preserves_empty_root: bool,
    pub preserves_max_depth: bool,
    pub preserves_proof_semantics: bool,
    pub preserves_hash_domains: bool,
}

pub struct FreezeBoundaryValidator;

impl FreezeBoundaryValidator {
    pub fn validate_change(
        field_name: &str,
        context: &ValidationContext,
    ) -> Result<(), FreezeViolation> {
        let boundaries = ConstitutionalFreezeMap::frozen_boundaries();
        let boundary = boundaries
            .iter()
            .find(|b| b.field_name == field_name)
            .ok_or_else(|| FreezeViolation::ImmutableViolation {
                field: field_name.to_string(),
                description: "Unknown field - cannot modify unregistered fields".to_string(),
            })?;

        if boundary.mutability == MutabilityClass::Immutable {
            return Err(FreezeViolation::ImmutableViolation {
                field: field_name.to_string(),
                description: boundary.description.to_string(),
            });
        }

        if boundary.requires_replay_preservation && !context.preserves_replay {
            return Err(FreezeViolation::ReplayPreservationViolation {
                field: field_name.to_string(),
            });
        }

        if boundary.requires_snapshot_compatibility && !context.preserves_snapshot {
            return Err(FreezeViolation::SnapshotCompatibilityViolation {
                field: field_name.to_string(),
            });
        }

        if let Some(proof) = &context.evolution_proof {
            if !proof.verify() {
                return Err(FreezeViolation::PhysicsProofFailed {
                    field: field_name.to_string(),
                    reason: "Evolution proof verification failed".to_string(),
                });
            }
        } else if boundary.requires_replay_preservation || boundary.requires_snapshot_compatibility
        {
            return Err(FreezeViolation::PhysicsProofRequired {
                field: field_name.to_string(),
            });
        }

        Ok(())
    }

    pub fn is_immutable(field_name: &str) -> bool {
        ConstitutionalFreezeMap::frozen_boundaries()
            .iter()
            .any(|b| b.field_name == field_name && b.mutability == MutabilityClass::Immutable)
    }

    pub fn get_boundary(field_name: &str) -> Option<FreezeBoundary> {
        ConstitutionalFreezeMap::frozen_boundaries()
            .into_iter()
            .find(|b| b.field_name == field_name)
    }
}
