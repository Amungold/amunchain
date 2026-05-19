use crate::constants::HEARTBEAT_INTERVAL_MS;

pub struct Heartbeat {
    last_sent_ms: u64,
    last_received_ms: u64,
    interval_ms: u64,
    missed_count: u32,
}

impl Heartbeat {
    pub fn new() -> Self {
        Self { last_sent_ms: 0, last_received_ms: 0, interval_ms: HEARTBEAT_INTERVAL_MS, missed_count: 0 }
    }
    pub fn should_send(&self, now_ms: u64) -> bool {
        now_ms.saturating_sub(self.last_sent_ms) >= self.interval_ms
    }
    pub fn sent(&mut self, now_ms: u64) {
        self.last_sent_ms = now_ms;
    }
    pub fn received(&mut self, now_ms: u64) {
        self.last_received_ms = now_ms;
        self.missed_count = 0;
    }
    pub fn check_timeout(&mut self, now_ms: u64, timeout_ms: u64) -> bool {
        if now_ms.saturating_sub(self.last_received_ms) >= timeout_ms {
            self.missed_count = self.missed_count.saturating_add(1);
            return true;
        }
        false
    }
}
