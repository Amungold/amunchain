use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_sync::protocol::{
    MSG_TIP_REQUEST, MSG_TIP_RESPONSE,
    MSG_BLOCK_RANGE_REQUEST, MSG_BLOCK_RANGE_RESPONSE,
};

/// NetworkingRuntime owns the TCP listener and handles all incoming P2P messages.
///
/// ADR-023: This is a standalone service. It depends on ConsensusEngine and ChainStore
/// via Arc<Mutex<>> for now; a future refactor should replace these with a trait
/// or message channel to fully decouple networking from consensus.
///
/// Uses Arc<Mutex<bool>> for the running flag to maintain compatibility with the
/// rest of the system. A unified lifecycle mechanism (AtomicBool or CancellationToken)
/// will be introduced in Phase 6 across all services.
pub struct NetworkingRuntime {
    listener: TcpListener,
    engine: Arc<Mutex<ConsensusEngine>>,
    store: Arc<Mutex<ChainStore>>,
    running: Arc<Mutex<bool>>,
}

impl NetworkingRuntime {
    pub fn new(
        listen_addr: &str,
        engine: Arc<Mutex<ConsensusEngine>>,
        store: Arc<Mutex<ChainStore>>,
        running: Arc<Mutex<bool>>,
    ) -> Result<Self, String> {
        let listener = TcpListener::bind(listen_addr)
            .map_err(|e| format!("Bind error on {}: {}", listen_addr, e))?;
        listener
            .set_nonblocking(true)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;
        Ok(Self {
            listener,
            engine,
            store,
            running,
        })
    }

    /// Spawn the listener loop on a dedicated thread.
    pub fn spawn_listener(&self) -> JoinHandle<()> {
        let listener = self.listener.try_clone().expect("Failed to clone listener");
        let engine = self.engine.clone();
        let store = self.store.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            while *running.lock().unwrap() {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let _ = stream.set_nonblocking(false);
                        Self::handle_connection(&mut stream, &engine, &store);
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => {
                        eprintln!("LISTENER ERROR: {:?}", e);
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        })
    }

    fn handle_connection(
        stream: &mut TcpStream,
        engine: &Arc<Mutex<ConsensusEngine>>,
        store: &Arc<Mutex<ChainStore>>,
    ) {
        let mut peek_buf = [0u8; 1];
        if stream.peek(&mut peek_buf).is_err() {
            return;
        }

        if peek_buf[0] == MSG_TIP_REQUEST {
            Self::handle_tip_request(stream, store);
        } else if peek_buf[0] == MSG_BLOCK_RANGE_REQUEST {
            Self::handle_block_range_request(stream, store);
        } else {
            Self::handle_vote(stream, engine);
        }
    }

    fn handle_tip_request(stream: &mut TcpStream, store: &Arc<Mutex<ChainStore>>) {
        let _ = stream.read_exact(&mut [0u8; 1]);
        let store_g = store.lock().unwrap();
        let tip = store_g.load_tip();
        let height = tip.as_ref().map(|r| r.height).unwrap_or(0);
        let hash = tip.map(|r| r.block_hash).unwrap_or([0u8; 32]);
        let mut response = vec![0u8; 1 + 8 + 32];
        response[0] = MSG_TIP_RESPONSE;
        response[1..9].copy_from_slice(&height.to_be_bytes());
        response[9..41].copy_from_slice(&hash);
        let _ = stream.write_all(&response);
        let _ = stream.flush();
        eprintln!("SYNC_SERVED: tip_request -> height={}", height);
    }

    fn handle_block_range_request(stream: &mut TcpStream, store: &Arc<Mutex<ChainStore>>) {
        let _ = stream.read_exact(&mut [0u8; 1]);
        let mut range_buf = [0u8; 16];
        if stream.read_exact(&mut range_buf).is_err() {
            return;
        }
        let start = u64::from_be_bytes(range_buf[0..8].try_into().unwrap());
        let end = u64::from_be_bytes(range_buf[8..16].try_into().unwrap());
        eprintln!("SYNC_SERVED: block_range_request {}..{}", start, end);

        let store_g = store.lock().unwrap();
        let mut records: Vec<Vec<u8>> = Vec::new();
        for h in start..=end {
            if let Some(record) = store_g.load_height(h) {
                records.push(record.encode());
            }
        }
        drop(store_g);

        let mut response = vec![MSG_BLOCK_RANGE_RESPONSE];
        response.extend_from_slice(&(records.len() as u32).to_be_bytes());
        for rec in &records {
            let len_bytes = (rec.len() as u32).to_be_bytes();
            response.extend_from_slice(&len_bytes);
            response.extend_from_slice(rec);
        }
        let _ = stream.write_all(&response);
        let _ = stream.flush();
        eprintln!("SYNC_SERVED: sent {} records", records.len());
    }

    fn handle_vote(stream: &mut TcpStream, engine: &Arc<Mutex<ConsensusEngine>>) {
        let mut len_buf = [0u8; 4];
        if stream.read_exact(&mut len_buf).is_err() {
            return;
        }
        let len = u32::from_be_bytes(len_buf) as usize;
        if len >= 1024 * 1024 {
            return;
        }
        let mut buf = vec![0u8; len];
        if stream.read_exact(&mut buf).is_err() {
            return;
        }
        if let Ok(vote) = postcard::from_bytes::<ConsensusVote>(&buf) {
            let mut eng = engine.lock().unwrap();
            if let Err(e) = eng.process_vote(vote) {
                if e != "Duplicate vote from validator" {
                    eprintln!("VOTE REJECTED: {}", e);
                }
            }
        }
    }
}
