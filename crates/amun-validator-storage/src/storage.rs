use crate::metadata::MetadataStore;
use crate::recovery::RecoveryEngine;
use crate::snapshot::SnapshotStore;
use crate::wal::WriteAheadLog;
use crate::StorageProvider;
use amun_validator_api::error::PlatformResult;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Mainnet-grade storage service — single source of truth.
/// All components injected via constructor (Dependency Injection).
pub struct StorageService {
    data_dir: PathBuf,
    wal: Arc<WriteAheadLog>,
    snapshots: Arc<SnapshotStore>,
    metadata: Arc<MetadataStore>,
}

impl StorageService {
    pub fn new(
        data_dir: PathBuf,
        wal: Arc<WriteAheadLog>,
        snapshots: Arc<SnapshotStore>,
        metadata: Arc<MetadataStore>,
    ) -> Self {
        StorageService {
            data_dir,
            wal,
            snapshots,
            metadata,
        }
    }

    pub fn wal(&self) -> &Arc<WriteAheadLog> {
        &self.wal
    }
    pub fn snapshots(&self) -> &Arc<SnapshotStore> {
        &self.snapshots
    }
    pub fn metadata(&self) -> &Arc<MetadataStore> {
        &self.metadata
    }
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
    pub fn recover(&self) -> PlatformResult<crate::recovery::RecoveryResult> {
        RecoveryEngine::recover(&self.wal, &self.snapshots, &self.metadata)
    }
}

impl StorageProvider for StorageService {
    fn latest_height(&self) -> PlatformResult<u64> {
        self.metadata.get_u64("latest_height")
    }
    fn state_root(&self) -> PlatformResult<[u8; 32]> {
        self.metadata.get_bytes32("state_root")
    }
    fn is_healthy(&self) -> PlatformResult<bool> {
        Ok(self.wal.is_healthy() && self.snapshots.is_healthy() && self.metadata.is_healthy())
    }
    fn create_snapshot(&self, block_height: u64) -> PlatformResult<PathBuf> {
        self.snapshots.create(block_height, self.state_root()?)
    }
    fn restore_snapshot(&self, path: &Path) -> PlatformResult<()> {
        let (h, r) = self.snapshots.restore(path)?;
        self.metadata.set_u64("latest_height", h)?;
        self.metadata.set_bytes32("state_root", &r)?;
        Ok(())
    }
    fn verify_integrity(&self) -> PlatformResult<()> {
        self.wal.verify()?;
        self.metadata.verify()?;
        if let Some(ref p) = self.snapshots.latest()? {
            let _ = self.snapshots.restore(p)?;
        }
        Ok(())
    }
    fn flush(&self) -> PlatformResult<()> {
        if let Ok(w) = self.wal.writer() {
            w.sync()?;
        }
        Ok(())
    }
    fn sync(&self) -> PlatformResult<()> {
        self.flush()
    }
    fn shutdown(&self) -> PlatformResult<()> {
        self.flush()?;
        self.sync()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn svc(dir: &Path) -> StorageService {
        let _ = std::fs::create_dir_all(dir);
        StorageService::new(
            dir.to_path_buf(),
            Arc::new(WriteAheadLog::new(&dir.join("wal")).unwrap()),
            Arc::new(SnapshotStore::new(&dir.join("snapshots"))),
            Arc::new(MetadataStore::new(&dir.join("metadata")).unwrap()),
        )
    }

    #[test]
    fn test_healthy() {
        let d = tempfile::tempdir().unwrap();
        assert!(svc(d.path()).is_healthy().unwrap());
    }
    #[test]
    fn test_metadata() {
        let d = tempfile::tempdir().unwrap();
        let s = svc(d.path());
        s.metadata.set_u64("latest_height", 42).unwrap();
        assert_eq!(s.latest_height().unwrap(), 42);
    }
    #[test]
    fn test_snapshot() {
        let d = tempfile::tempdir().unwrap();
        let s = svc(d.path());
        s.metadata.set_u64("h", 5000).unwrap();
        s.metadata.set_bytes32("state_root", &[0xAA; 32]).unwrap();
        let p = s.create_snapshot(5000).unwrap();
        assert!(p.exists());
    }
    #[test]
    fn test_recovery() {
        let d = tempfile::tempdir().unwrap();
        assert!(svc(d.path()).recover().unwrap().recovered);
    }
    #[test]
    fn test_flush() {
        let d = tempfile::tempdir().unwrap();
        assert!(svc(d.path()).flush().is_ok());
    }
    #[test]
    fn test_shutdown() {
        let d = tempfile::tempdir().unwrap();
        assert!(svc(d.path()).shutdown().is_ok());
    }
}
