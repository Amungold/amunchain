#[derive(Debug, Clone)]
pub struct TimeoutLaw {
    pub base_timeout_rounds: u64,
    pub max_timeout_rounds: u64,
    pub current_timeout_rounds: u64,
    pub consecutive_timeouts: u64,
}

impl TimeoutLaw {
    pub fn new(base_timeout_rounds: u64, max_timeout_rounds: u64) -> Self {
        Self {
            base_timeout_rounds,
            max_timeout_rounds,
            current_timeout_rounds: base_timeout_rounds,
            consecutive_timeouts: 0,
        }
    }

    pub fn on_timeout(&mut self) -> u64 {
        self.consecutive_timeouts += 1;
        let exponent = self.consecutive_timeouts.min(6);
        let multiplier = 1u64.checked_shl(exponent as u32).unwrap_or(64);
        self.current_timeout_rounds = self
            .base_timeout_rounds
            .saturating_mul(multiplier)
            .min(self.max_timeout_rounds);
        self.current_timeout_rounds
    }

    pub fn on_progress(&mut self) {
        self.consecutive_timeouts = 0;
        self.current_timeout_rounds = self.base_timeout_rounds;
    }

    pub fn current_timeout(&self) -> u64 {
        self.current_timeout_rounds
    }
}
