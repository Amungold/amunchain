use amun_chain_position::ChainPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConsensusPhase {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusVote {
    pub message: ConsensusMessage,
    pub signature: [u8; 64],
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
        Self {
            message: ConsensusMessage {
                validator_id,
                position,
                round,
                phase,
                block_hash,
            },
            signature,
            weight,
        }
    }
}
