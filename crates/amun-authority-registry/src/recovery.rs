use crate::transaction::GovernanceState;
use crate::wal::GovernanceWal;

/// Engine that restores governance state from the latest snapshot
/// plus any WAL entries appended after the snapshot was taken.
pub struct GovernanceRecoveryEngine;

impl GovernanceRecoveryEngine {
    /// Recover the governance state by applying a base snapshot and
    /// replaying only WAL entries whose block_height is greater than
    /// the snapshot's height.
    pub fn recover(
        snapshot_bytes: &[u8],
        snapshot_height: u64,
        wal: &GovernanceWal,
        total_validators: usize,
        registry: &mut crate::registry::AuthorityRegistry,
    ) -> Result<GovernanceState, String> {
        // 1. Restore the snapshot
        let mut state = GovernanceState::restore(snapshot_bytes)?;

        // 2. Replay only WAL entries after the snapshot height
        let entries = wal.entries_since(snapshot_height);
        for record in entries {
            state.apply_transaction(&record.transaction);
        }

        // 3. Finalize any newly approved proposals
        state.finalize_block(total_validators, registry)?;

        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::ConstitutionalAuthority;
    use crate::governance::{GovernanceAction, GovernanceProposal};
    use crate::transaction::GovernanceTransaction;
    use crate::voting::GovernanceVote;

    fn build_base_snapshot() -> (Vec<u8>, u64, [u8; 32]) {
        let mut state = GovernanceState::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [1u8; 32],
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xAA; 32], action, 100);
        let proposal_id = proposal.proposal_id;
        state.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal));
        (state.snapshot(), 100, proposal_id)
    }

    #[test]
    fn n107_8c_recover_from_snapshot_and_wal() {
        let (snapshot, snap_height, _snap_id) = build_base_snapshot();

        // Append new transactions to the WAL after the snapshot
        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xBB; 32], action, 150);
        wal.append(150, GovernanceTransaction::SubmitProposal(proposal.clone()));
        wal.append(
            150,
            GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [1u8; 32],
                vote: GovernanceVote::Approve,
            },
        );

        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let recovered =
            GovernanceRecoveryEngine::recover(&snapshot, snap_height, &wal, 4, &mut registry)
                .unwrap();

        // The new proposal should be present
        assert!(recovered.proposals.contains_key(&proposal.proposal_id));
        // The original proposal from the snapshot should still be present
        assert_eq!(recovered.proposals.len(), 2);
    }

    #[test]
    fn n107_8c_partial_replay() {
        let (snapshot, snap_height, snap_id) = build_base_snapshot();

        let mut wal = GovernanceWal::new();
        // This entry is BEFORE the snapshot height and should be ignored
        let old_action = GovernanceAction::AddAuthority {
            authority_public_key: [9u8; 32],
            authority_version: 9,
        };
        wal.append(
            50,
            GovernanceTransaction::SubmitProposal(GovernanceProposal::new(
                [0x99; 32], old_action, 50,
            )),
        );

        // This entry is AFTER the snapshot height and should be replayed
        let new_action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let new_proposal = GovernanceProposal::new([0xCC; 32], new_action, 200);
        let new_proposal_id = new_proposal.proposal_id;
        wal.append(200, GovernanceTransaction::SubmitProposal(new_proposal));

        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let recovered =
            GovernanceRecoveryEngine::recover(&snapshot, snap_height, &wal, 4, &mut registry)
                .unwrap();

        // Only the entry after snapshot should be present (plus the snapshot's entry)
        assert_eq!(recovered.proposals.len(), 2);
        assert!(
            recovered.proposals.contains_key(&snap_id),
            "Snapshot proposal missing"
        );
        assert!(
            recovered.proposals.contains_key(&new_proposal_id),
            "WAL proposal missing"
        );
        assert!(!recovered.proposals.contains_key(&[0x99; 32])); // before snapshot, ignored
    }

    #[test]
    fn n107_8c_journal_recovery() {
        let (snapshot, snap_height, _snap_id) = build_base_snapshot();

        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::AddAuthority {
            authority_public_key: [2u8; 32],
            authority_version: 2,
        };
        let proposal = GovernanceProposal::new([0xDD; 32], action, 150);
        wal.append(150, GovernanceTransaction::SubmitProposal(proposal.clone()));
        for id in 1..=3u8 {
            wal.append(
                150,
                GovernanceTransaction::CastVote {
                    proposal_id: proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                },
            );
        }

        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let recovered =
            GovernanceRecoveryEngine::recover(&snapshot, snap_height, &wal, 4, &mut registry)
                .unwrap();

        // The executed proposal should be in the journal
        assert!(recovered.journal.is_executed(&proposal.proposal_id));
        // The new authority should be registered
        assert!(registry.by_version(2).is_some());
    }

    #[test]
    fn n107_8c_vote_recovery() {
        let (snapshot, snap_height, _snap_id) = build_base_snapshot();

        let mut wal = GovernanceWal::new();
        let action = GovernanceAction::RetireAuthority {
            authority_version: 1,
        };
        let proposal = GovernanceProposal::new([0xEE; 32], action, 150);
        wal.append(150, GovernanceTransaction::SubmitProposal(proposal.clone()));
        wal.append(
            150,
            GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [1u8; 32],
                vote: GovernanceVote::Approve,
            },
        );
        wal.append(
            150,
            GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [2u8; 32],
                vote: GovernanceVote::Reject,
            },
        );

        let mut registry = crate::registry::AuthorityRegistry::new();
        registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

        let recovered =
            GovernanceRecoveryEngine::recover(&snapshot, snap_height, &wal, 4, &mut registry)
                .unwrap();

        let votes = recovered.votes.get(&proposal.proposal_id).unwrap();
        assert_eq!(votes.total_votes(), 2);
        let tally = votes.tally();
        assert_eq!(tally.approvals, 1);
        assert_eq!(tally.rejections, 1);
    }

    #[test]
    fn n107_8c_deterministic_recovery() {
        let (snapshot, snap_height, _snap_id) = build_base_snapshot();

        let run_recovery = || {
            let mut wal = GovernanceWal::new();
            let action = GovernanceAction::AddAuthority {
                authority_public_key: [5u8; 32],
                authority_version: 5,
            };
            let proposal = GovernanceProposal::new([0xFF; 32], action, 200);
            wal.append(200, GovernanceTransaction::SubmitProposal(proposal.clone()));
            for id in 1..=3u8 {
                wal.append(
                    200,
                    GovernanceTransaction::CastVote {
                        proposal_id: proposal.proposal_id,
                        validator_id: [id; 32],
                        vote: GovernanceVote::Approve,
                    },
                );
            }

            let mut registry = crate::registry::AuthorityRegistry::new();
            registry.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));

            let recovered =
                GovernanceRecoveryEngine::recover(&snapshot, snap_height, &wal, 4, &mut registry)
                    .unwrap();
            recovered.snapshot()
        };

        let bytes1 = run_recovery();
        let bytes2 = run_recovery();
        assert_eq!(bytes1, bytes2);
    }
}
