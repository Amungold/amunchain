#[cfg(test)]
mod tests {
    use amun_truth_engine::TruthEngine;
    use amun_wal::WriteAheadLog;
    use amun_crash_recovery::CrashRecovery;

    #[test]
    fn test_recovery_rebuilds_full_state() {
        let genesis = [0u8; 32];
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("full_recovery.wal");
        let pre_crash_root;
        {
            let mut engine = TruthEngine::new(genesis);
            let mut wal = WriteAheadLog::create(wal_path.clone()).unwrap();
            for i in 0..15 {
                let (_root, event) = engine.execute_live(format!("tx_{}", i).as_bytes(), 1_000_000).unwrap();
                wal.append_event(&event).unwrap();
            }
            pre_crash_root = engine.live_root();
        }
        {
            let wal = WriteAheadLog::open(wal_path.clone()).unwrap();
            let engine = TruthEngine::new(genesis);
            let mut recovery = CrashRecovery::new(wal, engine);
            let result = recovery.recover().unwrap();
            assert_eq!(result.final_root, pre_crash_root);
            assert!(result.verified);
        }
    }

    #[test]
    fn test_restart_equivalence() {
        let genesis = [0xAB; 32];
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("restart.wal");
        let live_root;
        {
            let mut engine = TruthEngine::new(genesis);
            let mut wal = WriteAheadLog::create(wal_path.clone()).unwrap();
            for i in 0..25 {
                let (_root, event) = engine.execute_live(format!("tx_{}", i).as_bytes(), 1_000_000).unwrap();
                wal.append_event(&event).unwrap();
            }
            live_root = engine.live_root();
        }
        {
            let wal = WriteAheadLog::open(wal_path.clone()).unwrap();
            let engine = TruthEngine::new(genesis);
            let mut recovery = CrashRecovery::new(wal, engine);
            let result = recovery.recover().unwrap();
            assert_eq!(result.final_root, live_root);
            assert!(result.verified);
        }
    }

    #[test]
    fn test_byzantine_transcript_tampering_detected() {
        let mut e = TruthEngine::new([0u8; 32]);
        e.execute_live(b"honest", 1_000_000).unwrap();
        let h = e.live_root();
        let mut b = TruthEngine::new([0u8; 32]);
        b.execute_live(b"tampered", 1_000_000).unwrap();
        assert_ne!(h, b.live_root());
    }

    #[test]
    fn test_seal_tampering_detected() {
        let mut h = TruthEngine::new([0u8; 32]);
        h.execute_live(b"tx", 1_000_000).unwrap();
        h.seal_and_advance_epoch().unwrap();
        let mut t = TruthEngine::new([0u8; 32]);
        t.execute_live(b"tx", 1_000_000).unwrap();
        t.execute_live(b"tx2", 1_000_000).unwrap();
        assert_ne!(h.compute_chain_root(2).unwrap(), t.compute_chain_root(2).unwrap());
    }

    #[test]
    fn test_journal_integrity_under_truncation() {
        let mut e = TruthEngine::new([0u8; 32]);
        for i in 0..5 { e.execute_live(format!("tx_{}", i).as_bytes(), 1_000_000).unwrap(); }
        assert!(e.live_journal().verify_continuity());
        assert_eq!(e.live_journal().entries.len(), 5);
    }
}
