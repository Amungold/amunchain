use amun_chain_store::store::ChainStore;
use amun_sync::catch_up::{append_missing_records, download_missing_records};
use std::net::SocketAddr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: backfill <data_dir> <sync_server_addr>");
        std::process::exit(1);
    }
    let data_dir = &args[1];
    let sync_server: SocketAddr = args[2].parse().expect("Invalid sync server address");

    let mut store = ChainStore::open(data_dir).expect("Failed to open store");
    let local_height = store.latest_height();
    eprintln!(
        "BACKFILL: local_height={}, sync_server={}",
        local_height, sync_server
    );

    let peers = vec![sync_server];
    match download_missing_records(local_height, &peers) {
        Ok(records) => {
            eprintln!("BACKFILL: downloaded {} records", records.len());
            if !records.is_empty() {
                let new_h = append_missing_records(&mut store, local_height, records)
                    .unwrap_or(local_height);
                eprintln!("BACKFILL: store height now {}", new_h);
            }
        }
        Err(e) => {
            eprintln!("BACKFILL: download failed: {}", e);
        }
    }
    eprintln!("BACKFILL: complete");
}
