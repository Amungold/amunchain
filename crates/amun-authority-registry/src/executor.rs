use crate::authority::ConstitutionalAuthority;
use crate::governance::{GovernanceAction, GovernanceProposal};
use crate::registry::{AuthorityRegistry, AuthorityTransition};
use crate::voting::ProposalVotes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Errors that can occur during governance execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceError {
    AlreadyExecuted,
    QuorumNotReached,
    VoteNotApproved,
    InvalidAction(String),
}

/// Tracks which proposals have been executed to prevent replay.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionJournal {
    executed: BTreeSet<[u8; 32]>,
}

impl ExecutionJournal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_executed(&mut self, proposal_id: &[u8; 32]) {
        self.executed.insert(*proposal_id);
    }

    pub fn is_executed(&self, proposal_id: &[u8; 32]) -> bool {
        self.executed.contains(proposal_id)
    }
}

/// Executes an approved governance proposal against the authority registry.
pub fn execute_governance(
    proposal: &GovernanceProposal,
    votes: &ProposalVotes,
    total_validators: usize,
    registry: &mut AuthorityRegistry,
    journal: &mut ExecutionJournal,
) -> Result<(), GovernanceError> {
    // Guard: already executed
    if journal.is_executed(&proposal.proposal_id) {
        return Err(GovernanceError::AlreadyExecuted);
    }

    // Guard: quorum
    if !votes.reached_quorum(total_validators) {
        return Err(GovernanceError::QuorumNotReached);
    }

    // Guard: majority
    if !votes.is_approved(total_validators) {
        return Err(GovernanceError::VoteNotApproved);
    }

    // Execute the action
    match &proposal.action {
        GovernanceAction::AddAuthority {
            authority_public_key,
            authority_version,
        } => {
            let authority = ConstitutionalAuthority::new(
                *authority_public_key,
                *authority_version,
                proposal.created_height,
            );
            registry.register(authority);
        }
        GovernanceAction::ScheduleTransition {
            from_version,
            to_version,
            activation_height,
            grace_period_blocks,
        } => {
            // Verify both versions exist
            if registry.by_version(*from_version).is_none() {
                return Err(GovernanceError::InvalidAction(
                    format!("from_version {} does not exist", from_version)
                ));
            }
            if registry.by_version(*to_version).is_none() {
                return Err(GovernanceError::InvalidAction(
                    format!("to_version {} does not exist", to_version)
                ));
            }
            let transition = AuthorityTransition {
                from_version: *from_version,
                to_version: *to_version,
                activation_height: *activation_height,
                grace_period_blocks: *grace_period_blocks,
            };
            registry.schedule_transition(transition);
        }
        GovernanceAction::RetireAuthority { authority_version } => {
            if registry.by_version(*authority_version).is_none() {
                return Err(GovernanceError::InvalidAction(
                    format!("authority_version {} does not exist", authority_version)
                ));
            }
            registry.retire(*authority_version, proposal.created_height);
        }
    }

    // Mark as executed
    journal.mark_executed(&proposal.proposal_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::governance::{GovernanceAction, GovernanceProposal};
    use crate::voting::GovernanceVote;

    fn setup_registry_with_two_authorities() -> AuthorityRegistry {
        let mut reg = AuthorityRegistry::new();
        reg.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));
        reg.register(ConstitutionalAuthority::new([2u8; 32], 2, 0));
        reg
    }

    #[test]
    fn n107_7c_execute_add_authority() {
        let mut reg = AuthorityRegistry::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [3u8; 32],
            authority_version: 3,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Approve);
        votes.submit_vote([2u8; 32], GovernanceVote::Approve);
        votes.submit_vote([3u8; 32], GovernanceVote::Approve);
        let mut journal = ExecutionJournal::new();

        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
        assert!(reg.by_version(3).is_some());
        assert!(journal.is_executed(&proposal.proposal_id));
    }

    #[test]
    fn n107_7c_execute_transition() {
        let mut reg = setup_registry_with_two_authorities();
        let action = GovernanceAction::ScheduleTransition {
            from_version: 1,
            to_version: 2,
            activation_height: 500,
            grace_period_blocks: 100,
        };
        let proposal = GovernanceProposal::new([0xBB; 32], action, 200);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Approve);
        votes.submit_vote([2u8; 32], GovernanceVote::Approve);
        votes.submit_vote([3u8; 32], GovernanceVote::Approve);
        let mut journal = ExecutionJournal::new();

        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
        assert!(reg.transition.is_some());
    }

    #[test]
    fn n107_7c_execute_retirement() {
        let mut reg = setup_registry_with_two_authorities();
        let action = GovernanceAction::RetireAuthority { authority_version: 1 };
        let proposal = GovernanceProposal::new([0xCC; 32], action, 300);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Approve);
        votes.submit_vote([2u8; 32], GovernanceVote::Approve);
        votes.submit_vote([3u8; 32], GovernanceVote::Approve);
        let mut journal = ExecutionJournal::new();

        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
        assert!(reg.is_revoked(1));
    }

    #[test]
    fn n107_7c_reject_without_quorum() {
        let mut reg = setup_registry_with_two_authorities();
        let action = GovernanceAction::RetireAuthority { authority_version: 1 };
        let proposal = GovernanceProposal::new([0xDD; 32], action, 400);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Approve);
        votes.submit_vote([2u8; 32], GovernanceVote::Approve); // 2/4 => no quorum
        let mut journal = ExecutionJournal::new();

        let result = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
        assert_eq!(result, Err(GovernanceError::QuorumNotReached));
    }

    #[test]
    fn n107_7c_reject_failed_vote() {
        let mut reg = setup_registry_with_two_authorities();
        let action = GovernanceAction::RetireAuthority { authority_version: 1 };
        let proposal = GovernanceProposal::new([0xEE; 32], action, 500);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Reject);
        votes.submit_vote([2u8; 32], GovernanceVote::Reject);
        votes.submit_vote([3u8; 32], GovernanceVote::Approve); // quorum yes, but rejections > approvals
        let mut journal = ExecutionJournal::new();

        let result = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
        assert_eq!(result, Err(GovernanceError::VoteNotApproved));
    }

    #[test]
    fn n107_7c_idempotent_execution() {
        let mut reg = setup_registry_with_two_authorities();
        let action = GovernanceAction::RetireAuthority { authority_version: 1 };
        let proposal = GovernanceProposal::new([0xFF; 32], action, 600);
        let mut votes = ProposalVotes::new(proposal.proposal_id);
        votes.submit_vote([1u8; 32], GovernanceVote::Approve);
        votes.submit_vote([2u8; 32], GovernanceVote::Approve);
        votes.submit_vote([3u8; 32], GovernanceVote::Approve);
        let mut journal = ExecutionJournal::new();

        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
        let second = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
        assert_eq!(second, Err(GovernanceError::AlreadyExecuted));
    }
}
