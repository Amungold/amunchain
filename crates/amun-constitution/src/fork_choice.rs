// Formal fork-choice function.

use amun_kernel_types::BlockHash;
use heapless::Vec;

#[derive(Clone, Debug)]
pub struct ForkChoiceState {
    pub finalized_block: BlockHash,
    pub justified_block: BlockHash,
    pub head: BlockHash,
}

#[derive(Clone, Debug)]
pub struct PreferredChain {
    pub head: BlockHash,
    pub path: Vec<BlockHash, 64>,
}

pub struct BlockDAG;

impl BlockDAG {
    pub fn children_of(&self, _block: &BlockHash) -> heapless::Vec<BlockHash, 8> {
        heapless::Vec::new()
    }
    pub fn votes_for(&self, _block: &BlockHash) -> u64 {
        0
    }
}

pub struct ForkChoiceFunction;

impl ForkChoiceFunction {
    pub fn choose_chain(state: &ForkChoiceState, _dag: &BlockDAG) -> PreferredChain {
        let mut path = Vec::new();
        path.push(state.head).ok();
        PreferredChain {
            head: state.head,
            path,
        }
    }
}
