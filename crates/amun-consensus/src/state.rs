use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStep {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    pub height: u64,
    pub round: u64,
    pub step: ConsensusStep,
    pub locked_block: Option<[u8; 32]>,
    pub locked_round: u64,
    pub valid_block: Option<[u8; 32]>,
    pub valid_round: u64,
}

impl ConsensusState {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            round: 0,
            step: ConsensusStep::Propose,
            locked_block: None,
            locked_round: 0,
            valid_block: None,
            valid_round: 0,
        }
    }

    pub fn advance_round(&mut self) -> Result<(), &'static str> {
        self.round = self.round.checked_add(1).ok_or("round overflow")?;
        self.step = ConsensusStep::Propose;
        Ok(())
    }

    pub fn advance_step(&mut self) {
        self.step = match self.step {
            ConsensusStep::Propose => ConsensusStep::Prevote,
            ConsensusStep::Prevote => ConsensusStep::Precommit,
            ConsensusStep::Precommit => ConsensusStep::Commit,
            ConsensusStep::Commit => ConsensusStep::Propose,
        };
    }

    /// Transition to the next height. Fails on overflow instead of saturating.
    pub fn commit(&mut self) -> Result<(), &'static str> {
        self.height = self.height.checked_add(1).ok_or("height overflow")?;
        self.round = 0;
        self.step = ConsensusStep::Propose;
        self.locked_block = None;
        self.locked_round = 0;
        self.valid_block = None;
        self.valid_round = 0;
        Ok(())
    }

    pub fn lock_on(&mut self, block_hash: [u8; 32]) {
        self.locked_block = Some(block_hash);
        self.locked_round = self.round;
    }

    pub fn see_valid(&mut self, block_hash: [u8; 32]) {
        self.valid_block = Some(block_hash);
        self.valid_round = self.round;
    }
}
