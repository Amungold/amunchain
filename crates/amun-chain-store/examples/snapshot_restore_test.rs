use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::snapshot::{create_snapshot, restore_snapshot};
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
        timestamp: height * 1000,
    }
}

fn main() {
    let dir = tempdir().unwrap();
    let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();

    // Create 1000 blocks
    for h in 1..=1000 {
        store.append(make_record(h)).unwrap();
    }

    // Build snapshot
    let snap_dir = dir.path().join("snap");
    let manifest = create_snapshot(&store, &snap_dir).unwrap();
    println!("Snapshot created at height {}", manifest.snapshot_height);

    // Restore into a new store
    let restore_dir = dir.path().join("restored");
    let restored_manifest = restore_snapshot(&snap_dir, &restore_dir).unwrap();
    println!(
        "Snapshot restored at height {}",
        restored_manifest.snapshot_height
    );

    // Verify restored store
    let restored_store = ChainStore::open(restore_dir.to_str().unwrap()).unwrap();
    let tip = restored_store.load_tip().unwrap();
    assert_eq!(tip.height, 1000);
    assert_eq!(tip.state_root, [0xBB; 32]);
    assert_eq!(tip.history_root, [232u8; 32]);
    assert_eq!(restored_store.latest_height(), 1000);

    // Ensure the restored store has only one record (the tip) because we didn't store history
    // That's expected.

    println!("N99.3 Snapshot Restore Test: PASS");
}
