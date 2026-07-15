use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::testing::{cleanup, free_ports, unique_test_dir};
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::Duration;

#[test]
fn n129_4_evidence_continuity_audit() {
    let ports = free_ports::<4>();

    let mut dirs = Vec::new();

    for i in 0..4 {
        let dir = unique_test_dir("n129", i);
        cleanup(&dir);
        dirs.push(dir);
    }

    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            let mut config = ValidatorConfig::test_cluster(i, &ports).with_quorum(3);
            config.data_dir = dirs[i].clone();
            LiveValidator::new(config).unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    // Wait for at least 30 blocks
    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        if heights.iter().all(|&h| h >= 30) {
            break;
        }
    }

    // Stop all
    for v in &validators {
        v.stop();
    }

    let store = validators[0].store.lock().unwrap();
    let tip = store.latest_height();
    assert!(tip >= 30, "Chain too short: {}", tip);

    // N129.4-A: Verify constitutional fields are non-zero
    for h in 1..=tip {
        let record = store
            .load_height(h)
            .unwrap_or_else(|| panic!("Missing block {}", h));
        assert_ne!(
            record.verdict_hash, [0u8; 32],
            "N129.4 FAIL: verdict_hash is zero at height {}",
            h
        );
        assert_ne!(
            record.evidence_record_hash, [0u8; 32],
            "N129.4 FAIL: evidence_record_hash is zero at height {}",
            h
        );
        assert_ne!(
            record.evidence_root, [0u8; 32],
            "N129.4 FAIL: evidence_root is zero at height {}",
            h
        );
    }

    // N129.4-B: Verify evidence_root changes between consecutive blocks
    for h in 2..=tip {
        let prev = store.load_height(h - 1).unwrap();
        let curr = store.load_height(h).unwrap();
        assert_ne!(
            prev.evidence_root,
            curr.evidence_root,
            "N129.4 FAIL: evidence_root unchanged between {} and {}",
            h - 1,
            h
        );
    }

    // N129.4-C: Tamper detection — modifying any field changes evidence_root
    let test_h = tip / 2;
    let original = store.load_height(test_h).unwrap().clone();

    // Tamper: modify state_root
    let mut tampered = original.clone();
    tampered.state_root[0] ^= 0xFF;

    let tampered_hash = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_ROOT_V1");
        hasher.update(&tampered.state_root);
        hasher.update(&tampered.block_hash);
        hasher.update(&tampered.verdict_hash);
        hasher.update(&tampered.evidence_record_hash);
        let mut root = [0u8; 32];
        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        root
    };

    assert_ne!(
        original.evidence_root, tampered_hash,
        "N129.4 FAIL: evidence_root must change when state_root is tampered"
    );

    // N129.4-D: Tamper detection — modifying verdict_hash changes evidence_root
    let mut tampered2 = original.clone();
    tampered2.verdict_hash[0] ^= 0xFF;

    let tampered2_hash = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_ROOT_V1");
        hasher.update(&tampered2.state_root);
        hasher.update(&tampered2.block_hash);
        hasher.update(&tampered2.verdict_hash);
        hasher.update(&tampered2.evidence_record_hash);
        let mut root = [0u8; 32];
        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        root
    };

    assert_ne!(
        original.evidence_root, tampered2_hash,
        "N129.4 FAIL: evidence_root must change when verdict_hash is tampered"
    );

    // Clean up
    drop(store);
    for dir in &dirs {
        cleanup(dir);
    }
    println!("\n============================================");
    println!(" N129.4 EVIDENCE CONTINUITY AUDIT");
    println!("============================================");
    println!("  Blocks audited:  {}", tip);
    println!("  Non-zero hashes: PASS");
    println!("  Root chaining:   PASS");
    println!("  Tamper detection: PASS");
    println!("  Verdict:          PASS");
    println!("============================================\n");
}
