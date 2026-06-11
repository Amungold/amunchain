use amun_chain_store::store::ChainStore;
use amun_sync::protocol::handle_incoming_with_store;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: sync-server <data_dir> <bind_addr>");
        std::process::exit(1);
    }
    let data_dir = &args[1];
    let bind_addr = &args[2];

    let store = ChainStore::open(data_dir).expect("Failed to open store");
    let store = Arc::new(Mutex::new(store));

    let listener = TcpListener::bind(bind_addr.as_str()).expect("Failed to bind");
    eprintln!("Sync server listening on {}", bind_addr);

    for stream in listener.incoming().flatten() {
        let store_guard = store.lock().unwrap();
        handle_incoming_with_store(stream, &store_guard, |_data| {
            // vote handler — not used by sync server
        });
    }
}
