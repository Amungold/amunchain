use crate::metadata::MetadataStore;
use crate::snapshot::SnapshotStore;
use crate::wal::{WalEntry, WriteAheadLog};
use amun_validator_api::error::{PlatformError, PlatformResult, RecoveryError, RecoveryErrorCode};

/// Recovery engine — replays WAL and restores state after a crash.
pub struct RecoveryEngine;

impl RecoveryEngine {
    /// Full recovery:
    /// 1. Find latest snapshot → restore (height, state_root)
    /// 2. Replay WAL entries AFTER snapshot height
    /// 3. Write final height to metadata
    pub fn recover(
        wal: &WriteAheadLog,
        snapshots: &SnapshotStore,
        metadata: &MetadataStore,
    ) -> PlatformResult<RecoveryResult> {
        let latest_snapshot = snapshots.latest()?;
        let snapshot_height = if let Some(ref path) = latest_snapshot {
            let (height, state_root) = snapshots.restore(path)?;
            metadata.set_u64("latest_height", height)?;
            metadata.set_bytes32("state_root", &state_root)?;
            height
        } else {
            0u64
        };

        let wal_entries = wal.replay()?;
        let entries_replayed = Self::apply_wal_entries(&wal_entries, snapshot_height, metadata)?;

        Ok(RecoveryResult {
            snapshot_height,
            entries_replayed,
            final_height: metadata.get_u64("latest_height")?,
            recovered: true,
        })
    }

    fn apply_wal_entries(
        entries: &[WalEntry],
        snapshot_height: u64,
        metadata: &MetadataStore,
    ) -> PlatformResult<u64> {
        let mut count = 0u64;
        for entry in entries {
            if entry.height > snapshot_height {
                if !entry.verify_checksum() {
                    return Err(PlatformError::Recovery(RecoveryError::new(
                        RecoveryErrorCode::WalCorrupted,
                        format!("Checksum mismatch at seq {}", entry.sequence),
                    )));
                }
                match entry.operation.as_str() {
                    "put_block" | "update_height" => {
                        metadata.set_u64("latest_height", entry.height)?;
                        count += 1;
                    }
                    "set_state_root" => {
                        let mut root = [0u8; 32];
                        let len = entry.payload.len().min(32);
                        root[..len].copy_from_slice(&entry.payload[..len]);
                        metadata.set_bytes32("state_root", &root)?;
                        count += 1;
                    }
                    _ => {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}

#[derive(Debug, Clone)]
pub struct RecoveryResult {
    pub snapshot_height: u64,
    pub entries_replayed: u64,
    pub final_height: u64,
    pub recovered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_recovery_wal_only() {
        let dir = test_dir();
        let wal_dir = dir.path().join("wal");
        std::fs::create_dir_all(&wal_dir).unwrap();
        std::fs::create_dir_all(dir.path().join("snapshots")).unwrap();
        let meta_dir = dir.path().join("metadata");
        std::fs::create_dir_all(&meta_dir).unwrap();

        let e1 = WalEntry::new(1, 1, "update_height".into(), vec![1]);
        let e2 = WalEntry::new(2, 2, "update_height".into(), vec![2]);
        let mut data = e1.encode();
        data.extend_from_slice(&e2.encode());
        std::fs::write(wal_dir.join("wal.log"), &data).unwrap();

        let wal = WriteAheadLog::new(&wal_dir).unwrap();
        let snaps = SnapshotStore::new(&dir.path().join("snapshots"));
        let meta = MetadataStore::new(&meta_dir).unwrap();
        let r = RecoveryEngine::recover(&wal, &snaps, &meta).unwrap();
        assert!(r.recovered);
        assert!(
            r.entries_replayed >= 1,
            "At least one entry should be replayed"
        );
    }

    #[test]
    fn test_recovery_skips_pre_snapshot() {
        let dir = test_dir();
        let snaps = SnapshotStore::new(&dir.path().join("snapshots"));
        snaps.create(10, [0xAA; 32]).unwrap();
        let mut wal = WriteAheadLog::new(&dir.path().join("wal")).unwrap();
        wal.open().unwrap();
        wal.writer()
            .unwrap()
            .append(5, "update_height", vec![5])
            .unwrap();
        wal.writer()
            .unwrap()
            .append(15, "update_height", vec![15])
            .unwrap();
        let meta = MetadataStore::new(&dir.path().join("metadata")).unwrap();
        let r = RecoveryEngine::recover(&wal, &snaps, &meta).unwrap();
        assert_eq!(r.entries_replayed, 1);
        assert_eq!(meta.get_u64("latest_height").unwrap(), 15);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn test_recovery_empty() {
        let dir = test_dir();
        let wal = WriteAheadLog::new(&dir.path().join("wal")).unwrap();
        let snaps = SnapshotStore::new(&dir.path().join("snapshots"));
        let meta = MetadataStore::new(&dir.path().join("metadata")).unwrap();
        assert!(
            RecoveryEngine::recover(&wal, &snaps, &meta)
                .unwrap()
                .recovered
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn test_corrupt_wal_stops_at_last_valid() {
        let dir = test_dir();
        std::fs::create_dir_all(dir.path().join("wal")).unwrap();
        let entry = WalEntry::new(1, 100, "update_height".into(), vec![1]);
        let mut data = entry.encode();
        data.extend_from_slice(b"GARBAGE");
        std::fs::write(dir.path().join("wal").join("wal.log"), &data).unwrap();
        let wal = WriteAheadLog::new(&dir.path().join("wal")).unwrap();
        let snaps = SnapshotStore::new(&dir.path().join("snapshots"));
        let meta = MetadataStore::new(&dir.path().join("metadata")).unwrap();
        let r = RecoveryEngine::recover(&wal, &snaps, &meta).unwrap();
        assert_eq!(r.entries_replayed, 1);
        assert_eq!(meta.get_u64("latest_height").unwrap(), 100);
        let _ = std::fs::remove_dir_all(dir);
    }
}
