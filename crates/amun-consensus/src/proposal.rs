use amun_kernel_types::{PublicHash32, Epoch};
use amun_block::Block;
use amun_consensus_types::ConsensusRound;

#[derive(Clone, Debug)]
pub struct BlockProposal {
    pub block: Block,
    pub epoch: Epoch,
    pub round: ConsensusRound,
    pub proposer: usize,
    pub signature: [u8; 96],
}

impl BlockProposal {
    pub fn new(block: Block, epoch: Epoch, round: ConsensusRound, proposer: usize) -> Self {
        Self { block, epoch, round, proposer, signature: [0u8; 96] }
    }

    pub fn block_hash(&self) -> PublicHash32 {
        let id = self.block.compute_id();
        id.to_public_hash32()
    }
}
