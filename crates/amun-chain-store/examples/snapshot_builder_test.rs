use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::snapshot::create_snapshot;
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
        verdict_hash: [0u8; 32],
        evidence_record_hash: [0u8; 32],
        evidence_root: [0u8; 32],
        timestamp: height * 1000,
    }
}

fn main() {
    let dir = tempdir().unwrap();
    let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();

    // Produce 1000 blocks
    for h in 1..=1000 {
        store.append(make_record(h)).unwrap();
    }

    // Create snapshot
    let snapshot_dir = dir.path().join("snapshot");
    let manifest = create_snapshot(&store, &snapshot_dir).unwrap();

    // Verify
    assert_eq!(manifest.snapshot_height, 1000);
    assert!(snapshot_dir.join("manifest.json").exists());
    assert!(snapshot_dir.join("state.bin").exists());
    assert_ne!(manifest.snapshot_hash, [0u8; 32]);

    println!("N99.1 Snapshot Builder Test: PASS");
    println!("  Height: {}", manifest.snapshot_height);
    println!("  Hash:   {}", hex::encode(manifest.snapshot_hash));
}
