use amun_chain_store::snapshot::{create_snapshot, restore_snapshot, verify_snapshot};
use amun_chain_store::store::ChainStore;
use amun_sync::catch_up::{download_missing_records, append_missing_records};
use std::net::SocketAddr;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: bootstrap <source_data_dir> <target_data_dir> <sync_peer_addr>");
        eprintln!("  source_data_dir: path to a running validator's store (for creating snapshot)");
        eprintln!("  target_data_dir: path where the new node will be restored");
        eprintln!("  sync_peer_addr:  address of the embedded sync endpoint (port+10000)");
        std::process::exit(1);
    }
    let source_dir = Path::new(&args[1]);
    let target_dir = Path::new(&args[2]);
    let sync_peer: SocketAddr = args[3].parse().expect("Invalid sync peer address");

    // Phase 1: Snapshot
    let snap_dir = source_dir.join("snapshot_bootstrap");
    let source_store = ChainStore::open(source_dir.to_str().unwrap()).expect("Failed to open source store");
    eprintln!("Creating snapshot from {} at height {}...", source_dir.display(), source_store.latest_height());
    let manifest = create_snapshot(&source_store, &snap_dir).expect("Failed to create snapshot");
    eprintln!("Snapshot created: height={}, hash={}", manifest.snapshot_height, hex::encode(manifest.snapshot_hash));

    // Phase 2: Verify
    eprintln!("Verifying snapshot...");
    verify_snapshot(&snap_dir).expect("Snapshot verification failed");
    eprintln!("Snapshot verified.");

    // Phase 3: Restore
    eprintln!("Restoring snapshot to {}...", target_dir.display());
    let restored_manifest = restore_snapshot(&snap_dir, target_dir).expect("Failed to restore snapshot");
    eprintln!("Restored to height {}", restored_manifest.snapshot_height);

    // Phase 4: Historical gap-fill (1..snapshot_height)
    eprintln!("Filling historical gaps...");
    let mut target_store = ChainStore::open(target_dir.to_str().unwrap()).expect("Failed to open target store");
    let peers = vec![sync_peer];
    
    // Fill from height 1 to snapshot_height
    let mut current_gap_start = 1u64;
    while current_gap_start <= manifest.snapshot_height {
        match download_missing_records(current_gap_start - 1, &peers) {
            Ok(records) => {
                if records.is_empty() { break; }
                eprintln!("Gap-fill: downloaded {} records from height {}", records.len(), records.first().unwrap().height);
                let new_h = append_missing_records(&mut target_store, current_gap_start - 1, records)
                    .unwrap_or(current_gap_start - 1);
                current_gap_start = new_h + 1;
                eprintln!("Gap-fill: store height now {}", new_h);
            }
            Err(e) => {
                eprintln!("Gap-fill: download failed at height {}: {}", current_gap_start, e);
                break;
            }
        }
    }

    // Phase 5: Delta sync – catch up any blocks produced since snapshot
    eprintln!("Syncing recent blocks...");
    let local_height = target_store.latest_height();
    match download_missing_records(local_height, &peers) {
        Ok(records) => {
            eprintln!("Delta sync: downloaded {} records", records.len());
            if !records.is_empty() {
                let new_h = append_missing_records(&mut target_store, local_height, records)
                    .unwrap_or(local_height);
                eprintln!("Delta sync: store height now {}", new_h);
            }
        }
        Err(e) => {
            eprintln!("Delta sync failed: {}", e);
        }
    }

    // Phase 6: Verify final state
    let final_height = target_store.latest_height();
    
    // Count missing records
    let mut missing = 0u64;
    for h in 1..=final_height {
        if target_store.load_height(h).is_none() {
            missing += 1;
        }
    }
    eprintln!("Bootstrap complete: final_height={} store_records={} missing={}",
        final_height, target_store.len(), missing);
}
