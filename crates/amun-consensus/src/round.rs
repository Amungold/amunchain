use amun_consensus_types::ConsensusRound;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::Epoch;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundPhase {
    Proposal,
    Prepare,
    PreCommit,
    Commit,
    Timeout,
}

#[derive(Clone, Debug)]
pub struct RoundState {
    pub epoch: Epoch,
    pub round: ConsensusRound,
    pub phase: RoundPhase,
    pub timeout_count: u32,
    pub max_timeout_count: u32,
}

impl RoundState {
    pub fn new(epoch: Epoch) -> Self {
        Self {
            epoch,
            round: ConsensusRound::new(0),
            phase: RoundPhase::Proposal,
            timeout_count: 0,
            max_timeout_count: 100,
        }
    }

    pub fn advance_phase(&mut self) -> AmunResult<()> {
        self.phase = match self.phase {
            RoundPhase::Proposal => RoundPhase::Prepare,
            RoundPhase::Prepare => RoundPhase::PreCommit,
            RoundPhase::PreCommit => RoundPhase::Commit,
            RoundPhase::Commit => RoundPhase::Proposal,
            RoundPhase::Timeout => RoundPhase::Proposal,
        };
        Ok(())
    }

    pub fn advance_round(&mut self) -> AmunResult<()> {
        self.round = self.round.next()?;
        self.phase = RoundPhase::Proposal;
        self.timeout_count = self.timeout_count.saturating_add(1);

        if self.timeout_count > self.max_timeout_count {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000D,
                0x0001,
            ));
        }
        Ok(())
    }

    pub fn on_timeout(&mut self) -> AmunResult<()> {
        self.phase = RoundPhase::Timeout;
        self.advance_round()
    }

    pub fn is_leader(&self, validator_index: usize, total_validators: usize) -> bool {
        if total_validators == 0 {
            return false;
        }
        validator_index == (self.round.value() as usize % total_validators)
    }
}
