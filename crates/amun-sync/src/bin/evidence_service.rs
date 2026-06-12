use amun_consensus_network::messages::EquivocationProof;
use amun_consensus_network::misbehavior::MisbehaviorRegistry;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: evidence_service <listen_port> <data_dir> <peer1> [peer2] [peer3] ...");
        eprintln!("Example: evidence_service 19900 /tmp/amun-test-validator-0 127.0.0.1:19901 127.0.0.1:19902 127.0.0.1:19903");
        std::process::exit(1);
    }

    let listen_port: u16 = args[1].parse().expect("Invalid listen port");
    let data_dir = PathBuf::from(&args[2]);
    let peers: Vec<SocketAddr> = args[3..].iter().map(|a| a.parse().expect("Invalid peer address")).collect();

    let registry_path = data_dir.join("misbehavior.json");
    let registry = Arc::new(Mutex::new(
        MisbehaviorRegistry::open(&registry_path).unwrap_or_else(|_| MisbehaviorRegistry::new())
    ));

    eprintln!("EvidenceService starting on port {} with {} peers", listen_port, peers.len());

    // Thread 1: TCP listener for incoming evidence
    let listener_registry = registry.clone();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", listen_port)).expect("Failed to bind");
    listener.set_nonblocking(true).expect("Failed to set nonblocking");
    
    thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut len_buf = [0u8; 4];
                    if stream.read_exact(&mut len_buf).is_ok() {
                        let len = u32::from_be_bytes(len_buf) as usize;
                        if len < 16 * 1024 * 1024 {
                            let mut buf = vec![0u8; len];
                            if stream.read_exact(&mut buf).is_ok() {
                                if buf.starts_with(b"EVIDENCE") {
                                    if let Ok(proof) = postcard::from_bytes::<EquivocationProof>(&buf[8..]) {
                                        let mut reg = listener_registry.lock().unwrap();
                                        match reg.add_proof(proof) {
                                            Ok(_) => eprintln!("EVIDENCE: accepted proof"),
                                            Err(e) => eprintln!("EVIDENCE: rejected - {}", e),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(_) => break,
            }
        }
    });

    // Thread 2: Periodic gossip sender
    let gossip_registry = registry.clone();
    let gossip_peers = peers.clone();
    thread::spawn(move || {
        let mut broadcasted: HashSet<[u8; 32]> = HashSet::new();
        loop {
            thread::sleep(Duration::from_secs(30));
            
            let reg = gossip_registry.lock().unwrap();
            let new_proofs: Vec<EquivocationProof> = reg.all_proofs()
                .into_iter()
                .filter(|r| !broadcasted.contains(&r.proof_hash))
                .map(|r| r.proof.clone())
                .collect();
            drop(reg);
            
            if new_proofs.is_empty() {
                continue;
            }
            
            eprintln!("GOSSIP: sending {} new proofs to {} peers", new_proofs.len(), gossip_peers.len());
            
            for proof in &new_proofs {
                let proof_data = postcard::to_stdvec(proof).unwrap();
                let header = b"EVIDENCE";
                let total_len = (8 + proof_data.len()) as u32;
                
                for peer in &gossip_peers {
                    if let Ok(mut stream) = TcpStream::connect_timeout(peer, Duration::from_secs(2)) {
                        let _ = stream.write_all(&total_len.to_be_bytes());
                        let _ = stream.write_all(header);
                        let _ = stream.write_all(&proof_data);
                        let _ = stream.flush();
                    }
                }
                
                broadcasted.insert(MisbehaviorRegistry::hash_proof(proof));
            }
        }
    });

    // Main thread: keep alive and periodically save registry
    loop {
        thread::sleep(Duration::from_secs(60));
        if let Ok(reg) = registry.lock() {
            let _ = reg.save();
        }
    }
}
