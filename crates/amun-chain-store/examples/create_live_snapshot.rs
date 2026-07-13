use amun_chain_store::snapshot::create_snapshot;
use amun_chain_store::store::ChainStore;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let store_dir = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("/tmp/amun-test-validator-0");
    let snap_dir = args
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("/tmp/amun-test-validator-0/snapshot");

    let store = ChainStore::open(store_dir).expect("open store");
    let manifest = create_snapshot(&store, Path::new(snap_dir)).expect("create snapshot");
    println!("Snapshot created at height {}", manifest.snapshot_height);
    println!("Snapshot hash: {}", hex::encode(manifest.snapshot_hash));
}
