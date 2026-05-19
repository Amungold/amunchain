#[cfg(test)]
mod tests {
    use amun_truth_engine::TruthEngine;
    use amun_replay_certificate::ReplayCertificate;
    use amun_chain_position::ChainPosition;

    fn exec(e: &mut TruthEngine, data: &[u8]) -> [u8; 32] {
        e.execute_live(data, 1_000_000).unwrap().0
    }

    #[test]
    fn test_replay_idempotent() {
        let mut e = TruthEngine::new([0u8; 32]);
        for i in 0..50 { e.record_message(format!("tx_{}", i).as_bytes()).unwrap(); }
        let a = e.compute_chain_root(50).unwrap();
        let b = e.compute_chain_root(50).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_live_equals_replay() {
        let mut e = TruthEngine::new([0u8; 32]);
        for i in 0..100 { exec(&mut e, format!("tx_{}", i).as_bytes()); }
        let live = e.live_root();
        let replay = e.compute_chain_root(100).unwrap();
        assert_eq!(live, replay);
    }

    #[test]
    fn test_snapshot_export_import_roundtrip() {
        let mut e = TruthEngine::new([0xAA; 32]);
        for i in 0..30 { exec(&mut e, format!("tx_{}", i).as_bytes()); }
        let root_before = e.live_root();

        let snapshot = e.export_snapshot().unwrap();
        assert!(snapshot.verify());

        let mut e2 = TruthEngine::new([0xAA; 32]);
        e2.import_snapshot(&snapshot).unwrap();
        assert_eq!(e2.live_root(), root_before);
    }

    #[test]
    fn test_snapshot_rejects_foreign_genesis() {
        let mut e = TruthEngine::new([0xAA; 32]);
        for i in 0..20 { exec(&mut e, format!("tx_{}", i).as_bytes()); }

        let snapshot = e.export_snapshot().unwrap();

        // Try to import into engine with different genesis
        let mut e2 = TruthEngine::new([0xBB; 32]);
        let result = e2.import_snapshot(&snapshot);
        assert!(result.is_err(), "Foreign genesis must be rejected");
    }

    #[test]
    fn test_replay_certificate_self_verifying() {
        let cert = ReplayCertificate::new(
            [0x11; 32], [0x22; 32],
            ChainPosition::genesis(),
            ChainPosition::new(0, 100),
            100, 100, 0,
            [0x33; 32], 1,
        );
        assert!(cert.verify());
    }
}
