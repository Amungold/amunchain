use amun_chain_position::ChainPosition;
use blake3::Hasher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusPhase {
    Propose,
    Prevote,
    Precommit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusMessage {
    pub validator_id: u64,
    pub position: ChainPosition,
    pub round: u64,
    pub phase: ConsensusPhase,
    pub block_hash: Option<[u8; 32]>,
    pub signature: [u8; 64],
    pub message_hash: [u8; 32],
}

impl ConsensusMessage {
    pub fn new(
        validator_id: u64,
        position: ChainPosition,
        round: u64,
        phase: ConsensusPhase,
        block_hash: Option<[u8; 32]>,
        signature: [u8; 64],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_CONSENSUS_MSG_V1");
        h.update(&validator_id.to_le_bytes());
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        h.update(&[match phase {
            ConsensusPhase::Propose => 0u8,
            ConsensusPhase::Prevote => 1u8,
            ConsensusPhase::Precommit => 2u8,
        }]);
        if let Some(bh) = &block_hash {
            h.update(bh);
        }
        h.update(&signature);
        let mut message_hash = [0u8; 32];
        message_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { validator_id, position, round, phase, block_hash, signature, message_hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_CONSENSUS_MSG_V1");
        h.update(&self.validator_id.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        h.update(&[match self.phase {
            ConsensusPhase::Propose => 0u8,
            ConsensusPhase::Prevote => 1u8,
            ConsensusPhase::Precommit => 2u8,
        }]);
        if let Some(bh) = &self.block_hash {
            h.update(bh);
        }
        h.update(&self.signature);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.message_hash
    }
}
