/// Temporal drift between two civilizations' replay timelines.
#[derive(Debug, Clone)]
pub struct TemporalDrift {
    pub epoch_drift: i64,
    pub generation_drift: i64,
    pub accumulated_divergence: u64,
    pub can_reconcile: bool,
}

impl TemporalDrift {
    pub fn new(epoch_drift: i64, generation_drift: i64) -> Self {
        let abs_epoch = epoch_drift.unsigned_abs();
        let abs_gen = generation_drift.unsigned_abs();
        let accumulated = abs_epoch * 1000 + abs_gen;
        Self {
            epoch_drift,
            generation_drift,
            accumulated_divergence: accumulated,
            can_reconcile: accumulated < 10000,
        }
    }
}
