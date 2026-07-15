use amun_validator_api::error::{PlatformError, PlatformResult, SyncError, SyncErrorCode};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct SyncService {
    current_height: AtomicU64,
    target_height: AtomicU64,
    synced: AtomicBool,
}

impl Default for SyncService {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncService {
    pub fn new() -> Self {
        SyncService {
            current_height: AtomicU64::new(0),
            target_height: AtomicU64::new(0),
            synced: AtomicBool::new(false),
        }
    }
    pub fn start_sync(&self, from: u64, to: u64) -> PlatformResult<()> {
        if to < from {
            return Err(PlatformError::Sync(SyncError::new(
                SyncErrorCode::DivergentState,
                format!("{} < {}", to, from),
            )));
        }
        self.current_height.store(from, Ordering::SeqCst);
        self.target_height.store(to, Ordering::SeqCst);
        self.synced.store(false, Ordering::SeqCst);
        Ok(())
    }
    pub fn update_progress(&self, h: u64) {
        self.current_height.store(h, Ordering::SeqCst);
        if h >= self.target_height.load(Ordering::SeqCst) {
            self.synced.store(true, Ordering::SeqCst);
        }
    }
    pub fn is_synced(&self) -> bool {
        self.synced.load(Ordering::SeqCst)
    }
    pub fn progress(&self) -> (u64, u64) {
        (
            self.current_height.load(Ordering::SeqCst),
            self.target_height.load(Ordering::SeqCst),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sync() {
        let s = SyncService::new();
        s.start_sync(0, 100).unwrap();
        s.update_progress(100);
        assert!(s.is_synced());
    }
    #[test]
    fn test_reject() {
        assert!(SyncService::new().start_sync(100, 50).is_err());
    }
}
