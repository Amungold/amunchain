use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct GlobalRateLimiter {
    max_messages_per_sec: u64,
    max_bytes_per_sec: u64,
    message_count: AtomicU64,
    byte_count: AtomicU64,
    last_reset: std::sync::Mutex<Instant>,
}

impl GlobalRateLimiter {
    pub fn new(max_messages_per_sec: u64, max_bytes_per_sec: u64) -> Self {
        Self {
            max_messages_per_sec,
            max_bytes_per_sec,
            message_count: AtomicU64::new(0),
            byte_count: AtomicU64::new(0),
            last_reset: std::sync::Mutex::new(Instant::now()),
        }
    }

    pub fn try_consume(&self, bytes: usize) -> bool {
        let now = Instant::now();
        let mut last_reset = self.last_reset.lock().unwrap();

        if now.duration_since(*last_reset).as_secs() >= 1 {
            self.message_count.store(0, Ordering::Relaxed);
            self.byte_count.store(0, Ordering::Relaxed);
            *last_reset = now;
        }

        let msg_count = self.message_count.fetch_add(1, Ordering::Relaxed);
        let byte_total = self.byte_count.fetch_add(bytes as u64, Ordering::Relaxed);

        msg_count < self.max_messages_per_sec && byte_total < self.max_bytes_per_sec
    }

    pub fn current_rate(&self) -> (u64, u64) {
        (
            self.message_count.load(Ordering::Relaxed),
            self.byte_count.load(Ordering::Relaxed),
        )
    }
}
