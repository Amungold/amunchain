use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use amun_sync::catch_up::{append_missing_records, download_missing_records};
use amun_sync::protocol::handle_incoming_with_store;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create store A with 1000 records (heights 1..1000)
    let dir_a = tempfile::tempdir().unwrap();
    let mut store_a = ChainStore::open(dir_a.path().to_str().unwrap()).unwrap();
    for h in 1..=1000 {
        store_a.append(make_record(h)).unwrap();
    }
    let store_a = Arc::new(Mutex::new(store_a));

    // 2. Start sync server on a random port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server_addr = listener.local_addr().unwrap();
    let server_store = store_a.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let store_guard = server_store.lock().unwrap();
                handle_incoming_with_store(stream, &store_guard, |_data| {
                    // vote handler – not used in this test
                });
            }
        }
    });
    thread::sleep(Duration::from_millis(100));

    // 3. Client: download full chain from genesis (current_height=0)
    let peers = vec![server_addr];
    let records = download_missing_records(0, &peers).unwrap();

    println!("Downloaded {} records", records.len());
    assert_eq!(
        records.len(),
        1000,
        "Expected 1000 records, got {}",
        records.len()
    );
    assert_eq!(records.first().unwrap().height, 1);
    assert_eq!(records.last().unwrap().height, 1000);

    // 4. Append to empty store B and verify
    let dir_b = tempfile::tempdir().unwrap();
    let mut store_b = ChainStore::open(dir_b.path().to_str().unwrap()).unwrap();
    let new_h = append_missing_records(&mut store_b, 0, records).unwrap();
    println!("Store B new height: {}", new_h);
    assert_eq!(new_h, 1000);
    assert_eq!(store_b.latest_height(), 1000);

    // 5. Verify all heights 1..1000 are present
    let mut missing = 0u64;
    for h in 1..=1000 {
        if store_b.load_height(h).is_none() {
            eprintln!("MISSING height {}", h);
            missing += 1;
        }
    }
    assert_eq!(
        missing, 0,
        "Found {} missing heights after full sync",
        missing
    );
    assert!(store_b.load_height(1001).is_none());

    println!("N98.0 Block Sync Protocol Validation: PASS");
}

fn make_record(height: u64) -> FinalizedChainRecord {
    FinalizedChainRecord {
        height,
        block_hash: [height as u8; 32],
        state_root: [0xBB; 32],
        history_root: [height as u8; 32],
        certificate_hash: [0u8; 32],
        timestamp: height * 1000,
    }
}
