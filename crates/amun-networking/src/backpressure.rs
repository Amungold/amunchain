const HIGH_WATERMARK_BYTES: usize = 16 * 1024 * 1024;
const LOW_WATERMARK_BYTES: usize = 8 * 1024 * 1024;
const HIGH_WATERMARK_COUNT: usize = 500;
const LOW_WATERMARK_COUNT: usize = 250;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressureState {
    Normal,
    Warning,
    Critical,
}

pub struct BackpressureManager {
    state: BackpressureState,
}

impl BackpressureManager {
    pub fn new() -> Self {
        Self {
            state: BackpressureState::Normal,
        }
    }

    pub fn check(&mut self, _queue_size: usize, pending_bytes: usize) -> BackpressureState {
        let new_state =
            if pending_bytes > HIGH_WATERMARK_BYTES || _queue_size > HIGH_WATERMARK_COUNT {
                BackpressureState::Critical
            } else if pending_bytes > LOW_WATERMARK_BYTES || _queue_size > LOW_WATERMARK_COUNT {
                BackpressureState::Warning
            } else {
                BackpressureState::Normal
            };

        let old_state = self.state;
        self.state = new_state;

        if old_state == BackpressureState::Critical && new_state == BackpressureState::Warning {
            self.state = BackpressureState::Critical;
        }

        self.state
    }

    pub fn should_drop(&self, data_size: usize, _queue_size: usize, pending_bytes: usize) -> bool {
        match self.state {
            BackpressureState::Normal => false,
            BackpressureState::Warning => pending_bytes + data_size > HIGH_WATERMARK_BYTES * 2,
            BackpressureState::Critical => true,
        }
    }

    pub fn state(&self) -> BackpressureState {
        self.state
    }
}
