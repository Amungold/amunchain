use serde::{Deserialize, Serialize};

/// 256-bit hash – used for block hashes, certificate IDs, etc.
pub type Hash256 = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProposal {
    pub height: u64,
    pub block_hash: Hash256,
    pub proposer: Hash256,
    pub round: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub height: u64,
    pub block_hash: Hash256,
    pub voter: Hash256,
    pub round: u64,
    pub vote_type: VoteType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoteType {
    Prevote,
    Precommit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumCertificate {
    pub height: u64,
    pub block_hash: Hash256,
    pub round: u64,
    pub aggregated_signature: Vec<u8>,
    pub signers_bitmap: Vec<u8>,
}
