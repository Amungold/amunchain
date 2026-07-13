use amun_chain_store::store::ChainStore;
use amun_sync::catch_up::{append_missing_records, download_missing_records};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

fn handshake(peer: &SocketAddr) -> Option<(u64, u8)> {
    let mut stream = TcpStream::connect_timeout(peer, Duration::from_secs(5)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .ok()?;
    let request = b"HELLO".to_vec();
    let req_len = (request.len() as u32).to_be_bytes();
    stream.write_all(&req_len).ok()?;
    stream.write_all(&request).ok()?;
    stream.flush().ok()?;
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).ok()?;
    let resp_len = u32::from_be_bytes(len_buf) as usize;
    let mut resp_data = vec![0u8; resp_len];
    stream.read_exact(&mut resp_data).ok()?;
    let network_height = u64::from_le_bytes(resp_data[7..15].try_into().ok()?);
    let validator_count = resp_data[15];
    Some((network_height, validator_count))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: rejoin <data_dir> <peer1> [peer2] [peer3] ...");
        std::process::exit(1);
    }
    let data_dir = Path::new(&args[1]);
    let peers: Vec<SocketAddr> = args[2..]
        .iter()
        .map(|a| a.parse().expect("Invalid peer address"))
        .collect();

    // Phase 1: Handshake with any available peer
    let mut network_height = 0u64;
    let mut sync_peer = peers[0];

    for peer in &peers {
        eprintln!("REJOIN: Trying handshake on {}...", peer);
        if let Some((nh, vc)) = handshake(peer) {
            network_height = nh;
            sync_peer = SocketAddr::new(peer.ip(), peer.port() + 10000);
            eprintln!(
                "REJOIN: Network height = {}, validators = {}",
                network_height, vc
            );
            break;
        }
    }

    if network_height == 0 {
        eprintln!("REJOIN: Could not connect to any peer.");
        std::process::exit(1);
    }

    // Phase 2: Fast-forward to network height
    let mut store = ChainStore::open(data_dir.to_str().unwrap()).expect("Failed to open store");
    let local_height = store.latest_height();

    if network_height > local_height {
        eprintln!(
            "REJOIN: Fast-forwarding from {} to {}...",
            local_height, network_height
        );
        eprintln!(
            "REJOIN: Using sync endpoint {} for block download",
            sync_peer
        );

        match download_missing_records(local_height, &[sync_peer]) {
            Ok(records) => {
                eprintln!("REJOIN: Downloaded {} records", records.len());
                if !records.is_empty() {
                    let new_h = append_missing_records(&mut store, local_height, records)
                        .unwrap_or(local_height);
                    eprintln!("REJOIN: Store height now {}", new_h);
                }
            }
            Err(e) => eprintln!("REJOIN: Download failed: {}", e),
        }
    } else {
        eprintln!(
            "REJOIN: Already at network height ({} >= {})",
            local_height, network_height
        );
    }

    // Phase 3: Fill historical gaps
    let final_height = store.latest_height();
    let mut missing = 0u64;
    for h in 1..=final_height {
        if store.load_height(h).is_none() {
            missing += 1;
        }
    }

    if missing > 0 {
        eprintln!("REJOIN: Found {} missing records. Filling gaps...", missing);
        let mut current_start = 1u64;
        while current_start <= final_height {
            match download_missing_records(current_start - 1, &[sync_peer]) {
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
        missing = 0u64;
        let final_height = store.latest_height();
        for h in 1..=final_height {
            if store.load_height(h).is_none() {
                missing += 1;
            }
        }
    }

    eprintln!(
        "REJOIN: Complete. final_height={} missing={}",
        store.latest_height(),
        missing
    );
    if missing == 0 {
        eprintln!("REJOIN: Store is fully synchronized.");
    } else {
        eprintln!("REJOIN: Store still has {} gaps.", missing);
    }
    eprintln!("REJOIN: Node can now be restarted to join active consensus.");
}
