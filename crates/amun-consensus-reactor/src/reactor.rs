use crate::event::ReactorEvent;
use amun_chain_position::ChainPosition;
use amun_deterministic_scheduler::{DeterministicScheduler, ResourceBudget};
use amun_deterministic_timer::DeterministicTimerWheel;
use amun_pacemaker::{Pacemaker, RoundPhase};

#[derive(Debug, Clone)]
pub struct ConsensusReactor {
    pub pacemaker: Pacemaker,
    pub scheduler: DeterministicScheduler,
    timer: DeterministicTimerWheel,
    pub current_epoch: u64,
    pub finalized_height: u64,
    event_queue: Vec<ReactorEvent>,
}

impl ConsensusReactor {
    pub fn new(position: ChainPosition) -> Self {
        Self {
            pacemaker: Pacemaker::new(position),
            scheduler: DeterministicScheduler::new(ResourceBudget::new(100000)),
            timer: DeterministicTimerWheel::new(),
            current_epoch: 0,
            finalized_height: 0,
            event_queue: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: ReactorEvent) {
        self.event_queue.push(event);
    }

    pub fn tick(&mut self) -> Vec<ReactorEvent> {
        let mut output = Vec::new();

        for event in self.event_queue.drain(..) {
            match &event {
                ReactorEvent::ProposalReceived {
                    position,
                    round,
                    block_hash: _,
                } => {
                    if *round >= self.pacemaker.current_round {
                        self.pacemaker.on_progress();
                        self.pacemaker.advance_position(*position);
                    }
                }
                ReactorEvent::PrevoteQuorum { .. } => {
                    self.pacemaker.round_state.prevote_quorum = true;
                    self.pacemaker.round_state.advance_phase();
                }
                ReactorEvent::PrecommitQuorum { .. } => {
                    self.pacemaker.round_state.precommit_quorum = true;
                    self.pacemaker.round_state.advance_phase();
                }
                ReactorEvent::RoundTimeout { .. } => {
                    let new_round = self.pacemaker.current_round + 1;
                    let _ = self.pacemaker.start_round(new_round);
                }
                ReactorEvent::ViewChangeQuorum { new_round } => {
                    let _ = self.pacemaker.start_round(*new_round);
                    self.pacemaker.on_progress();
                }
                ReactorEvent::EpochTransition { new_epoch } => {
                    self.current_epoch = *new_epoch;
                }
                ReactorEvent::Tick => {}
            }
            output.push(event);
        }

        let _batch = self.scheduler.execute_batch(10);
        self.timer.advance();
        output
    }

    pub fn round(&self) -> u64 {
        self.pacemaker.current_round
    }
    pub fn phase(&self) -> &RoundPhase {
        &self.pacemaker.round_state.phase
    }
}
