use crate::timeout_cert::TimeoutCertificate;
use amun_chain_position::ChainPosition;
use amun_validator_attestation::ValidatorSet;
use std::collections::BTreeSet;

/// Aggregates timeout votes and forms Timeout Certificates.
#[derive(Debug, Clone)]
pub struct TimeoutAggregator {
    /// timeout votes: round -> set of validator_ids
    timeout_votes: BTreeSet<(u64, u64)>,
    validator_set: ValidatorSet,
}

impl TimeoutAggregator {
    pub fn new(validator_set: ValidatorSet) -> Self {
        Self {
            timeout_votes: BTreeSet::new(),
            validator_set,
        }
    }

    /// Add a timeout vote. Returns Some(TC) if quorum reached.
    pub fn add_timeout_vote(
        &mut self,
        validator_id: u64,
        position: ChainPosition,
        round: u64,
    ) -> Option<TimeoutCertificate> {
        self.timeout_votes.insert((round, validator_id));

        // Count unique validators and their total stake
        let mut total_stake: u64 = 0;
        let mut seen = BTreeSet::new();
        for (r, vid) in &self.timeout_votes {
            if *r == round && seen.insert(*vid) {
                if let Some(vi) = self.validator_set.get_validator(*vid) {
                    total_stake += vi.stake;
                }
            }
        }

        if self.validator_set.has_quorum(total_stake) {
            let signatures: Vec<(u64, [u8; 64])> =
                seen.iter().map(|vid| (*vid, [0u8; 64])).collect();
            Some(TimeoutCertificate::new(position, round, signatures))
        } else {
            None
        }
    }

    pub fn vote_count(&self, round: u64) -> usize {
        self.timeout_votes
            .iter()
            .filter(|(r, _)| *r == round)
            .count()
    }
}
