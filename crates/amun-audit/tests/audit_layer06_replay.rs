#[cfg(test)]
mod audit_replay {
    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use std::fs::OpenOptions;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let mut file = f;
        for e in entries {
            let b = e.encode();
            file.write_all(&(b.len() as u32).to_le_bytes())?;
            file.write_all(&b)?;
        }
        file.sync_all()?;
        Ok(())
    }

    // CONST-REPLAY-001: Replay produces identical state
    #[test]
    fn replay001_equivalence() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("replay001.wal");

        let key = Key256([0x42u8; 32]);
        let value = [0x99u8; 32];
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
        create_wal(path.to_str().unwrap(), &[entry]).unwrap();

        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
        match result {
            Ok((replayed_root, count)) => {
                assert_eq!(
                    count, 1,
                    "CONST-REPLAY-001: Expected 1 frame, got {}",
                    count
                );
                assert_eq!(
                    replayed_root, root.0,
                    "CONST-REPLAY-001 VIOLATION: Replayed root diverges"
                );
            }
            Err(e) => panic!("CONST-REPLAY-001: Replay failed: {}", e),
        }
    }

    // CONST-REPLAY-002: Replay detects state root divergence
    #[test]
    fn replay002_divergence_detection() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("replay002.wal");

        let key = Key256([0x42u8; 32]);
        let value = [0x99u8; 32];
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
        create_wal(path.to_str().unwrap(), &[entry]).unwrap();

        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
        assert!(
            result.is_err(),
            "CONST-REPLAY-002 VIOLATION: Must detect divergence, got Ok"
        );
    }

    // CONST-REPLAY-003: Replay detects epoch regression
    #[test]
    fn replay003_epoch_regression() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("replay003.wal");

        let key = Key256([0x42u8; 32]);
        let value = [0x99u8; 32];
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
        create_wal(path.to_str().unwrap(), &[e1, e2]).unwrap();

        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
        assert!(
            result.is_err(),
            "CONST-REPLAY-003 VIOLATION: Must detect epoch regression, got Ok"
        );
    }
}
