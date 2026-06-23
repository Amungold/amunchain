use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::snapshot::{create_snapshot, verify_snapshot};
use amun_chain_store::store::ChainStore;
use tempfile::tempdir;

fn make_record(height: u64) -> FinalizedChainRecord {
    FinalizedChainRecord {
        height,
        block_hash: [height as u8; 32],
        state_root: [0xBB; 32],
        history_root: [height as u8; 32],
        certificate_hash: [0u8; 32],
        slashing_root: [0u8; 32],
        commitment_root: [0u8; 32],
        constitutional_root: [0u8; 32],
        economic_root: [0u8; 32],
        identity_root: [0u8; 32],
        governance_root: [0u8; 32],
        verdict_hash: [0u8; 32],
        evidence_record_hash: [0u8; 32],
        evidence_root: [0u8; 32],
        timestamp: height * 1000,
    }
}

fn main() {
    let dir = tempdir().unwrap();
    let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();

    for h in 1..=1000 {
        store.append(make_record(h)).unwrap();
    }

    let snapshot_dir = dir.path().join("snap");
    create_snapshot(&store, &snapshot_dir).unwrap();

    // Test 1: verify valid snapshot
    let manifest = verify_snapshot(&snapshot_dir).unwrap();
    println!(
        "Test 1 PASS: valid snapshot verified, height={}",
        manifest.snapshot_height
    );

    // Test 2: corrupt manifest and expect failure
    std::fs::write(snapshot_dir.join("manifest.json"), "garbage").unwrap();
    match verify_snapshot(&snapshot_dir) {
        Err(e) => println!("Test 2 PASS: corrupted manifest rejected: {}", e),
        Ok(_) => panic!("Test 2 FAIL: should have rejected corrupted manifest"),
    }

    // Test 3: corrupt state.bin (change height)
    let mut state = std::fs::read(snapshot_dir.join("state.bin")).unwrap();
    state[0] ^= 0xFF;
    std::fs::write(snapshot_dir.join("state.bin"), state).unwrap();
    // Recreate a valid manifest for this test
    create_snapshot(&store, &snapshot_dir).unwrap(); // re-create valid manifest
                                                     // Now corrupt the state.bin height
    let mut state2 = std::fs::read(snapshot_dir.join("state.bin")).unwrap();
    state2[0] ^= 0xFF;
    std::fs::write(snapshot_dir.join("state.bin"), state2).unwrap();
    match verify_snapshot(&snapshot_dir) {
        Err(e) => println!("Test 3 PASS: corrupted state rejected: {}", e),
        Ok(_) => panic!("Test 3 FAIL: should have rejected corrupted state"),
    }

    println!("N99.2 Snapshot Verification: ALL PASS");
}
