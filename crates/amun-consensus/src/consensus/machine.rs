//! Validator State Machine - PANIC-FREE

use super::qc_store::{QC, QCHash};
use super::state::ConsensusState;
use super::pacemaker::{Pacemaker, Round, PacemakerConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolPhase { Propose, Vote, Precommit, Commit, Timeout }

#[derive(Debug)]
pub struct ValidatorStateMachine {
    pub current_phase: ProtocolPhase,
    pub current_round: Round,
    pub current_proposal: Option<[u8; 32]>,
    pub voted_in_round: bool,
    pub consensus_state: ConsensusState,
    pub pacemaker: Pacemaker,
}

impl ValidatorStateMachine {
    pub fn new(cfg: PacemakerConfig) -> Self {
        Self {
            current_phase: ProtocolPhase::Propose,
            current_round: Round::new(0),
            current_proposal: None,
            voted_in_round: false,
            consensus_state: ConsensusState::new(),
            pacemaker: Pacemaker::new(cfg),
        }
    }
    
    pub fn process_qc(&mut self, qc: QC) -> (Option<QCHash>, bool) {
        let finalized = self.consensus_state.update_qc(qc);
        if finalized {
            self.current_phase = ProtocolPhase::Commit;
            self.pacemaker.on_commit();
        }
        (None, finalized)
    }
    
    pub fn advance_round(&mut self) -> Result<(), &'static str> {
        self.pacemaker.advance_round()?;
        self.current_round = self.pacemaker.current_round();
        self.voted_in_round = false;
        self.current_proposal = None;
        self.current_phase = ProtocolPhase::Propose;
        Ok(())
    }
    
    pub fn should_timeout(&self) -> bool {
        self.pacemaker.is_timeout()
    }
    
    pub fn current_leader(&self) -> u64 {
        self.pacemaker.current_leader()
    }
    
    pub fn can_vote(&self, _proposal: [u8; 32]) -> bool {
        !self.voted_in_round && self.current_phase == ProtocolPhase::Vote
    }
    
    pub fn record_vote(&mut self) {
        self.voted_in_round = true;
    }
    
    pub fn locked_height(&self) -> Option<u64> {
        self.consensus_state.locked_height()
    }
}
