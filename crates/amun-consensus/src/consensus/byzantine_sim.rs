//! Byzantine Simulation - FULLY DETERMINISTIC

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use super::qc_store::{QCStore, QC};

struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }
    
    fn gen_range(&mut self, low: usize, high: usize) -> usize {
        assert!(high > low, "gen_range: high must be greater than low");
        (self.next_u64() as usize % (high - low)) + low
    }
}

#[derive(Debug, Clone)]
pub enum ByzantineBehavior {
    Honest,
    Equivocate,
    DoubleVote,
    ProposeFork,
    LineageFork,
    AuthorityDrift,
}

fn block_hash_from_round(round: u64) -> [u8; 32] {
    let mut hash = [0u8; 32];
    let bytes = round.to_le_bytes();
    hash[..8].copy_from_slice(&bytes);
    hash
}

pub struct ByzantineSimulator {
    pub validators: BTreeMap<u64, ByzantineBehavior>,
    pub store: QCStore,
    pub rounds: u64,
    pub equivocations_detected: u64,
    pub forks_detected: u64,
    pub transcript_hashes: Vec<[u8; 32]>,
    rng: DeterministicRng,
}

impl ByzantineSimulator {
    pub fn new(num: usize, num_byz: usize, seed: u64) -> Self {
        let mut rng = DeterministicRng::new(seed);
        let mut validators = BTreeMap::new();
        
        for i in 0..num {
            let b = if i < num_byz {
                match rng.gen_range(0, 5) {
                    0 => ByzantineBehavior::Equivocate,
                    1 => ByzantineBehavior::DoubleVote,
                    2 => ByzantineBehavior::ProposeFork,
                    3 => ByzantineBehavior::LineageFork,
                    _ => ByzantineBehavior::AuthorityDrift,
                }
            } else {
                ByzantineBehavior::Honest
            };
            validators.insert(i as u64, b);
        }
        
        Self {
            validators,
            store: QCStore::new(),
            rounds: 0,
            equivocations_detected: 0,
            forks_detected: 0,
            transcript_hashes: Vec::new(),
            rng,
        }
    }
    
    pub fn run(&mut self, rounds: u64) {
        for r in 0..rounds {
            self.run_round(r);
        }
    }
    
    fn run_round(&mut self, round: u64) {
        let mut votes: BTreeMap<u64, Vec<[u8; 32]>> = BTreeMap::new();
        let mut round_hash_bytes = Vec::new();
        let round_bytes = round.to_le_bytes();
        
        for (id, behavior) in &self.validators {
            match behavior {
                ByzantineBehavior::Honest => {
                    let h = blake3::hash(&round_bytes).into();
                    votes.entry(*id).or_default().push(h);
                    round_hash_bytes.extend_from_slice(&h[..]);
                }
                ByzantineBehavior::Equivocate => {
                    let mut bytes1 = round_bytes.to_vec();
                    bytes1.push(1);
                    let mut bytes2 = round_bytes.to_vec();
                    bytes2.push(2);
                    let h1 = blake3::hash(&bytes1).into();
                    let h2 = blake3::hash(&bytes2).into();
                    votes.entry(*id).or_default().push(h1);
                    votes.entry(*id).or_default().push(h2);
                    round_hash_bytes.extend_from_slice(&h1[..]);
                    round_hash_bytes.extend_from_slice(&h2[..]);
                }
                ByzantineBehavior::DoubleVote => {
                    let h = blake3::hash(&round_bytes).into();
                    votes.entry(*id).or_default().push(h);
                    votes.entry(*id).or_default().push(h);
                    round_hash_bytes.extend_from_slice(&h[..]);
                }
                ByzantineBehavior::ProposeFork => {
                    let block_hash = block_hash_from_round(round);
                    let q = QC::new(round, block_hash, 3, round, 0, [0u8; 32]).finalize();
                    self.store.insert(q);
                    self.forks_detected += 1;
                }
                ByzantineBehavior::LineageFork => {
                    let block_hash1 = block_hash_from_round(round);
                    let block_hash2 = block_hash_from_round(round + 1);
                    let block_hash3 = block_hash_from_round(round + 2);
                    
                    let q1 = QC::new(round, block_hash1, 3, round, 0, [0u8; 32]).finalize();
                    let q2 = QC::new(round + 1, block_hash2, 3, round + 1, 0, [0u8; 32])
                        .with_parent(q1.hash).finalize();
                    let q3 = QC::new(round + 1, block_hash3, 3, round + 1, 0, [0u8; 32])
                        .with_parent(q1.hash).finalize();
                    self.store.insert(q1);
                    self.store.insert(q2);
                    self.store.insert(q3);
                    self.forks_detected += 1;
                }
                ByzantineBehavior::AuthorityDrift => {
                    let block_hash = block_hash_from_round(round);
                    let mut q = QC::new(round, block_hash, 3, round, 0, [0u8; 32]);
                    q.authority_root = block_hash;
                    let q = q.finalize();
                    self.store.insert(q);
                }
            }
        }
        
        for (_, vhashes) in votes {
            if vhashes.len() > 1 {
                self.equivocations_detected += 1;
            }
        }
        
        let mut domain_bytes = Vec::new();
        domain_bytes.extend_from_slice(b"AMUN_BYZANTINE_ROUND_V1");
        domain_bytes.extend_from_slice(&round_bytes);
        domain_bytes.extend_from_slice(&round_hash_bytes);
        let transcript_hash = blake3::hash(&domain_bytes).into();
        self.transcript_hashes.push(transcript_hash);
        self.rounds += 1;
    }
    
    pub fn transcript_hash(&self) -> [u8; 32] {
        let mut all_bytes = Vec::new();
        all_bytes.extend_from_slice(b"AMUN_BYZANTINE_TRANSCRIPT_V1");
        for h in &self.transcript_hashes {
            all_bytes.extend_from_slice(&h[..]);
        }
        blake3::hash(&all_bytes).into()
    }
    
    pub fn state_root(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"AMUN_STATE_ROOT_V1");
        for qc in self.store.values() {
            bytes.extend_from_slice(&qc.hash.as_bytes()[..]);
        }
        blake3::hash(&bytes).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_simulation() {
        let mut sim1 = ByzantineSimulator::new(10, 3, 42);
        let mut sim2 = ByzantineSimulator::new(10, 3, 42);
        
        sim1.run(10);
        sim2.run(10);
        
        assert_eq!(sim1.equivocations_detected, sim2.equivocations_detected);
        assert_eq!(sim1.forks_detected, sim2.forks_detected);
        assert_eq!(sim1.transcript_hash(), sim2.transcript_hash());
        assert_eq!(sim1.state_root(), sim2.state_root());
    }
}
