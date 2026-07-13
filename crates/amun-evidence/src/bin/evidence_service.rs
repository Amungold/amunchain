use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: evidence_service <listen_port> <data_dir> <peer1> [peer2] [peer3] ...");
        return;
    }

    let listen_port: u16 = args[1].parse().expect("Invalid port");
    let data_dir = PathBuf::from(&args[2]);
    let peers: Vec<String> = args[3..].to_vec();

    eprintln!(
        "EvidenceService starting on port {} with {} peers",
        listen_port,
        peers.len()
    );

    let evidence_dir = data_dir.join("evidence");
    fs::create_dir_all(&evidence_dir).expect("Failed to create evidence dir");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", listen_port)).expect("Failed to bind");

    // Registry watcher thread
    let evidence_dir_clone = evidence_dir.clone();
    let peers_clone = peers.clone();
    thread::spawn(move || {
        let mut seen: Vec<String> = Vec::new();
        loop {
            eprintln!("WATCHER: polling registry at {:?}", evidence_dir_clone);
            if let Ok(entries) = fs::read_dir(&evidence_dir_clone) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    eprintln!("WATCHER: found file {:?}", path);
                    if path.extension().is_some_and(|e| e == "json") {
                        let filename = path.to_string_lossy().to_string();
                        if !seen.contains(&filename) {
                            seen.push(filename.clone());
                            eprintln!("WATCHER: new proof detected: {}", filename);
                            if let Ok(data) = fs::read_to_string(&path) {
                                // Gossip to peers
                                for peer in &peers_clone {
                                    let msg = format!("PROOF:{}", data);
                                    let len = (msg.len() as u32).to_be_bytes();
                                    if let Ok(mut stream) = TcpStream::connect_timeout(
                                        &peer.parse().unwrap(),
                                        Duration::from_secs(2),
                                    ) {
                                        let _ = stream.write_all(&len);
                                        let _ = stream.write_all(msg.as_bytes());
                                        let _ = stream.flush();
                                        eprintln!("WATCHER: sent proof to {}", peer);
                                    } else {
                                        eprintln!("WATCHER: failed to connect to {}", peer);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("WATCHER: failed to read_dir");
            }
            thread::sleep(Duration::from_secs(3));
        }
    });

    // Accept incoming gossip
    for stream in listener.incoming().flatten() {
        let mut stream = stream;
        let evidence_dir = evidence_dir.clone();
        thread::spawn(move || {
            let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
            let mut len_buf = [0u8; 4];
            if stream.read_exact(&mut len_buf).is_ok() {
                let len = u32::from_be_bytes(len_buf) as usize;
                if len < 10 * 1024 * 1024 {
                    let mut buf = vec![0u8; len];
                    if stream.read_exact(&mut buf).is_ok() {
                        if let Ok(msg) = String::from_utf8(buf) {
                            if let Some(proof_data) = msg.strip_prefix("PROOF:") {
                                let hash = blake3::hash(proof_data.as_bytes());
                                let filename = format!("gossip_{}.json", hash.to_hex());
                                let dest = evidence_dir.join(&filename);
                                fs::write(&dest, proof_data).ok();
                                eprintln!("RECEIVED: proof from gossip -> {}", dest.display());
                            }
                        }
                    }
                }
            }
        });
    }
}
