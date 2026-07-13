use crate::message::{ConsensusMessage, ConsensusPhase};
use amun_chain_position::ChainPosition;

#[derive(Debug, Clone)]
pub struct ConsensusVote {
    pub message: ConsensusMessage,
    pub weight: u64,
}

impl ConsensusVote {
    pub fn new(
        validator_id: u64,
        position: ChainPosition,
        round: u64,
        phase: ConsensusPhase,
        block_hash: Option<[u8; 32]>,
        signature: [u8; 64],
        weight: u64,
    ) -> Self {
        let message = ConsensusMessage::new(validator_id, position, round, phase, block_hash, signature);
        Self { message, weight }
    }

    pub fn is_prevote(&self) -> bool { self.message.phase == ConsensusPhase::Prevote }
    pub fn is_precommit(&self) -> bool { self.message.phase == ConsensusPhase::Precommit }
    pub fn verify(&self) -> bool { self.message.verify() }
}
