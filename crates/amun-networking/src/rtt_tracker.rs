use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Per-peer RTT tracker using EWMA (Exponential Weighted Moving Average).
pub struct RttTracker {
    /// Smoothed RTT in microseconds (EWMA)
    pub smoothed_rtt_us: AtomicU64,
    /// RTT variance in microseconds
    pub rtt_variance_us: AtomicU64,
    /// Number of samples collected
    pub samples: AtomicU64,
    /// Timestamp of last sent ping (sequence → timestamp)
    last_ping_time: parking_lot::Mutex<Option<(u64, Instant)>>,
}

impl RttTracker {
    pub fn new() -> Self {
        Self {
            smoothed_rtt_us: AtomicU64::new(500_000), // Default: 500ms
            rtt_variance_us: AtomicU64::new(100_000), // Default: 100ms
            samples: AtomicU64::new(0),
            last_ping_time: parking_lot::Mutex::new(None),
        }
    }

    /// Record that a ping was sent with this sequence number.
    pub fn ping_sent(&self, seq: u64) {
        *self.last_ping_time.lock() = Some((seq, Instant::now()));
    }

    /// Record a pong response. Returns the measured RTT in microseconds.
    pub fn pong_received(&self, seq: u64) -> Option<u64> {
        let mut lock = self.last_ping_time.lock();
        if let Some((sent_seq, sent_time)) = *lock {
            if sent_seq == seq {
                let rtt_us = sent_time.elapsed().as_micros() as u64;
                self.update(rtt_us);
                *lock = None;
                return Some(rtt_us);
            }
        }
        None
    }

    /// Update smoothed RTT using EWMA.
    /// alpha = 0.125 (standard TCP smoothing factor)
    fn update(&self, measured_rtt_us: u64) {
        let alpha = 0.125_f64;
        let old = self.smoothed_rtt_us.load(Ordering::Relaxed) as f64;
        let new = (1.0 - alpha) * old + alpha * measured_rtt_us as f64;
        self.smoothed_rtt_us.store(new as u64, Ordering::Relaxed);

        // Update variance
        let old_var = self.rtt_variance_us.load(Ordering::Relaxed) as f64;
        let diff = (measured_rtt_us as f64 - old).abs();
        let new_var = (1.0 - alpha) * old_var + alpha * diff;
        self.rtt_variance_us
            .store(new_var as u64, Ordering::Relaxed);

        self.samples.fetch_add(1, Ordering::Relaxed);
    }

    /// Get adaptive timeout: smoothed_rtt + 4 * variance
    pub fn timeout_us(&self) -> u64 {
        let rtt = self.smoothed_rtt_us.load(Ordering::Relaxed);
        let var = self.rtt_variance_us.load(Ordering::Relaxed);
        rtt.saturating_add(var.saturating_mul(4))
    }
}
