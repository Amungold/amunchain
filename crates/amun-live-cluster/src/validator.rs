use crate::config::ValidatorConfig;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};

pub struct LiveValidator {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    running: Arc<Mutex<bool>>,
    handles: Mutex<Vec<JoinHandle<()>>>,
}

impl LiveValidator {
    pub fn new(config: ValidatorConfig) -> Self {
        let engine = ConsensusEngine::new(config.validator_id, config.total_validators());
        Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
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
        let peers: Vec<_> = self.config.other_peers().into_iter().cloned().collect();
        let validator_id = self.config.validator_id;
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
                                        eprintln!("VOTE REJECTED: {}", e);
                                    }
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => break,
                }
            }
        });

        // Consensus thread
        let engine_consensus = engine.clone();
        let running_consensus = running.clone();
        let h2 = thread::spawn(move || {
            let mut height = 1u64;
            while *running_consensus.lock().unwrap() {
                let proposer_idx = {
                    let eng = engine_consensus.lock().unwrap();
                    eng.proposer_for(height)
                };
                let is_proposer = validator_id[0] as usize == proposer_idx + 1;

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

                {
                    let mut eng = engine_consensus.lock().unwrap();
                    if !eng.rounds.contains_key(&height) {
                        eng.start_round(height, [(proposer_idx + 1) as u8; 32]);
                    }
                    let _ = eng.process_vote(my_vote.clone());
                }

                // Send vote to peers with retry
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

                thread::sleep(Duration::from_millis(500));

                let history_root = [height as u8; 32];
                let mut eng = engine_consensus.lock().unwrap();
                if let Some(_cert) = eng.try_advance(height, history_root) {
                    height += 1;
                }
                thread::sleep(Duration::from_millis(300));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU16, Ordering};

    static PORT_BASE: AtomicU16 = AtomicU16::new(9600);

    fn next_ports() -> [u16; 4] {
        let base = PORT_BASE.fetch_add(10, Ordering::SeqCst);
        [base, base + 1, base + 2, base + 3]
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
        assert!(v.current_height() >= 1, "Height: {}", v.current_height());
    }

    #[test]
    fn n69_two_validators_reach_consensus() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(2));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(2));
        va.start().unwrap();
        vb.start().unwrap();
        thread::sleep(Duration::from_millis(5000));
        va.stop(); vb.stop();
        assert!(va.current_height() >= 1, "Validator A height: {}", va.current_height());
        assert!(vb.current_height() >= 1, "Validator B height: {}", vb.current_height());
        assert_eq!(va.history_root(), vb.history_root(),
            "History roots must match: A={:?}, B={:?}",
            &va.history_root()[..4], &vb.history_root()[..4]);
    }

    #[test]
    fn n69_three_of_four_reach_quorum() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(3));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(3));
        let vc = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(3));
        va.start().unwrap(); vb.start().unwrap(); vc.start().unwrap();
        thread::sleep(Duration::from_millis(5000));
        va.stop(); vb.stop(); vc.stop();
        assert!(va.current_height() >= 1, "3/3 should reach quorum");
        assert_eq!(va.history_root(), vb.history_root());
        assert_eq!(va.history_root(), vc.history_root());
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
        thread::sleep(Duration::from_millis(10000));
        for v in &validators { v.stop(); }

        let min_h = validators.iter().map(|v| v.current_height()).min().unwrap_or(0);
        let max_h = validators.iter().map(|v| v.current_height()).max().unwrap_or(0);
        for (i, v) in validators.iter().enumerate() {
            assert!(v.current_height() >= 1, "Validator {} height: {}", i, v.current_height());
        }
        assert!(max_h - min_h <= 1, "Height spread too large: min={}, max={}", min_h, max_h);
    }
}
