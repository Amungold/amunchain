use amun_chain_position::ChainPosition;
use amun_deterministic_timer::{DeterministicTimerWheel, TimeoutLaw};
use crate::round_state::RoundState;

/// Consensus pacemaker: drives round progression using logical time.
/// Produces deterministic timeout behavior across all validators.
#[derive(Debug, Clone)]
pub struct Pacemaker {
    pub current_round: u64,
    pub current_position: ChainPosition,
    pub round_state: RoundState,
    timer: DeterministicTimerWheel,
    timeout_law: TimeoutLaw,
    timeout_timer_id: Option<u64>,
    round_started: bool,
}

impl Pacemaker {
    pub fn new(position: ChainPosition) -> Self {
        Self {
            current_round: 0,
            current_position: position,
            round_state: RoundState::new(),
            timer: DeterministicTimerWheel::new(),
            timeout_law: TimeoutLaw::new(10, 200),
            timeout_timer_id: None,
            round_started: false,
        }
    }

    /// Start a new round. Schedules a timeout.
    pub fn start_round(&mut self, round: u64) -> Result<(), &'static str> {
        if round < self.current_round {
            return Err("cannot start past round");
        }
        self.current_round = round;
        self.round_state.enter_round(round);
        self.round_started = true;

        // Cancel previous timeout if any
        if let Some(id) = self.timeout_timer_id.take() {
            self.timer.cancel(id);
        }

        // Schedule new timeout
        let timeout_round = self.timer.current_round() + self.timeout_law.current_timeout();
        self.timeout_timer_id = Some(self.timer.schedule(timeout_round));

        Ok(())
    }

    /// Advance the timer. Returns true if a timeout fired.
    pub fn tick(&mut self) -> PacemakerEvent {
        let fired = self.timer.advance();

        if fired.contains(&self.timeout_timer_id.unwrap_or(u64::MAX)) {
            self.timeout_timer_id = None;
            self.timeout_law.on_timeout();
            self.round_started = false;
            PacemakerEvent::Timeout {
                round: self.current_round,
                new_timeout: self.timeout_law.current_timeout(),
            }
        } else {
            PacemakerEvent::Tick
        }
    }

    /// Signal that progress was made (QC received). Resets timeout.
    pub fn on_progress(&mut self) {
        self.timeout_law.on_progress();
        if let Some(id) = self.timeout_timer_id.take() {
            self.timer.cancel(id);
        }
    }

    pub fn advance_position(&mut self, new_position: ChainPosition) {
        self.current_position = new_position;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacemakerEvent {
    Tick,
    Timeout { round: u64, new_timeout: u64 },
}
