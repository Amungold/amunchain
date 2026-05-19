use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Pacemaker {
    pub current_round: u64,
    pub base_timeout: Duration,
    pub timeout_multiplier: f64,
    pub consecutive_timeouts: u64,
    pub max_rounds_without_progress: u64,
}

impl Pacemaker {
    pub fn new(base_timeout: Duration) -> Self {
        Self {
            current_round: 0,
            base_timeout,
            timeout_multiplier: 1.5,
            consecutive_timeouts: 0,
            max_rounds_without_progress: 100,
        }
    }

    pub fn advance_round(&mut self) {
        self.current_round += 1;
        self.consecutive_timeouts = 0;
    }

    pub fn on_timeout(&mut self) {
        self.consecutive_timeouts += 1;
        self.current_round += 1;
    }

    pub fn on_progress(&mut self, qc_round: u64) {
        if qc_round >= self.current_round {
            self.current_round = qc_round + 1;
            self.consecutive_timeouts = 0;
        }
    }

    pub fn current_timeout(&self) -> Duration {
        let multiplier = self.timeout_multiplier.powi(self.consecutive_timeouts as i32);
        let millis = (self.base_timeout.as_millis() as f64 * multiplier) as u64;
        Duration::from_millis(millis.min(60_000))
    }

    pub fn should_halt(&self) -> bool {
        self.consecutive_timeouts > self.max_rounds_without_progress
    }

    pub fn reset(&mut self) {
        self.current_round = 0;
        self.consecutive_timeouts = 0;
    }
}

impl Default for Pacemaker {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
    }
}
