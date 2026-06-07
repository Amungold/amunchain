use std::collections::BTreeMap;
use crate::block::Block;

/// Deterministic finalized blockchain.
#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks_by_height: BTreeMap<u64, Block>,
    tip_height: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            blocks_by_height: BTreeMap::new(),
            tip_height: 0,
        };
        chain.blocks_by_height.insert(0, Block::genesis());
        chain
    }

    pub fn finalize_block(&mut self, block: Block) {
        let height = block.header.height;
        self.blocks_by_height.insert(height, block);
        if height > self.tip_height {
            self.tip_height = height;
        }
    }

    pub fn tip_height(&self) -> u64 { self.tip_height }
    pub fn get_block(&self, height: u64) -> Option<&Block> { self.blocks_by_height.get(&height) }
}
