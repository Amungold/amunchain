use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_sync::protocol::{
    MSG_BLOCK_RANGE_REQUEST, MSG_BLOCK_RANGE_RESPONSE, MSG_TIP_REQUEST, MSG_TIP_RESPONSE,
};

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct ListenerService;

impl ListenerService {
    pub fn spawn(
        listener: TcpListener,
        engine: Arc<Mutex<ConsensusEngine>>,
        store: Arc<Mutex<ChainStore>>,
        running: Arc<AtomicBool>,
    ) -> JoinHandle<()> {
        let engine_listen = engine.clone();
        let store_listen = store.clone();
        let running_listen = running.clone();

        thread::spawn(move || {
            while running_listen.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let _ = stream.set_nonblocking(false);

                        let mut peek_buf = [0u8; 1];
                        if stream.peek(&mut peek_buf).is_err() {
                            continue;
                        }

                        if peek_buf[0] == MSG_TIP_REQUEST {
                            let _ = stream.read_exact(&mut [0u8; 1]);

                            let store_g = store_listen.lock().expect("mutex poisoned");
                            let tip = store_g.load_tip();

                            let height = tip.as_ref().map(|r| r.height).unwrap_or(0);
                            let hash = tip.map(|r| r.block_hash).unwrap_or([0u8; 32]);

                            let mut response = vec![0u8; 41];
                            response[0] = MSG_TIP_RESPONSE;
                            response[1..9].copy_from_slice(&height.to_be_bytes());
                            response[9..41].copy_from_slice(&hash);

                            let _ = stream.write_all(&response);
                            let _ = stream.flush();

                            eprintln!("SYNC_SERVED: tip_request -> height={}", height);
                        } else if peek_buf[0] == MSG_BLOCK_RANGE_REQUEST {
                            let _ = stream.read_exact(&mut [0u8; 1]);

                            let mut range_buf = [0u8; 16];

                            if stream.read_exact(&mut range_buf).is_ok() {
                                // SAFETY: range_buf is [0u8; 16] populated by read_exact above.
                                // The slice [0..8] is always exactly 8 bytes.
                                let start = u64::from_be_bytes(range_buf[0..8].try_into().expect("8-byte slice"));
                                let end = u64::from_be_bytes(range_buf[8..16].try_into().expect("8-byte slice"));

                                eprintln!("SYNC_SERVED: block_range_request {}..{}", start, end);

                                let store_g = store_listen.lock().expect("mutex poisoned");

                                let mut records = Vec::new();

                                for h in start..=end {
                                    if let Some(record) = store_g.load_height(h) {
                                        records.push(record.encode());
                                    }
                                }

                                let mut response = vec![MSG_BLOCK_RANGE_RESPONSE];

                                response.extend_from_slice(&(records.len() as u32).to_be_bytes());

                                for rec in &records {
                                    response.extend_from_slice(&(rec.len() as u32).to_be_bytes());
                                    response.extend_from_slice(rec);
                                }

                                let _ = stream.write_all(&response);
                                let _ = stream.flush();

                                eprintln!("SYNC_SERVED: sent {} records", records.len());
                            }
                        } else {
                            let mut len_buf = [0u8; 4];

                            if stream.read_exact(&mut len_buf).is_ok() {
                                let len = u32::from_be_bytes(len_buf) as usize;

                                if len < 1024 * 1024 {
                                    let mut buf = vec![0u8; len];

                                    if stream.read_exact(&mut buf).is_ok() {
                                        if let Ok(vote) =
                                            postcard::from_bytes::<ConsensusVote>(&buf)
                                        {
                                            let mut eng = engine_listen.lock().expect("mutex poisoned");

                                            if let Err(e) = eng.process_vote(&vote) {
                                                if e != "Duplicate vote from validator" {
                                                    eprintln!("VOTE REJECTED: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
}
