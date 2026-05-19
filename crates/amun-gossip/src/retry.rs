use crate::constants::GOSSIP_TIMEOUT_MS;

pub struct RetryManager {
    retry_count: u32,
    max_retries: u32,
    timeout_ms: u64,
}

impl RetryManager {
    pub fn new() -> Self {
        Self { retry_count: 0, max_retries: 5, timeout_ms: GOSSIP_TIMEOUT_MS }
    }
    pub fn should_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
    pub fn retry(&mut self) {
        self.retry_count = self.retry_count.saturating_add(1);
    }
    pub fn reset(&mut self) {
        self.retry_count = 0;
    }
    pub fn backoff_ms(&self) -> u64 {
        self.timeout_ms * (1u64 << self.retry_count.min(6))
    }
}
