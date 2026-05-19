use amun_consensus_types::{ConsensusRound, ValidatorIndex};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use hashbrown::HashSet;

pub struct VoteTracker {
    seen_votes: HashSet<(u64, u16)>,
    max_tracked_rounds: u64,
}

impl VoteTracker {
    pub fn new() -> Self { Self { seen_votes: HashSet::new(), max_tracked_rounds: 10 } }
    pub fn is_duplicate(&self, round: ConsensusRound, validator: ValidatorIndex) -> bool {
        self.seen_votes.contains(&(round.value(), validator.value()))
    }
    pub fn record_vote(&mut self, round: ConsensusRound, validator: ValidatorIndex) -> AmunResult<()> {
        let key = (round.value(), validator.value());
        if self.seen_votes.contains(&key) {
            return Err(FailureContext::new(ConstitutionalFault::InvalidInput, 0x000D, 0x0010));
        }
        self.seen_votes.insert(key);
        Ok(())
    }
    pub fn advance_round(&mut self, new_round: u64) {
        if new_round > self.max_tracked_rounds {
            let cutoff = new_round - self.max_tracked_rounds + 1;
            self.seen_votes.retain(|(round, _)| *round >= cutoff);
        }
    }
    pub fn clear(&mut self) { self.seen_votes.clear(); }
    pub fn len(&self) -> usize { self.seen_votes.len() }
}
