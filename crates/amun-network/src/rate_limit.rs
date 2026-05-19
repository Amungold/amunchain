use crate::constants::RATE_LIMIT_PER_SECOND;

pub struct RateLimiter {
    count: u32,
    window_start_ms: u64,
    limit: u32,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self { count: 0, window_start_ms: 0, limit: RATE_LIMIT_PER_SECOND }
    }
    pub fn allow(&mut self, now_ms: u64) -> bool {
        if now_ms.saturating_sub(self.window_start_ms) >= 1000 {
            self.count = 0;
            self.window_start_ms = now_ms;
        }
        if self.count >= self.limit {
            return false;
        }
        self.count = self.count.saturating_add(1);
        true
    }
}
