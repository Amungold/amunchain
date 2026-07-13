use amun_chain_position::ChainPosition;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct BlockProposal {
    pub proposer_id: u64,
    pub position: ChainPosition,
    pub round: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proposal_hash: [u8; 32],
}

impl BlockProposal {
    pub fn new(
        proposer_id: u64,
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_PROPOSAL_V1");
        h.update(&proposer_id.to_le_bytes());
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        h.update(&block_hash);
        h.update(&state_root);
        let mut proposal_hash = [0u8; 32];
        proposal_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { proposer_id, position, round, block_hash, state_root, proposal_hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_PROPOSAL_V1");
        h.update(&self.proposer_id.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        h.update(&self.block_hash);
        h.update(&self.state_root);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.proposal_hash
    }
}
