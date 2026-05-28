/// Temporal alignment between two civilizations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalAlignment {
    pub local_epoch: u64,
    pub remote_epoch: u64,
    pub epoch_difference: i64,
    pub is_aligned: bool,
    pub requires_synchronization: bool,
}

impl TemporalAlignment {
    pub fn new(local_epoch: u64, remote_epoch: u64) -> Self {
        let diff = remote_epoch as i64 - local_epoch as i64;
        Self {
            local_epoch,
            remote_epoch,
            epoch_difference: diff,
            is_aligned: diff == 0,
            requires_synchronization: diff != 0,
        }
    }
}
