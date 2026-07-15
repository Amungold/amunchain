use amun_validator_api::error::{PlatformError, PlatformResult, StorageError, StorageErrorCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const SNAPSHOT_MAGIC: &[u8; 4] = b"AMSN";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnapshotHeader {
    magic: [u8; 4],
    block_height: u64,
    state_root: [u8; 32],
    timestamp_secs: u64,
}

impl SnapshotHeader {
    fn new(block_height: u64, state_root: [u8; 32]) -> Self {
        SnapshotHeader {
            magic: *SNAPSHOT_MAGIC,
            block_height,
            state_root,
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }

    fn decode(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

pub struct SnapshotStore {
    path: PathBuf,
}

impl SnapshotStore {
    pub fn new(path: &Path) -> Self {
        let _ = fs::create_dir_all(path);
        SnapshotStore {
            path: path.to_path_buf(),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.path.exists()
    }

    /// Create a snapshot at a specific block height.
    pub fn create(&self, block_height: u64, state_root: [u8; 32]) -> PlatformResult<PathBuf> {
        let header = SnapshotHeader::new(block_height, state_root);
        let name = format!("snapshot-height-{:020}.snap", block_height);
        let full_path = self.path.join(&name);
        let data = header.encode();
        fs::write(&full_path, &data).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotFailed,
                format!("Snapshot write: {}", e),
            ))
        })?;
        Ok(full_path)
    }

    /// Restore state from a snapshot file. Returns (block_height, state_root).
    pub fn restore(&self, path: &Path) -> PlatformResult<(u64, [u8; 32])> {
        if !path.exists() {
            return Err(PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotFailed,
                format!("Snapshot not found: {}", path.display()),
            )));
        }
        let data = fs::read(path).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotFailed,
                format!("Snapshot read: {}", e),
            ))
        })?;
        let header = SnapshotHeader::decode(&data).ok_or_else(|| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotCorrupted,
                "Invalid snapshot header".into(),
            ))
        })?;
        if header.magic != *SNAPSHOT_MAGIC {
            return Err(PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotCorrupted,
                "Snapshot magic mismatch".into(),
            )));
        }
        Ok((header.block_height, header.state_root))
    }

    pub fn list(&self) -> PlatformResult<Vec<PathBuf>> {
        let mut snapshots = Vec::new();
        if !self.path.exists() {
            return Ok(snapshots);
        }
        for entry in fs::read_dir(&self.path).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::SnapshotFailed,
                format!("List: {}", e),
            ))
        })? {
            let entry = entry.map_err(|e| {
                PlatformError::Storage(StorageError::new(
                    StorageErrorCode::SnapshotFailed,
                    format!("Entry: {}", e),
                ))
            })?;
            snapshots.push(entry.path());
        }
        snapshots.sort_by_key(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.strip_prefix("snapshot-height-"))
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0)
        });
        Ok(snapshots)
    }

    pub fn latest(&self) -> PlatformResult<Option<PathBuf>> {
        let mut list = self.list()?;
        Ok(list.pop())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_restore() {
        let dir = tempfile::tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let path = store.create(10240, [0xAA; 32]).unwrap();
        assert!(path.exists());
        let (height, root) = store.restore(&path).unwrap();
        assert_eq!(height, 10240);
        assert_eq!(root, [0xAA; 32]);
    }

    #[test]
    fn test_magic_mismatch_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let path = store.create(1, [0u8; 32]).unwrap();
        std::fs::write(&path, b"CORRUPT").unwrap();
        assert!(store.restore(&path).is_err());
    }

    #[test]
    fn test_numeric_sort() {
        let dir = tempfile::tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        store.create(20, [0u8; 32]).unwrap();
        store.create(100, [0u8; 32]).unwrap();
        let list = store.list().unwrap();
        assert_eq!(list.len(), 2, "Should have exactly 2 snapshots");
        let latest = store.latest().unwrap().unwrap();
        assert!(
            latest.to_string_lossy().contains("100"),
            "Latest should be height 100"
        );
    }
}
