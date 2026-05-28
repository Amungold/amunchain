use super::fork_merge::MergeDeclaration;

/// Merge resolution semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeResolution {
    /// The merged replay contains all entries from both parents (union)
    ReplayUnion,
    /// The longer replay chain survives (winner-takes-all)
    ReplayLongestChainWins,
    /// Replay histories are interleaved by epoch
    ReplayEpochInterleave,
    /// Identities are merged into a new composite identity
    IdentityMerged { new_constitution_hash: [u8; 32] },
    /// One identity absorbs the other
    IdentityAbsorbed {
        survivor_hash: [u8; 32],
        absorbed_hash: [u8; 32],
    },
    /// Identities remain separate (federation)
    IdentityFederated,
}

/// Merge Conflict Resolution Algebra.
pub struct MergeConflictResolver;

impl MergeConflictResolver {
    /// Resolve replay conflict between merging civilizations.
    pub fn resolve_replay(
        a_replay_height: u64,
        b_replay_height: u64,
        merge_type: &super::fork_merge::MergeType,
    ) -> MergeResolution {
        match merge_type {
            super::fork_merge::MergeType::Union => MergeResolution::ReplayUnion,
            super::fork_merge::MergeType::Absorption { .. } => {
                if a_replay_height >= b_replay_height {
                    MergeResolution::ReplayLongestChainWins
                } else {
                    MergeResolution::ReplayLongestChainWins
                }
            }
            super::fork_merge::MergeType::Federation => MergeResolution::ReplayEpochInterleave,
        }
    }

    /// Resolve identity conflict.
    pub fn resolve_identity(merge: &MergeDeclaration) -> MergeResolution {
        match &merge.merge_type {
            super::fork_merge::MergeType::Union => MergeResolution::IdentityMerged {
                new_constitution_hash: merge.target_civilization,
            },
            super::fork_merge::MergeType::Absorption { survivor } => {
                MergeResolution::IdentityAbsorbed {
                    survivor_hash: *survivor,
                    absorbed_hash: merge
                        .source_civilizations
                        .iter()
                        .find(|s| *s != survivor)
                        .copied()
                        .unwrap_or([0u8; 32]),
                }
            }
            super::fork_merge::MergeType::Federation => MergeResolution::IdentityFederated,
        }
    }
}
