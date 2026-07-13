use amun_chain_store::store::ChainStore;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: gap-fill <data_dir> <sync_server_addr>");
        std::process::exit(1);
    }
    let data_dir = &args[1];
    let server: SocketAddr = args[2].parse().expect("Invalid sync server address");

    let mut store = ChainStore::open(data_dir).expect("Failed to open store");
    let tip = store.latest_height();
    eprintln!("GAP-FILL: store tip = {}", tip);

    // Discover missing ranges
    let mut gaps: Vec<(u64, u64)> = Vec::new();
    let mut gap_start = 0u64;
    for h in 1..=tip {
        if store.load_height(h).is_none() {
            if gap_start == 0 {
                gap_start = h;
            }
        } else if gap_start != 0 {
            gaps.push((gap_start, h - 1));
            gap_start = 0;
        }
    }
    if gap_start != 0 {
        gaps.push((gap_start, tip));
    }
    eprintln!("GAP-FILL: found {} gap(s): {:?}", gaps.len(), gaps);

    if gaps.is_empty() {
        eprintln!("GAP-FILL: store is already complete, nothing to do.");
        return;
    }

    // Connect to the sync server
    let mut stream = TcpStream::connect_timeout(&server, Duration::from_secs(5))
        .expect("Failed to connect to sync server");
    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(10))).ok();

    for (start, end) in gaps {
        eprintln!("GAP-FILL: requesting {}..{}", start, end);

        // Send MSG_BLOCK_RANGE_REQUEST
        let mut req = vec![0x03u8]; // MSG_BLOCK_RANGE_REQUEST
        req.extend_from_slice(&start.to_be_bytes());
        req.extend_from_slice(&end.to_be_bytes());
        stream.write_all(&req).expect("write request");
        stream.flush().expect("flush");

        // Read response type
        let mut resp_type = [0u8; 1];
        stream.read_exact(&mut resp_type).expect("read type");
        if resp_type[0] != 0x04u8 {
            eprintln!("GAP-FILL: unexpected response type {}", resp_type[0]);
            continue;
        }

        // Read record count
        let mut count_buf = [0u8; 4];
        stream.read_exact(&mut count_buf).expect("read count");
        let count = u32::from_be_bytes(count_buf) as usize;
        eprintln!("GAP-FILL: receiving {} records", count);

        // Read each record
        let mut records = Vec::new();
        for _ in 0..count {
            let mut len_buf = [0u8; 4];
            if stream.read_exact(&mut len_buf).is_err() {
                break;
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            if stream.read_exact(&mut data).is_err() {
                break;
            }
            if let Ok(record) = amun_chain_store::record::FinalizedChainRecord::decode(&data) {
                records.push(record);
            }
        }
        eprintln!("GAP-FILL: decoded {} records", records.len());

        // Append to store
        for record in records {
            if store.load_height(record.height).is_none() {
                store.append(record).expect("append");
            }
        }
    }

    // Verify
    let mut missing = 0u64;
    for h in 1..=store.latest_height() {
        if store.load_height(h).is_none() {
            missing += 1;
        }
    }
    eprintln!(
        "GAP-FILL: done. store_len={} tip={} missing={}",
        store.len(),
        store.latest_height(),
        missing
    );
}
