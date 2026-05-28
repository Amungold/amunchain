#[cfg(test)]
mod tests {
    use amun_storage_kernel::{
        persistence::wal::{ReplayVerifier, WalEntry},
        Key256, SparseMerkleTree,
    };
    use std::fs::OpenOptions;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        for entry in entries {
            let bytes = entry.encode();
            let len = (bytes.len() as u32).to_le_bytes();
            file.write_all(&len)?;
            file.write_all(&bytes)?;
        }
        file.sync_all()?;
        Ok(())
    }

    #[test]
    fn replay_equivalence_passes() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");

        let key = Key256([1u8; 32]);
        let value = [42u8; 32];

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

        create_test_wal(wal_path.to_str().unwrap(), &[entry]).unwrap();

        let (replayed_root, count) =
            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();

        assert_eq!(count, 1);
        assert_eq!(replayed_root, root.0);
    }

    #[test]
    fn replay_detects_divergence() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test_bad.wal");

        let key = Key256([1u8; 32]);
        let value = [42u8; 32];

        let bad_root = [0xFFu8; 32];
        let entry = WalEntry::new(
            0,
            0x05,
            1,
            0,
            0,
            key.0.to_vec(),
            value.to_vec(),
            0,
            bad_root,
            [0u8; 32],
        );

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&wal_path)
            .unwrap();
        let bytes = entry.encode();
        let len = (bytes.len() as u32).to_le_bytes();
        file.write_all(&len).unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all().unwrap();

        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("replay divergence"));
    }

    #[test]
    fn replay_detects_epoch_regression() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test_epoch.wal");

        let key = Key256([1u8; 32]);
        let value = [42u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
        let root = tree.root();

        let e1 = WalEntry::new(
            0,
            0x05,
            1,
            5,
            0,
            key.0.to_vec(),
            value.to_vec(),
            0,
            root.0,
            [0u8; 32],
        );
        let e2 = WalEntry::new(
            1,
            0x05,
            2,
            3,
            0,
            key.0.to_vec(),
            value.to_vec(),
            0,
            root.0,
            e1.entry_hash,
        );

        create_test_wal(wal_path.to_str().unwrap(), &[e1, e2]).unwrap();

        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("epoch regression"));
    }
}
