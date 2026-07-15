use crate::error::PlatformResult;

pub trait RecoveryProvider: Send + Sync {
    fn last_checkpoint(&self) -> PlatformResult<u64>;
    fn replay_wal(&self) -> PlatformResult<()>;
    fn recover_state(&self) -> PlatformResult<()>;
    fn verify_after_recovery(&self) -> PlatformResult<()>;
}
