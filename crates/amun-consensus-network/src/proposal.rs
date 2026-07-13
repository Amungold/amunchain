use crate::engine::ConsensusEngine;
use crate::round::ConsensusRound;

impl ConsensusEngine {
    pub fn start_round(&mut self, height: u64, proposer_id: [u8; 32]) {
        self.rounds
            .entry(height)
            .or_insert_with(|| ConsensusRound::new(height, proposer_id));
    }

    pub fn round_mut(&mut self, height: u64) -> Option<&mut ConsensusRound> {
        self.rounds.get_mut(&height)
    }
}
