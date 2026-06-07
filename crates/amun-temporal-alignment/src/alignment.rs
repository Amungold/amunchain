/// Tracks temporal alignment between two epochs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporalAlignment {
    pub local_epoch: u64,
    pub remote_epoch: u64,
    pub drift: i64,
    pub is_aligned: bool,
}

impl TemporalAlignment {
    #[must_use]
    pub const fn new(local_epoch: u64, remote_epoch: u64) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        let diff = remote_epoch as i64 - local_epoch as i64;
        Self {
            local_epoch,
            remote_epoch,
            drift: diff,
            is_aligned: diff == 0,
        }
    }
}
