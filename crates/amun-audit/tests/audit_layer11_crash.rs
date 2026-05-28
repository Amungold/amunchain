#[cfg(test)]
mod audit_crash {
    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use std::fs::OpenOptions;
    use std::io::Write;
    use tempfile::tempdir;

    // CONST-CRASH-001: Truncated WAL is detected as corrupted
    #[test]
    fn crash001_truncated_wal_detection() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("crash001.wal");

        let key = Key256([0x11u8; 32]);
        let value = [0x22u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
        let root = tree.root();

        let entry = WalEntry::new(
            0,
            0x05,
            1,
            0,
            0,
            key.0.to_vec(),
            value.to_vec(),
            0,
            root.0,
            [0u8; 32],
        );
        let encoded = entry.encode();
        let len = (encoded.len() as u32).to_le_bytes();
        let mut full = Vec::new();
        full.extend_from_slice(&len);
        full.extend_from_slice(&encoded);

        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        let mut file = f;
        file.write_all(&full[..full.len() - 5]).unwrap();
        file.sync_all().unwrap();

        let mut iter = WalIterator::new(path.to_str().unwrap());
        let entries: Vec<_> = iter.by_ref().collect();
        assert!(
            entries.is_empty() || iter.is_corrupted(),
            "CONST-CRASH-001 VIOLATION: Truncated WAL must be detected as corrupted"
        );
    }

    // CONST-CRASH-002: Partial frame is rejected
    #[test]
    fn crash002_partial_frame_rejection() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("crash002.wal");

        let partial = vec![0u8; 10];
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        let mut file = f;
        file.write_all(&partial).unwrap();
        file.sync_all().unwrap();

        let mut iter = WalIterator::new(path.to_str().unwrap());
        let entries: Vec<_> = iter.by_ref().collect();
        assert!(
            entries.is_empty() || iter.is_corrupted(),
            "CONST-CRASH-002 VIOLATION: Partial frame must produce no valid entries"
        );
    }

    // CONST-CRASH-003: Mid-frame byte corruption is detected
    #[test]
    fn crash003_mid_frame_corruption() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("crash003.wal");

        let key = Key256([0x33u8; 32]);
        let value = [0x44u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
        let root = tree.root();

        let entry = WalEntry::new(
            0,
            0x05,
            1,
            0,
            0,
            key.0.to_vec(),
            value.to_vec(),
            0,
            root.0,
            [0u8; 32],
        );
        let encoded = entry.encode();
        let len = (encoded.len() as u32).to_le_bytes();
        let mut full = Vec::new();
        full.extend_from_slice(&len);
        full.extend_from_slice(&encoded);

        if full.len() > 17 {
            full[17] ^= 0xFF;
        }

        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        let mut file = f;
        file.write_all(&full).unwrap();
        file.sync_all().unwrap();

        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
        assert!(
            result.is_err(),
            "CONST-CRASH-003 VIOLATION: Corrupted frame must fail replay"
        );
    }
}
