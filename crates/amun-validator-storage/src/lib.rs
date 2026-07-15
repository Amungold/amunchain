pub mod metadata;
pub mod recovery;
pub mod snapshot;
pub mod storage;
pub mod wal;

pub use metadata::MetadataStore;
pub use recovery::RecoveryEngine;
pub use snapshot::SnapshotStore;
pub use storage::StorageService;
pub use wal::{WalEntry, WalWriter, WriteAheadLog};

use amun_validator_api::error::PlatformResult;
use std::path::Path;

/// Mainnet-grade storage provider trait.
pub trait StorageProvider: Send + Sync {
    fn latest_height(&self) -> PlatformResult<u64>;
    fn state_root(&self) -> PlatformResult<[u8; 32]>;
    fn is_healthy(&self) -> PlatformResult<bool>;
    fn create_snapshot(&self, block_height: u64) -> PlatformResult<std::path::PathBuf>;
    fn restore_snapshot(&self, path: &Path) -> PlatformResult<()>;
    fn verify_integrity(&self) -> PlatformResult<()>;
    fn flush(&self) -> PlatformResult<()>;
    fn sync(&self) -> PlatformResult<()>;
    fn shutdown(&self) -> PlatformResult<()>;
}
