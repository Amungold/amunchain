use crate::error::PlatformResult;

pub trait StorageProvider: Send + Sync {
    fn latest_height(&self) -> PlatformResult<u64>;
    fn state_root(&self) -> PlatformResult<[u8; 32]>;
    fn is_healthy(&self) -> PlatformResult<bool>;
    fn create_snapshot(&self) -> PlatformResult<String>;
    fn verify_integrity(&self) -> PlatformResult<()>;
}
