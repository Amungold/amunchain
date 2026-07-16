use std::sync::atomic::{AtomicU64, Ordering};

/// Fault injection mode for R2.3 testing.
/// Deterministic — no external randomness.
#[derive(Clone, Debug)]
pub enum FaultMode {
    /// No faults injected.
    None,
    /// Drop messages with a given percentage (0..100).
    Drop(u8),
    /// Delay messages with a given percentage, for a duration in ms.
    Delay {
        percent: u8,
        min_ms: u64,
        max_ms: u64,
    },
    /// Reorder messages: buffer some, release in LIFO order.
    Reorder { percent: u8, buffer_size: usize },
    /// Duplicate messages: send original + N extra copies.
    Duplicate { percent: u8, count: u8 },
    /// Corrupt message content.
    Corrupt { percent: u8, kind: CorruptKind },
    /// Equivocate: send conflicting votes for same height.
    Equivocate { percent: u8 },
}

/// Type of corruption to apply to a message payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CorruptKind {
    /// Zero out the signature field.
    InvalidSignature,
    /// Flip a single byte in the payload.
    BitFlip,
    /// Replace the height field with a wrong value.
    WrongHeight,
    /// Replace the block hash with zeros.
    WrongBlockHash,
    /// Truncate the payload to half its size.
    Truncated,
}

/// Thread-safe fault injector.
/// Shared across all validator adapters in test mode.
#[derive(Debug)]
pub struct FaultInjector {
    mode: FaultMode,
    counter: AtomicU64,
}

impl FaultInjector {
    /// Create a fault injector with no faults.
    pub fn disabled() -> Self {
        Self {
            mode: FaultMode::None,
            counter: AtomicU64::new(0),
        }
    }

    /// Create a fault injector that drops `percent` of messages.
    pub fn drop(percent: u8) -> Self {
        assert!(percent <= 100, "Drop percent must be 0..100");
        Self {
            mode: FaultMode::Drop(percent),
            counter: AtomicU64::new(0),
        }
    }

    /// Create a fault injector that delays `percent` of messages.
    /// Delay duration is deterministic: (counter * prime) % (max_ms - min_ms) + min_ms
    pub fn delay(percent: u8, min_ms: u64, max_ms: u64) -> Self {
        assert!(percent <= 100, "Delay percent must be 0..100");
        assert!(min_ms <= max_ms, "min_ms must be <= max_ms");
        Self {
            mode: FaultMode::Delay {
                percent,
                min_ms,
                max_ms,
            },
            counter: AtomicU64::new(0),
        }
    }

    /// Create a fault injector that duplicates `percent` of messages `count` extra times.
    pub fn duplicate(percent: u8, count: u8) -> Self {
        assert!(percent <= 100, "Duplicate percent must be 0..100");
        assert!(count > 0, "Duplicate count must be > 0");
        Self {
            mode: FaultMode::Duplicate { percent, count },
            counter: AtomicU64::new(0),
        }
    }

    /// Create a fault injector that corrupts `percent` of messages.
    pub fn corrupt(percent: u8, kind: CorruptKind) -> Self {
        assert!(percent <= 100, "Corrupt percent must be 0..100");
        Self {
            mode: FaultMode::Corrupt { percent, kind },
            counter: AtomicU64::new(0),
        }
    }

    /// Create a fault injector that sends equivocating votes.
    pub fn equivocate(percent: u8) -> Self {
        assert!(percent <= 100, "Equivocate percent must be 0..100");
        Self {
            mode: FaultMode::Equivocate { percent },
            counter: AtomicU64::new(0),
        }
    }

    /// Returns true if the current message should be dropped.

    /// Create a fault injector that reorders `percent` of messages.
    pub fn reorder(percent: u8, buffer_size: usize) -> Self {
        assert!(percent <= 100, "Reorder percent must be 0..100");
        assert!(buffer_size > 0, "Buffer size must be > 0");
        Self {
            mode: FaultMode::Reorder {
                percent,
                buffer_size,
            },
            counter: AtomicU64::new(0),
        }
    }

    pub fn should_drop(&self) -> bool {
        match self.mode {
            FaultMode::None => false,
            FaultMode::Drop(percent) => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed) % 100;
                n < percent as u64
            }
            FaultMode::Delay { .. }
            | FaultMode::Reorder { .. }
            | FaultMode::Duplicate { .. }
            | FaultMode::Corrupt { .. }
            | FaultMode::Equivocate { .. } => {
                // Delay, Reorder, Duplicate, Corrupt, Equivocate don't drop.
                let _ = self.counter.fetch_add(1, Ordering::Relaxed);
                false
            }
        }
    }

    /// Returns Some(delay_ms) if this message should be delayed, None otherwise.
    /// Uses deterministic formula: (counter * 2654435761) % (max_ms - min_ms + 1) + min_ms
    pub fn should_delay(&self) -> Option<u64> {
        match self.mode {
            FaultMode::None => None,
            FaultMode::Drop(_) => None,
            FaultMode::Delay {
                percent,
                min_ms,
                max_ms,
            } => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed);
                let threshold = n % 100;
                if threshold < percent as u64 {
                    // Deterministic delay: multiply by a large prime, take modulo range
                    let range = max_ms - min_ms + 1;
                    let delay = (n.wrapping_mul(2654435761)) % range + min_ms;
                    Some(delay)
                } else {
                    None
                }
            }
            FaultMode::Reorder { .. }
            | FaultMode::Duplicate { .. }
            | FaultMode::Corrupt { .. }
            | FaultMode::Equivocate { .. } => {
                let _ = self.counter.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    /// Returns Some(buffer_size) if this message should be buffered for reordering.
    pub fn should_reorder(&self) -> Option<usize> {
        match self.mode {
            FaultMode::Reorder {
                percent,
                buffer_size,
            } => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed);
                if (n % 100) < percent as u64 {
                    Some(buffer_size)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns Some(count) if this message should be duplicated (extra copies).
    pub fn should_duplicate(&self) -> Option<u8> {
        match self.mode {
            FaultMode::Duplicate { percent, count } => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed);
                if (n % 100) < percent as u64 {
                    Some(count)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns Some(CorruptKind) if this message should be corrupted.
    pub fn should_corrupt(&self) -> Option<CorruptKind> {
        match self.mode {
            FaultMode::Corrupt { percent, ref kind } => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed);
                if (n % 100) < percent as u64 {
                    Some(kind.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns true if this vote should trigger an equivocation.
    pub fn should_equivocate(&self) -> bool {
        match self.mode {
            FaultMode::Equivocate { percent } => {
                let n = self.counter.fetch_add(1, Ordering::Relaxed);
                (n % 100) < percent as u64
            }
            _ => false,
        }
    }

    /// Returns the current fault mode (for diagnostics).
    pub fn mode(&self) -> &FaultMode {
        &self.mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_never_drops() {
        let f = FaultInjector::disabled();
        for _ in 0..1000 {
            assert!(!f.should_drop());
        }
    }

    #[test]
    fn drop_50_percent_is_deterministic() {
        let f1 = FaultInjector::drop(50);
        let f2 = FaultInjector::drop(50);
        // Same seed pattern should produce same sequence
        for _ in 0..100 {
            assert_eq!(f1.should_drop(), f2.should_drop());
        }
    }

    #[test]
    fn drop_0_never_drops() {
        let f = FaultInjector::drop(0);
        for _ in 0..1000 {
            assert!(!f.should_drop());
        }
    }

    #[test]
    fn drop_100_always_drops() {
        let f = FaultInjector::drop(100);
        for _ in 0..100 {
            assert!(f.should_drop());
        }
    }
}
