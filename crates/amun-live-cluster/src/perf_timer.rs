use std::time::Instant;

/// RAII performance timer. Prints when dropped if threshold exceeded.
pub struct PerfTimer {
    label: &'static str,
    start: Instant,
    threshold_us: u64,
}

impl PerfTimer {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            start: Instant::now(),
            threshold_us: 200,
        }
    }

    pub fn with_threshold(label: &'static str, threshold_us: u64) -> Self {
        Self {
            label,
            start: Instant::now(),
            threshold_us,
        }
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_micros() as u64;
        if elapsed > self.threshold_us {
            eprintln!("PERF {}: {}us", self.label, elapsed);
        }
    }
}
