use amun_consensus_types::{ConsensusPhase, ConsensusRound, ValidatorIndex};
use amun_kernel_types::PublicHash32;

#[derive(Clone, Debug)]
pub struct ConsensusVote {
    pub phase: ConsensusPhase,
    pub round: ConsensusRound,
    pub block_hash: PublicHash32,
    pub validator_index: ValidatorIndex,
    pub signature: [u8; 96],
}

impl ConsensusVote {
    pub fn new(
        phase: ConsensusPhase,
        round: ConsensusRound,
        block_hash: PublicHash32,
        validator_index: ValidatorIndex,
    ) -> Self {
        Self {
            phase,
            round,
            block_hash,
            validator_index,
            signature: [0u8; 96],
        }
    }

    pub fn signing_bytes(&self) -> [u8; 66] {
        let mut buf = [0u8; 66];
        buf[0] = self.phase as u8;
        let round_bytes = self.round.value().to_le_bytes();
        buf[1..9].copy_from_slice(&round_bytes);
        buf[9..41].copy_from_slice(&self.block_hash.0);
        let idx_bytes = self.validator_index.value().to_le_bytes();
        buf[41..43].copy_from_slice(&idx_bytes);
        buf
    }
}
