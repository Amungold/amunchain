use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstitutionalDecision { Admissible, Warning, Shadowed, Suffocated, Necromancy, Rejected }
impl ConstitutionalDecision { pub fn is_admissible(&self) -> bool { matches!(self, Self::Admissible | Self::Warning | Self::Shadowed) } }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorityProof { pub origin_present: bool, pub authority_root: [u8; 32] }
impl AuthorityProof { pub fn new(origin_present: bool, authority_root: [u8; 32]) -> Self { Self { origin_present, authority_root } } }
#[derive(Debug, Clone)]
pub struct ConstitutionalVote { pub validator_id: u64, pub block_height: u64, pub block_hash: [u8; 32], pub state_root: [u8; 32], pub receipts_root: [u8; 32], pub authority_proof: AuthorityProof, pub lineage_root: [u8; 32], pub suffocation_indicator: u64 }
impl ConstitutionalVote { pub fn new(vid: u64, height: u64) -> Self { Self { validator_id: vid, block_height: height, block_hash: [0u8; 32], state_root: [0u8; 32], receipts_root: [0u8; 32], authority_proof: AuthorityProof::new(true, [0u8; 32]), lineage_root: [0u8; 32], suffocation_indicator: 0 } } pub fn has_origin(&self) -> bool { self.authority_proof.origin_present } pub fn fork_key(&self) -> ([u8; 32], [u8; 32]) { (self.authority_proof.authority_root, self.lineage_root) } }
#[derive(Debug, Clone, Default)]
pub struct ConstitutionalVoteSet { pub votes: Vec<ConstitutionalVote>, pub block_height: u64 }
impl ConstitutionalVoteSet { pub fn new(height: u64) -> Self { Self { votes: Vec::new(), block_height: height } } pub fn add_vote(&mut self, vote: ConstitutionalVote) { self.votes.push(vote); } pub fn analyze_quorum(&self, thresh: usize) -> QuorumAnalysis { let mut counts: BTreeMap<([u8;32],[u8;32]), usize> = BTreeMap::new(); for v in &self.votes { *counts.entry(v.fork_key()).or_default() += 1; } QuorumAnalysis { has_quorum: counts.values().any(|&c| c >= thresh), has_topology_divergence: counts.len() > 1, fork_key_count: counts.len(), max_quorum_size: counts.values().max().copied().unwrap_or(0) } } }
#[derive(Debug, Clone)]
pub struct QuorumAnalysis { pub has_quorum: bool, pub has_topology_divergence: bool, pub fork_key_count: usize, pub max_quorum_size: usize }
