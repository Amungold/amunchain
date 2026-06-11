use amun_chain_store::snapshot::{restore_snapshot, verify_snapshot};
use amun_chain_store::store::ChainStore;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: bootstrap <store_dir> <peer_addr> <snapshot_dir>");
        std::process::exit(1);
    }
    let store_dir = Path::new(&args[1]);
    let peer: SocketAddr = args[2].parse().expect("Invalid peer address");
    let snapshot_dir = Path::new(&args[3]);

    // 1. Download snapshot manifest from peer
    eprintln!("BOOTSTRAP: downloading snapshot from {}", peer);
    let mut stream = TcpStream::connect_timeout(&peer, Duration::from_secs(5))
        .expect("Failed to connect to peer");
    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(10))).ok();

    let request = b"SNAPSHOT";
    let req_len = (request.len() as u32).to_be_bytes();
    stream.write_all(&req_len).expect("write len");
    stream.write_all(request).expect("write request");
    stream.flush().expect("flush");

    // Read manifest length
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).expect("read len");
    let manifest_len = u32::from_be_bytes(len_buf) as usize;
    if manifest_len == 0 {
        eprintln!("BOOTSTRAP: peer has no snapshot available");
        std::process::exit(1);
    }

    // Read manifest
    let mut manifest_data = vec![0u8; manifest_len];
    stream.read_exact(&mut manifest_data).expect("read manifest");

    // Write manifest to snapshot_dir
    std::fs::create_dir_all(snapshot_dir).expect("create snapshot dir");
    std::fs::write(snapshot_dir.join("manifest.json"), &manifest_data).expect("write manifest");

    eprintln!("BOOTSTRAP: snapshot manifest downloaded ({} bytes)", manifest_len);

    // 2. Verify snapshot
    let manifest = verify_snapshot(snapshot_dir).expect("snapshot verification failed");
    eprintln!("BOOTSTRAP: snapshot verified at height {}", manifest.snapshot_height);

    // 3. Restore store
    restore_snapshot(snapshot_dir, store_dir).expect("restore failed");
    eprintln!("BOOTSTRAP: store restored from snapshot");

    // 4. Verify restored store
    let store = ChainStore::open(store_dir.to_str().unwrap()).expect("open restored store");
    eprintln!("BOOTSTRAP: store tip height = {}", store.latest_height());
    eprintln!("BOOTSTRAP: complete — node is ready to join consensus");
}
