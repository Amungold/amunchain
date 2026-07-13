/// Tracks accumulated drift across epochs and generations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporalDrift {
    pub epoch_drift: i64,
    pub generation_drift: i64,
    pub accumulated: u64,
}

impl TemporalDrift {
    #[must_use]
    pub const fn new(epoch_drift: i64, generation_drift: i64) -> Self {
        let abs_epoch = epoch_drift.unsigned_abs();
        let abs_gen = generation_drift.unsigned_abs();
        let accumulated = abs_epoch * 1000 + abs_gen;
        Self {
            epoch_drift,
            generation_drift,
            accumulated,
        }
    }
}
