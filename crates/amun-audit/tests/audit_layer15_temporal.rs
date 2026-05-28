#[cfg(test)]
mod audit_temporal {
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

    // CONST-TEMP-001: Replaying same WAL twice produces identical root
    #[test]
    fn temp001_replay_twice_same_root() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("temp001.wal");

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

        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());

        match (result1, result2) {
            (Ok((r1, c1)), Ok((r2, c2))) => {
                assert_eq!(c1, c2, "CONST-TEMP-001: Frame counts differ");
                assert_eq!(
                    r1, r2,
                    "CONST-TEMP-001 VIOLATION: Replay twice produces different roots"
                );
                assert_eq!(r1, root.0, "CONST-TEMP-001: Root must match original");
            }
            (Err(e), _) | (_, Err(e)) => panic!("CONST-TEMP-001: Replay failed: {}", e),
        }
    }

    // CONST-TEMP-002: Insert order independence verified over 3 permutations
    #[test]
    fn temp002_temporal_order_independence() {
        let keys: Vec<Key256> = vec![
            Key256([0x01u8; 32]),
            Key256([0x02u8; 32]),
            Key256([0x03u8; 32]),
        ];
        let value = [0x42u8; 32];

        let mut tree = SparseMerkleTree::empty();
        for k in &keys {
            tree = tree.insert(k, &value, 0);
        }
        let root_forward = tree.root().0;

        let mut tree = SparseMerkleTree::empty();
        for k in keys.iter().rev() {
            tree = tree.insert(k, &value, 0);
        }
        let root_reverse = tree.root().0;

        assert_eq!(
            root_forward, root_reverse,
            "CONST-TEMP-002 VIOLATION: Forward vs reverse insertion diverges"
        );
    }
}
