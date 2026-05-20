/// Deterministic delivery schedule for network messages.
#[derive(Debug, Clone)]
pub struct DeliverySchedule {
    pub base_latency_rounds: u64,
    pub jitter_rounds: u64,
}

impl DeliverySchedule {
    pub fn new(base_latency_rounds: u64, jitter_rounds: u64) -> Self {
        Self {
            base_latency_rounds,
            jitter_rounds,
        }
    }

    pub fn latency_for(&self, sender: u64, receiver: u64) -> u64 {
        // Deterministic latency based on sender/receiver pair
        let base = self.base_latency_rounds;
        let jitter = sender.wrapping_mul(receiver) % self.jitter_rounds.max(1);
        base + jitter
    }
}
