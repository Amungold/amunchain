// Stream configuration and state management
// Resumable sync: track progress and resume from last verified chunk.

pub struct StreamConfig {
    pub max_chunks_in_flight: u64,
    pub chunk_timeout_ms: u64,
    pub max_retries: u64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            max_chunks_in_flight: 4,
            chunk_timeout_ms: 30000,
            max_retries: 3,
        }
    }
}

pub struct StreamState {
    pub total_chunks: u64,
    pub received_chunks: u64,
    pub verified_chunks: u64,
    pub last_verified_index: i64,
    pub current_root: Option<[u8; 32]>,
    pub is_complete: bool,
}

impl StreamState {
    pub fn new(total_chunks: u64) -> Self {
        Self {
            total_chunks,
            received_chunks: 0,
            verified_chunks: 0,
            last_verified_index: -1,
            current_root: None,
            is_complete: false,
        }
    }

    pub fn progress_percent(&self) -> f64 {
        if self.total_chunks == 0 {
            100.0
        } else {
            (self.verified_chunks as f64 / self.total_chunks as f64) * 100.0
        }
    }

    pub fn can_resume_from(&self) -> u64 {
        (self.last_verified_index + 1) as u64
    }
}
