use crate::types::{Vote, VoteType, QuorumCertificate, Hash256};
use crate::validator::ValidatorSet;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VoteKey { height: u64, round: u64, vote_type: VoteType, block_hash: Hash256 }
impl VoteKey { fn from_vote(v: &Vote) -> Self { Self { height: v.height, round: v.round, vote_type: v.vote_type, block_hash: v.block_hash } } }

pub struct VoteCollector { votes: HashMap<VoteKey, Vec<Vote>>, formed_qcs: HashSet<VoteKey> }
impl VoteCollector {
    pub fn new() -> Self { Self { votes: HashMap::new(), formed_qcs: HashSet::new() } }
    pub fn add_vote(&mut self, vote: Vote, validator_set: &ValidatorSet) -> Option<QuorumCertificate> {
        let key = VoteKey::from_vote(&vote);
        if self.formed_qcs.contains(&key) { return None; }
        let entry = self.votes.entry(key.clone()).or_default();
        if entry.iter().any(|v| v.voter == vote.voter) { return None; }
        entry.push(vote);
        let total_power: u64 = entry.iter().map(|v| validator_set.power_of(&v.voter)).sum();
        if validator_set.has_quorum(total_power) {
            self.formed_qcs.insert(key);
            let first = &entry[0];
            Some(QuorumCertificate { height: first.height, block_hash: first.block_hash, round: first.round, aggregated_signature: Vec::new(), signers_bitmap: Vec::new() })
        } else { None }
    }
    pub fn reset(&mut self) { self.votes.clear(); self.formed_qcs.clear(); }
}

impl Default for VoteCollector {
    fn default() -> Self { Self::new() }
}
