#[cfg(test)]
mod tests {
    use amun_truth_engine::TruthEngine;
    use amun_wal::WriteAheadLog;
    use amun_crash_recovery::CrashRecovery;

    #[test]
    fn test_event_sourced_replay_equivalence() {
        let genesis = [0x12; 32];
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("event_sourced.wal");
        let root_a;
        {
            let mut engine = TruthEngine::new(genesis);
            let mut wal = WriteAheadLog::create(wal_path.clone()).unwrap();
            for i in 0..50 {
                let (_root, event) = engine.execute_live(format!("tx_{}", i).as_bytes(), 1_000_000).unwrap();
                wal.append_event(&event).unwrap();
            }
            root_a = engine.live_root();
        }
        let root_b;
        {
            let wal = WriteAheadLog::open(wal_path.clone()).unwrap();
            let engine = TruthEngine::new(genesis);
            let mut recovery = CrashRecovery::new(wal, engine);
            let result = recovery.recover().unwrap();
            root_b = result.final_root;
            assert!(result.verified);
        }
        assert_eq!(root_a, root_b, "Event-sourced: roots must be identical");
    }

    #[test]
    fn test_recovery_path_convergence() {
        let genesis = [0x9A; 32];
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("converge.wal");
        let direct_root;
        {
            let mut engine = TruthEngine::new(genesis);
            for i in 0..25 {
                engine.execute_live(format!("direct_{}", i).as_bytes(), 1_000_000).unwrap();
            }
            direct_root = engine.live_root();
        }
        let wal_root;
        {
            let mut wal = WriteAheadLog::create(wal_path.clone()).unwrap();
            let mut temp_engine = TruthEngine::new(genesis);
            for i in 0..25 {
                let (_root, event) = temp_engine.execute_live(format!("direct_{}", i).as_bytes(), 1_000_000).unwrap();
                wal.append_event(&event).unwrap();
            }
            let wal = WriteAheadLog::open(wal_path).unwrap();
            let engine = TruthEngine::new(genesis);
            let mut recovery = CrashRecovery::new(wal, engine);
            let result = recovery.recover().unwrap();
            wal_root = result.final_root;
        }
        assert_eq!(direct_root, wal_root, "Direct and WAL recovery must converge");
    }
}
