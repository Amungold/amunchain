use amun_chain_position::ChainPosition;
use amun_consensus_messages::ConsensusVote;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct QuorumCertificate {
    pub position: ChainPosition,
    pub round: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub votes: Vec<ConsensusVote>,
    pub total_weight: u64,
    pub certificate_hash: [u8; 32],
}

impl QuorumCertificate {
    pub fn new(
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        votes: Vec<ConsensusVote>,
    ) -> Self {
        let total_weight: u64 = votes.iter().map(|v| v.weight).sum();

        let mut h = Hasher::new();
        h.update(b"AMUN_QC_V1");
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        h.update(&block_hash);
        h.update(&state_root);
        h.update(&total_weight.to_le_bytes());
        for vote in &votes {
            h.update(&vote.message.message_hash);
        }
        let mut certificate_hash = [0u8; 32];
        certificate_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { position, round, block_hash, state_root, votes, total_weight, certificate_hash }
    }

    pub fn verify(&self) -> bool {
        let total: u64 = self.votes.iter().map(|v| v.weight).sum();
        if total != self.total_weight { return false; }

        let mut h = Hasher::new();
        h.update(b"AMUN_QC_V1");
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        h.update(&self.block_hash);
        h.update(&self.state_root);
        h.update(&self.total_weight.to_le_bytes());
        for vote in &self.votes {
            h.update(&vote.message.message_hash);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.certificate_hash
    }
}
