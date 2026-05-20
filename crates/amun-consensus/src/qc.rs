use crate::validator::ValidatorSet;
use amun_consensus_types::{ConsensusPhase, ConsensusRound, ValidatorIndex};
use amun_kernel_types::PublicHash32;
use heapless::Vec;

#[derive(Clone, Debug)]
pub struct QuorumCert {
    pub phase: ConsensusPhase,
    pub round: ConsensusRound,
    pub block_hash: PublicHash32,
    pub signer_count: usize,
    pub signer_indices: Vec<ValidatorIndex, 256>,
    pub aggregate_signature: [u8; 96],
}

impl QuorumCert {
    pub fn new(phase: ConsensusPhase, round: ConsensusRound, block_hash: PublicHash32) -> Self {
        Self {
            phase,
            round,
            block_hash,
            signer_count: 0,
            signer_indices: Vec::new(),
            aggregate_signature: [0u8; 96],
        }
    }

    pub fn is_valid(&self, validator_set: &ValidatorSet) -> bool {
        validator_set.is_quorum(self.signer_count)
    }
}
