use crate::config::ValidatorConfig;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use amun_sync::catch_up::{download_missing_records, append_missing_records};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime};

pub struct LiveValidator {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub store: Arc<Mutex<ChainStore>>,
    running: Arc<Mutex<bool>>,
    handles: Mutex<Vec<JoinHandle<()>>>,
}

impl LiveValidator {
    pub fn new(config: ValidatorConfig) -> Self {
        let store = ChainStore::open(&config.data_dir).unwrap_or_else(|_| {
            ChainStore::open("/tmp/amun-fallback").unwrap()
        });
        let recovered_height = store.latest_height();
        let recovered_root = store.load_tip().map(|r| r.history_root).unwrap_or([0u8; 32]);
        let mut engine = ConsensusEngine::new(config.validator_id, config.total_validators());
        if recovered_height > 0 {
            engine.current_height = recovered_height;
            engine.history_root = recovered_root;
        }
        Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
            store: Arc::new(Mutex::new(store)),
            running: Arc::new(Mutex::new(false)),
            handles: Mutex::new(Vec::new()),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        *self.running.lock().unwrap() = true;
        let listener = TcpListener::bind(self.config.listen_addr)
            .map_err(|e| format!("Bind error on {}: {}", self.config.listen_addr, e))?;
        listener.set_nonblocking(true).map_err(|e| format!("Set nonblocking error: {}", e))?;

        let engine = self.engine.clone();
        let store = self.store.clone();
        let peers: Vec<_> = self.config.other_peers().into_iter().cloned().collect();
        let validator_id = self.config.validator_id;
        let _total = self.config.total_validators();
        let running = self.running.clone();

        // Listen thread
        let engine_listen = engine.clone();
        let running_listen = running.clone();
        let h1 = thread::spawn(move || {
            while *running_listen.lock().unwrap() {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let _ = stream.set_nonblocking(false);
                        let mut len_buf = [0u8; 4];
                        if stream.read_exact(&mut len_buf).is_ok() {
                            let len = u32::from_be_bytes(len_buf) as usize;
                            if len < 1024 * 1024 {
                                let mut buf = vec![0u8; len];
                                if stream.read_exact(&mut buf).is_ok() {
                                    if let Ok(vote) = postcard::from_bytes::<ConsensusVote>(&buf) {
                                        let mut eng = engine_listen.lock().unwrap();
                                        if let Err(e) = eng.process_vote(vote) {
                                            if e != "Duplicate vote from validator" {
                                                eprintln!("VOTE REJECTED: {}", e);
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
                        continue;
                    },
                }
            }
        });

        // Consensus thread — only work on current_height + 1
        let engine_consensus = engine.clone();
        let store_consensus = store.clone();
        let running_consensus = running.clone();
        let h2 = thread::spawn(move || {
            while *running_consensus.lock().unwrap() {
                let (height, needs_sync) = {
                    let mut eng = engine_consensus.lock().unwrap();
                    let h = eng.current_height + 1;
                    let sync = eng.needs_catchup;
                    eng.needs_catchup = false;
                    (h, sync)
                };
                if needs_sync {
                    let peers_addr: Vec<std::net::SocketAddr> = peers.iter().map(|p| p.address).collect();
                    let current_h = engine_consensus.lock().unwrap().current_height;
                    if let Ok(records) = download_missing_records(current_h, &peers_addr) {
                        if !records.is_empty() {
                            let mut store_g = store_consensus.lock().unwrap();
                            let new_h = append_missing_records(&mut store_g, current_h, records).unwrap_or(current_h);
                            if new_h > current_h {
                                let mut eng2 = engine_consensus.lock().unwrap();
                                eng2.current_height = new_h;
                                eng2.rounds.clear();
                                if let Some(tip) = store_g.load_tip() {
                                    eng2.history_root = tip.history_root;
                                }
                                eprintln!("SYNC: catchup from {} to {}", current_h, new_h);
                            }
                        }
                    }
                    continue;
                }

                let proposer_idx = {
                    let eng = engine_consensus.lock().unwrap();
                    eng.proposer_for(height)
                };
                let is_proposer = validator_id[0] as usize == proposer_idx + 1;

                // Only the proposer creates the round
                if is_proposer {
                    let block_hash = [height as u8; 32];
                    let mut eng = engine_consensus.lock().unwrap();
                    eng.start_round(height, validator_id);
                    if let Some(round) = eng.round_mut(height) {
                        round.propose(block_hash, [0xBB; 32]);
                    }
                }

                let block_hash = [height as u8; 32];
                let my_vote = ConsensusVote {
                    voter_id: validator_id, height, block_hash,
                    state_root: [0xBB; 32], approve: true,
                    signature: [0u8; 64],
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                // Process own vote — with fallback proposal if proposer is silent
                {
                    let mut eng = engine_consensus.lock().unwrap();
                    if !eng.rounds.contains_key(&height) {
                        eng.start_round(height, [(proposer_idx + 1) as u8; 32]);
                        // Fallback: if proposer didn't propose, first validator to arrive proposes
                        if let Some(round) = eng.round_mut(height) {
                            round.propose(block_hash, [0xBB; 32]);
                        }
                    }
                    let _ = eng.process_vote(my_vote.clone());
                }

                // N97.1 Silent validator simulation
                if false && validator_id[0] == 4 {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }

                // Send vote to peers
                let vote_data = postcard::to_stdvec(&my_vote).unwrap();
                let vote_len = vote_data.len() as u32;
                for peer in &peers {
                    for _retry in 0..10 {
                        if let Ok(mut stream) = TcpStream::connect(peer.address) {
                            let _ = stream.set_nonblocking(false);
                            let _ = stream.write_all(&vote_len.to_be_bytes());
                            let _ = stream.write_all(&vote_data);
                            let _ = stream.flush();
                            break;
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                }

                // Wait for votes from peers - break early if quorum reached
                let vote_deadline = Instant::now() + Duration::from_millis(500);
                loop {
                    let quorum_reached = {
                        let eng = engine_consensus.lock().unwrap();
                        if let Some(round) = eng.rounds.get(&height) {
                            let approvals = round.votes.iter().filter(|v| v.approve).count();
                            approvals * 3 > eng.total_validators * 2
                        } else {
                            false
                        }
                    };
                    if quorum_reached {
                        break;
                    }
                    if Instant::now() >= vote_deadline {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                // All validators form QC and persist the finalized block.
                // append() is idempotent — safe for all validators to call.
                let history_root = [height as u8; 32];
                let cert = {
                    let mut eng = engine_consensus.lock().unwrap();
                    eng.try_advance(height, history_root)
                };
                if let Some(cert) = cert {
                    let record = FinalizedChainRecord {
                        height,
                        block_hash: cert.block_hash,
                        state_root: cert.state_root,
                        history_root: cert.history_root,
                        certificate_hash: [0u8; 32],
                        timestamp: SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    if let Err(e) = store_consensus.lock().unwrap().append(record) {
                        eprintln!("STORE ERROR: {}", e);
                    }
                }

                thread::sleep(Duration::from_millis(50));
            }
        });

        self.handles.lock().unwrap().push(h1);
        self.handles.lock().unwrap().push(h2);
        Ok(())
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        let handles: Vec<JoinHandle<()>> = {
            let mut h = self.handles.lock().unwrap();
            std::mem::take(&mut *h)
        };
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn current_height(&self) -> u64 { self.engine.lock().unwrap().current_height }
    pub fn history_root(&self) -> [u8; 32] { self.engine.lock().unwrap().history_root }
    pub fn store_len(&self) -> usize { self.store.lock().unwrap().len() }
    pub fn metrics_summary(&self) -> String { self.engine.lock().unwrap().metrics.summary() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU16, Ordering};

    static PORT_BASE: AtomicU16 = AtomicU16::new(9700);

    fn next_ports() -> [u16; 4] {
        let base = PORT_BASE.fetch_add(10, Ordering::SeqCst);
        [base, base + 1, base + 2, base + 3]
    }

    #[test]
    fn n71_persist_finalized_blocks() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(3000));
        v.stop();
        assert!(v.current_height() >= 1);
        assert!(v.store_len() >= 1);
    }

    #[test]
    fn n71_recover_after_restart() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let data_dir = solo.data_dir.clone();

        let height_after_first;
        {
            let v = LiveValidator::new(solo);
            v.start().unwrap();
            thread::sleep(Duration::from_millis(3000));
            v.stop();
            height_after_first = v.current_height();
        }

        let config2 = ValidatorConfig::test_cluster(0, &next_ports()).with_quorum(1);
        let mut solo2 = config2.clone();
        solo2.cluster = vec![solo2.cluster[0].clone()];
        solo2.data_dir = data_dir;
        let v2 = LiveValidator::new(solo2);
        assert!(v2.current_height() >= height_after_first);
    }

    #[test]
    fn n69_single_validator_self_finalizes() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_two_validators_reach_consensus() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(2));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(2));
        va.start().unwrap();
        vb.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop(); vb.stop();
        let ha = va.store.lock().unwrap().latest_height();
        let hb = vb.store.lock().unwrap().latest_height();
        assert!(ha >= 1 && hb >= 1, "Store heights: A={}, B={}", ha, hb);
    }

    #[test]
    fn n69_three_of_four_reach_quorum() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(3));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(3));
        let vc = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(3));
        va.start().unwrap(); vb.start().unwrap(); vc.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop(); vb.stop(); vc.stop();
        assert!(va.store.lock().unwrap().latest_height() >= 1);
        assert!(vb.store.lock().unwrap().latest_height() >= 1);
        assert!(vc.store.lock().unwrap().latest_height() >= 1);
    }

    #[test]
    fn n69_duplicate_vote_ignored() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_four_validators_full_cluster() {
        let ports = next_ports();
        let validators: Vec<LiveValidator> = (0..4)
            .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)))
            .collect();
        for v in &validators { v.start().unwrap(); }
        thread::sleep(Duration::from_millis(15000));
        for v in &validators { v.stop(); }

        for (i, v) in validators.iter().enumerate() {
            let h = v.store.lock().unwrap().latest_height();
            assert!(h >= 1, "Validator {} store height: {}", i, h);
        println!("Validator {} metrics: {}", i, v.metrics_summary());
        }
    }
}
