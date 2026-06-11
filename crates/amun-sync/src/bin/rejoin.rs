use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;
use amun_chain_store::store::ChainStore;
use amun_sync::catch_up::{download_missing_records, append_missing_records};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: rejoin <data_dir> <peer_addr>");
        eprintln!("  data_dir: path to the validator's data directory");
        eprintln!("  peer_addr: address of any active validator's MAIN port (e.g., 127.0.0.1:9900)");
        std::process::exit(1);
    }
    let data_dir = Path::new(&args[1]);
    let main_peer: SocketAddr = args[2].parse().expect("Invalid peer address");
    
    // Compute sync address: main port + 10000
    let sync_peer = SocketAddr::new(main_peer.ip(), main_peer.port() + 10000);

    // Phase 1: Handshake on main port to discover network height
    eprintln!("REJOIN: Handshake on {}...", main_peer);
    {
        let mut stream = TcpStream::connect_timeout(&main_peer, Duration::from_secs(5))
            .expect("Failed to connect to peer for handshake");
        stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

        // Send HELLO
        let request = b"HELLO".to_vec();
        let req_len = (request.len() as u32).to_be_bytes();
        stream.write_all(&req_len).expect("write len");
        stream.write_all(&request).expect("write request");
        stream.flush().expect("flush");

        // Read WELCOME response
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).expect("read len");
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        let mut resp_data = vec![0u8; resp_len];
        stream.read_exact(&mut resp_data).expect("read data");

        let network_height = u64::from_le_bytes(resp_data[7..15].try_into().unwrap());
        let validator_count = resp_data[15];
        eprintln!("REJOIN: Network height = {}, validators = {}", network_height, validator_count);
        
        // Store for later use
        let nh = network_height;
        drop(stream);
        
        // Phase 2: Fast-forward to network height via sync endpoint
        let mut store = ChainStore::open(data_dir.to_str().unwrap()).expect("Failed to open store");
        let local_height = store.latest_height();
        
        if nh > local_height {
            eprintln!("REJOIN: Fast-forwarding from {} to {}...", local_height, nh);
            eprintln!("REJOIN: Using sync endpoint {} for block download", sync_peer);
            
            let peers = vec![sync_peer];
            match download_missing_records(local_height, &peers) {
                Ok(records) => {
                    eprintln!("REJOIN: Downloaded {} records", records.len());
                    if !records.is_empty() {
                        let new_h = append_missing_records(&mut store, local_height, records)
                            .unwrap_or(local_height);
                        eprintln!("REJOIN: Store height now {}", new_h);
                    }
                }
                Err(e) => {
                    eprintln!("REJOIN: Download failed: {}", e);
                    eprintln!("REJOIN: Continuing with fast-forward only (store will have gaps)");
                }
            }
        } else {
            eprintln!("REJOIN: Already at network height ({} >= {})", local_height, nh);
        }

        // Phase 3: Fill historical gaps (1..final_height)
        let final_height = store.latest_height();
        eprintln!("REJOIN: Checking for historical gaps...");
        let mut missing = 0u64;
        let mut gap_start = 0u64;
        for h in 1..=final_height {
            if store.load_height(h).is_none() {
                missing += 1;
                if gap_start == 0 { gap_start = h; }
            }
        }
        
        if missing > 0 {
            eprintln!("REJOIN: Found {} missing records. Filling gaps...", missing);
            // Use the sync endpoint to download missing ranges
            let peers = vec![sync_peer];
            let mut current_start = 1u64;
            while current_start <= final_height {
                match download_missing_records(current_start - 1, &peers) {
                    Ok(records) => {
                        if records.is_empty() { 
                            current_start += 1;
                            continue; 
                        }
                        let new_h = append_missing_records(&mut store, current_start - 1, records)
                            .unwrap_or(current_start - 1);
                        current_start = new_h + 1;
                        eprintln!("REJOIN: Gap-fill progress: height={}", new_h);
                    }
                    Err(e) => {
                        eprintln!("REJOIN: Gap-fill error at {}: {}", current_start, e);
                        break;
                    }
                }
            }
            
            // Recount
            missing = 0u64;
            let final_height = store.latest_height();
            for h in 1..=final_height {
                if store.load_height(h).is_none() { missing += 1; }
            }
        }
        
        eprintln!("REJOIN: Complete. final_height={} missing={}", store.latest_height(), missing);
        
        if missing == 0 {
            eprintln!("REJOIN: Store is fully synchronized.");
        } else {
            eprintln!("REJOIN: Store still has {} gaps.", missing);
        }
        
        eprintln!("REJOIN: Node can now be restarted to join active consensus.");
    }
}
