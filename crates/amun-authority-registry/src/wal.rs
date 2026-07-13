use crate::transaction::{GovernanceState, GovernanceTransaction};
use serde::{Deserialize, Serialize};

/// A single entry in the governance write-ahead log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceWalRecord {
    pub block_height: u64,
    pub transaction: GovernanceTransaction,
}

/// Appends governance transactions to an in-memory WAL (for testing)
/// and provides replay functionality.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GovernanceWal {
    entries: Vec<GovernanceWalRecord>,
}

impl GovernanceWal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a transaction at a given block height.
    pub fn append(&mut self, block_height: u64, tx: GovernanceTransaction) {
        self.entries.push(GovernanceWalRecord {
            block_height,
            transaction: tx,
        });
    }

    /// Replay all entries in order against a fresh GovernanceState.
    pub fn replay(&self) -> GovernanceState {
        let mut state = GovernanceState::new();
        for record in &self.entries {
            state.apply_transaction(&record.transaction);
        }
        state
    }

    /// Replay and then finalize all approved proposals.
    pub fn replay_and_finalize(
        &self,
        total_validators: usize,
        registry: &mut crate::registry::AuthorityRegistry,
    ) -> Result<GovernanceState, String> {
        let mut state = self.replay();
        state.finalize_block(total_validators, registry)?;
        Ok(state)
    }

    /// Return all WAL entries with block_height greater than the given height.
    pub fn entries_since(&self, height: u64) -> Vec<&GovernanceWalRecord> {
        self.entries
            .iter()
            .filter(|e| e.block_height >= height)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::governance::{GovernanceAction, GovernanceProposal};
    use crate::voting::GovernanceVote;

    #[test]
    fn n107_8b_replay_single_proposal() {
        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [2u8; 32],
            authority_version: 2,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        wal.append(1, GovernanceTransaction::SubmitProposal(proposal.clone()));

        let restored = wal.replay();
        assert!(restored.proposals.contains_key(&proposal.proposal_id));
    }

    #[test]
    fn n107_8b_replay_votes() {
        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xBB; 32], action, 100);
        wal.append(1, GovernanceTransaction::SubmitProposal(proposal.clone()));
        wal.append(
            1,
            GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [1u8; 32],
                vote: GovernanceVote::Approve,
            },
        );

        let restored = wal.replay();
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
    fn n107_8b_replay_execution() {
        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [3u8; 32],
            authority_version: 3,
        };
        let proposal = GovernanceProposal::new([0xCC; 32], action, 100);
        wal.append(1, GovernanceTransaction::SubmitProposal(proposal.clone()));
        for id in 1..=3u8 {
            wal.append(
                1,
                GovernanceTransaction::CastVote {
                    proposal_id: proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                },
            );
        }

        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(crate::authority::ConstitutionalAuthority::new(
            [1u8; 32], 1, 0,
        ));

        let restored = wal.replay_and_finalize(4, &mut registry).unwrap();
        assert!(restored.journal.is_executed(&proposal.proposal_id));
        assert!(registry.by_version(3).is_some());
    }

    #[test]
    fn n107_8b_replay_after_crash() {
        let mut wal = GovernanceWal::new();
        wal.append(
            1,
            GovernanceTransaction::SubmitProposal(GovernanceProposal::new(
                [0xDD; 32],
                GovernanceAction::RetireAuthority {
                    authority_version: 1,
                },
                100,
            )),
        );

        // Simulate crash: create a new WAL and replay from persisted entries
        let persisted = postcard::to_stdvec(&wal).unwrap();
        let restored_wal: GovernanceWal = postcard::from_bytes(&persisted).unwrap();
        let state = restored_wal.replay();
        assert_eq!(state.proposals.len(), 1);
    }

    #[test]
    fn n107_8b_replay_deterministic() {
        let mut wal1 = GovernanceWal::new();
        let mut wal2 = GovernanceWal::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [4u8; 32],
            authority_version: 4,
        };
        let proposal = GovernanceProposal::new([0xEE; 32], action, 100);
        wal1.append(1, GovernanceTransaction::SubmitProposal(proposal.clone()));
        wal2.append(1, GovernanceTransaction::SubmitProposal(proposal.clone()));

        let state1 = wal1.replay();
        let state2 = wal2.replay();
        let bytes1 = state1.snapshot();
        let bytes2 = state2.snapshot();
        assert_eq!(bytes1, bytes2);
    }
}
