use amun_chain_position::ChainPosition;
use amun_consensus_messages::ConsensusVote;

#[derive(Debug, Clone)]
pub struct QuorumCertificate {
    pub position: ChainPosition,
    pub round: u64,
    pub block_hash: [u8; 32],
    pub parent_hash: [u8; 32],
    pub votes: Vec<ConsensusVote>,
    pub aggregated_signature: Option<[u8; 64]>,
}

impl QuorumCertificate {
    pub fn new(
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        parent_hash: [u8; 32],
        votes: Vec<ConsensusVote>,
    ) -> Self {
        Self {
            position,
            round,
            block_hash,
            parent_hash,
            votes,
            aggregated_signature: None,
        }
    }

    pub fn total_weight(&self) -> u64 {
        self.votes.iter().map(|v| v.weight).sum()
    }

    pub fn voter_ids(&self) -> Vec<u64> {
        self.votes.iter().map(|v| v.message.validator_id).collect()
    }
}
