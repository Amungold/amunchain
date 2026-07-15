use crate::error::PlatformResult;

pub trait SyncProvider: Send + Sync {
    fn is_synced(&self) -> PlatformResult<bool>;
    fn sync_progress(&self) -> PlatformResult<(u64, u64)>;
    fn start_sync(&self, from: u64, to: u64) -> PlatformResult<()>;
    fn verify_state(&self) -> PlatformResult<()>;
}
