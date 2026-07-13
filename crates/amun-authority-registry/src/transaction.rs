#[allow(unused_imports)]
use crate::governance::{GovernanceAction, GovernanceProposal};
use crate::voting::GovernanceVote;
use serde::{Deserialize, Serialize};

/// A governance transaction that can be included in a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernanceTransaction {
    /// Submit a new governance proposal.
    SubmitProposal(GovernanceProposal),
    /// Cast a vote on an existing proposal.
    CastVote {
        proposal_id: [u8; 32],
        validator_id: [u8; 32],
        vote: GovernanceVote,
    },
}

/// The governance state that lives in the chain state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GovernanceState {
    pub proposals: std::collections::BTreeMap<[u8; 32], GovernanceProposal>,
    pub votes: std::collections::BTreeMap<[u8; 32], crate::voting::ProposalVotes>,
    pub journal: crate::executor::ExecutionJournal,
}

impl GovernanceState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a governance transaction and update state.
    pub fn apply_transaction(&mut self, tx: &GovernanceTransaction) {
        match tx {
            GovernanceTransaction::SubmitProposal(proposal) => {
                self.proposals
                    .insert(proposal.proposal_id, proposal.clone());
                self.votes.insert(
                    proposal.proposal_id,
                    crate::voting::ProposalVotes::new(proposal.proposal_id),
                );
            }
            GovernanceTransaction::CastVote {
                proposal_id,
                validator_id,
                vote,
            } => {
                if let Some(votes) = self.votes.get_mut(proposal_id) {
                    votes.submit_vote(*validator_id, *vote);
                }
            }
        }
    }

    /// After block finalization, execute any approved proposals.
    pub fn finalize_block(
        &mut self,
        total_validators: usize,
        registry: &mut crate::registry::AuthorityRegistry,
    ) -> Result<Vec<[u8; 32]>, String> {
        let mut executed = Vec::new();
        for (id, proposal) in &self.proposals {
            if let Some(votes) = self.votes.get(id) {
                if votes.is_approved(total_validators) && !self.journal.is_executed(id) {
                    crate::executor::execute_governance(
                        proposal,
                        votes,
                        total_validators,
                        registry,
                        &mut self.journal,
                    )
                    .map_err(|e| format!("Execution failed for {}: {:?}", hex::encode(id), e))?;
                    executed.push(*id);
                }
            }
        }
        Ok(executed)
    }

    /// Serialize the governance state to a byte vector for persistence.
    pub fn snapshot(&self) -> Vec<u8> {
        postcard::to_stdvec(&self).expect("GovernanceState serialization must not fail")
    }

    /// Restore governance state from a previously saved snapshot.
    pub fn restore(bytes: &[u8]) -> Result<Self, String> {
        postcard::from_bytes(bytes)
            .map_err(|e| format!("GovernanceState deserialization failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::ConstitutionalAuthority;
    use crate::governance::{GovernanceAction, GovernanceProposal};
    use crate::voting::GovernanceVote;

    #[test]
    fn n107_7d_submit_proposal_transaction() {
        let mut state = GovernanceState::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [3u8; 32],
            authority_version: 3,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        let tx = GovernanceTransaction::SubmitProposal(proposal.clone());
        state.apply_transaction(&tx);
        assert!(state.proposals.contains_key(&proposal.proposal_id));
    }

    #[test]
    fn n107_7d_cast_vote_transaction() {
        let mut state = GovernanceState::new();
        let action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));

        let vote_tx = GovernanceTransaction::CastVote {
            proposal_id: proposal.proposal_id,
            validator_id: [1u8; 32],
            vote: GovernanceVote::Approve,
        };
        state.apply_transaction(&vote_tx);
        assert_eq!(
            state
                .votes
                .get(&proposal.proposal_id)
                .unwrap()
                .total_votes(),
            1
        );
    }

    #[test]
    fn n107_7d_finalize_block_executes_governance() {
        let mut state = GovernanceState::new();
        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let action = GovernanceAction::AddAuthority {
            authority_public_key: [2u8; 32],
            authority_version: 2,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));

        // Cast 3 approving votes (quorum for 4 validators)
        for id in 1..=3u8 {
            state.apply_transaction(&GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [id; 32],
                vote: GovernanceVote::Approve,
            });
        }

        let executed = state.finalize_block(4, &mut registry).unwrap();
        assert_eq!(executed.len(), 1);
        assert!(registry.by_version(2).is_some());
    }

    #[test]
    fn n107_8a_snapshot_roundtrip() {
        let mut state = GovernanceState::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [3u8; 32],
            authority_version: 3,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));
        state.apply_transaction(&GovernanceTransaction::CastVote {
            proposal_id: proposal.proposal_id,
            validator_id: [1u8; 32],
            vote: GovernanceVote::Approve,
        });

        let bytes = state.snapshot();
        let restored = GovernanceState::restore(&bytes).unwrap();
        assert!(restored.proposals.contains_key(&proposal.proposal_id));
        assert_eq!(
            restored
                .votes
                .get(&proposal.proposal_id)
                .unwrap()
                .total_votes(),
            1
        );
    }

    #[test]
    fn n107_8a_empty_snapshot() {
        let state = GovernanceState::new();
        let bytes = state.snapshot();
        let restored = GovernanceState::restore(&bytes).unwrap();
        assert!(restored.proposals.is_empty());
    }

    #[test]
    fn n107_8a_multiple_proposals() {
        let mut state = GovernanceState::new();
        for i in 0..3u8 {
            let action = GovernanceAction::RetireAuthority {
                authority_version: i as u64,
            };
            let proposal = GovernanceProposal::new([i; 32], action, 100 + i as u64);
            state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal));
        }
        let bytes = state.snapshot();
        let restored = GovernanceState::restore(&bytes).unwrap();
        assert_eq!(restored.proposals.len(), 3);
    }

    #[test]
    fn n107_8a_executed_proposals_survive_snapshot() {
        let mut state = GovernanceState::new();
        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let action = GovernanceAction::AddAuthority {
            authority_public_key: [2u8; 32],
            authority_version: 2,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));
        for id in 1..=3u8 {
            state.apply_transaction(&GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [id; 32],
                vote: GovernanceVote::Approve,
            });
        }
        state.finalize_block(4, &mut registry).unwrap();

        let bytes = state.snapshot();
        let restored = GovernanceState::restore(&bytes).unwrap();
        assert!(restored.journal.is_executed(&proposal.proposal_id));
    }
}
