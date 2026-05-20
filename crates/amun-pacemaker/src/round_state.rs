/// Tracks the state of the current consensus round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundPhase {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

#[derive(Debug, Clone)]
pub struct RoundState {
    pub round: u64,
    pub phase: RoundPhase,
    pub proposal_received: bool,
    pub prevote_quorum: bool,
    pub precommit_quorum: bool,
}

impl RoundState {
    pub fn new() -> Self {
        Self {
            round: 0,
            phase: RoundPhase::Propose,
            proposal_received: false,
            prevote_quorum: false,
            precommit_quorum: false,
        }
    }

    pub fn enter_round(&mut self, round: u64) {
        self.round = round;
        self.phase = RoundPhase::Propose;
        self.proposal_received = false;
        self.prevote_quorum = false;
        self.precommit_quorum = false;
    }

    pub fn advance_phase(&mut self) -> bool {
        match self.phase {
            RoundPhase::Propose => {
                self.phase = RoundPhase::Prevote;
                true
            }
            RoundPhase::Prevote => {
                self.phase = RoundPhase::Precommit;
                true
            }
            RoundPhase::Precommit => {
                self.phase = RoundPhase::Commit;
                true
            }
            RoundPhase::Commit => false,
        }
    }
}

impl Default for RoundState {
    fn default() -> Self {
        Self::new()
    }
}
