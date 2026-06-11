use crate::types::{BlockProposal, Hash256};
use crate::validator::ValidatorSet;

pub trait LeaderSelector: Send + Sync {
    fn leader(&self, height: u64, round: u64, validator_set: &ValidatorSet) -> Option<Hash256>;
}

pub struct RoundRobinSelector;

impl LeaderSelector for RoundRobinSelector {
    fn leader(&self, height: u64, round: u64, validator_set: &ValidatorSet) -> Option<Hash256> {
        if validator_set.validators.is_empty() {
            return None;
        }
        let index = ((height.wrapping_add(round)) as usize) % validator_set.validators.len();
        Some(validator_set.validators[index].id)
    }
}

pub struct ProposalManager {
    pub current_proposal: Option<BlockProposal>,
    pub designated_proposer: Option<Hash256>,
}

impl ProposalManager {
    pub fn new() -> Self {
        Self {
            current_proposal: None,
            designated_proposer: None,
        }
    }

    pub fn prepare_round(
        &mut self,
        height: u64,
        round: u64,
        selector: &dyn LeaderSelector,
        validator_set: &ValidatorSet,
    ) {
        self.designated_proposer = selector.leader(height, round, validator_set);
        self.current_proposal = None;
    }

    pub fn accept_proposal(
        &mut self,
        proposal: BlockProposal,
        proposer_id: Hash256,
    ) -> Result<(), &'static str> {
        // Verify the proposal's own proposer field matches the caller.
        if proposal.proposer != proposer_id {
            return Err("proposal proposer mismatch");
        }
        match self.designated_proposer {
            Some(designated) if designated == proposer_id => {
                self.current_proposal = Some(proposal);
                Ok(())
            }
            Some(_) => Err("proposal from non-leader"),
            None => Err("no designated proposer for this round"),
        }
    }

    pub fn proposed_block_hash(&self) -> Option<Hash256> {
        self.current_proposal.as_ref().map(|p| p.block_hash)
    }

    pub fn reset(&mut self) {
        self.current_proposal = None;
        self.designated_proposer = None;
    }
}

impl Default for ProposalManager {
    fn default() -> Self {
        Self::new()
    }
}
