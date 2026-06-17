use serde::{Deserialize, Serialize};

/// On-chain governance actions that modify the authority registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernanceAction {
    /// Add a new authority to the registry.
    AddAuthority {
        authority_public_key: [u8; 32],
        authority_version: u64,
    },
    /// Schedule a transition from one authority version to another.
    ScheduleTransition {
        from_version: u64,
        to_version: u64,
        activation_height: u64,
        grace_period_blocks: u64,
    },
    /// Retire an authority so it can no longer issue certificates.
    RetireAuthority { authority_version: u64 },
}

/// A governance proposal submitted by a validator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceProposal {
    pub proposal_id: [u8; 32],
    pub proposer: [u8; 32], // ValidatorId
    pub action: GovernanceAction,
    pub created_height: u64,
}

impl GovernanceProposal {
    pub fn new(proposer: [u8; 32], action: GovernanceAction, created_height: u64) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_GOVERNANCE_PROPOSAL_V1");
        hasher.update(&proposer);
        hasher.update(&created_height.to_le_bytes());
        match &action {
            GovernanceAction::AddAuthority {
                authority_public_key,
                authority_version,
            } => {
                hasher.update(b"AddAuthority");
                hasher.update(authority_public_key);
                hasher.update(&authority_version.to_le_bytes());
            }
            GovernanceAction::ScheduleTransition {
                from_version,
                to_version,
                activation_height,
                grace_period_blocks,
            } => {
                hasher.update(b"ScheduleTransition");
                hasher.update(&from_version.to_le_bytes());
                hasher.update(&to_version.to_le_bytes());
                hasher.update(&activation_height.to_le_bytes());
                hasher.update(&grace_period_blocks.to_le_bytes());
            }
            GovernanceAction::RetireAuthority { authority_version } => {
                hasher.update(b"RetireAuthority");
                hasher.update(&authority_version.to_le_bytes());
            }
        }
        let proposal_id = *hasher.finalize().as_bytes();
        Self {
            proposal_id,
            proposer,
            action,
            created_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n107_7_add_authority_proposal() {
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [1u8; 32],
            authority_version: 2,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        assert_eq!(proposal.created_height, 100);
    }

    #[test]
    fn n107_7_schedule_transition_proposal() {
        let action = GovernanceAction::ScheduleTransition {
            from_version: 1,
            to_version: 2,
            activation_height: 500,
            grace_period_blocks: 100,
        };
        let proposal = GovernanceProposal::new([0xBB; 32], action, 200);
        assert_eq!(proposal.proposer, [0xBB; 32]);
    }

    #[test]
    fn n107_7_retire_authority_proposal() {
        let action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xCC; 32], action, 300);
        assert!(!proposal.proposal_id.iter().all(|&b| b == 0));
    }

    #[test]
    fn n107_7_different_actions_produce_different_ids() {
        let p1 = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 1,
            },
            100,
        );
        let p2 = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 2,
            },
            100,
        );
        assert_ne!(p1.proposal_id, p2.proposal_id);
    }

    #[test]
    fn n107_7_reject_invalid_governance_duplicate_proposal() {
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [1u8; 32],
            authority_version: 2,
        };
        let p1 = GovernanceProposal::new([0xAA; 32], action.clone(), 100);
        let p2 = GovernanceProposal::new([0xAA; 32], action, 100);
        assert_eq!(p1.proposal_id, p2.proposal_id);
    }
}
