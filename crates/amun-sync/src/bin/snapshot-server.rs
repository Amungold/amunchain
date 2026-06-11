use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: snapshot-server <snapshot_dir> [bind_addr]");
        std::process::exit(1);
    }
    let snapshot_dir = Path::new(&args[1]);
    let bind_addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:19998");

    let listener = TcpListener::bind(bind_addr).expect("Failed to bind");
    eprintln!("Snapshot server listening on {}", bind_addr);
    eprintln!("Serving snapshot from {}", snapshot_dir.display());

    for stream in listener.incoming().flatten() {
        handle_client(stream, snapshot_dir);
    }
}

fn handle_client(mut stream: TcpStream, snapshot_dir: &Path) {
    let mut len_buf = [0u8; 4];
    if stream.read_exact(&mut len_buf).is_err() { return; }
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > 1024 { return; }
    let mut buf = vec![0u8; len];
    if stream.read_exact(&mut buf).is_err() { return; }

    if buf == b"MANIFEST" {
        let manifest_path = snapshot_dir.join("manifest.json");
        if manifest_path.exists() {
            let data = fs::read(&manifest_path).unwrap_or_default();
            let _ = stream.write_all(&(data.len() as u32).to_be_bytes());
            let _ = stream.write_all(&data);
        } else {
            let _ = stream.write_all(&0u32.to_be_bytes());
        }
    } else if buf == b"STATE" {
        let state_path = snapshot_dir.join("state.bin");
        if state_path.exists() {
            let data = fs::read(&state_path).unwrap_or_default();
            let _ = stream.write_all(&(data.len() as u32).to_be_bytes());
            let _ = stream.write_all(&data);
        } else {
            let _ = stream.write_all(&0u32.to_be_bytes());
        }
    }
    let _ = stream.flush();
}
