// Constitutional block limits. SINGLE SOURCE OF TRUTH.
// max_tx_bytes removed — BlockBody stores only tx hashes (32 bytes each).
// Tx byte limits belong in the mempool/state layer, not the block layer.

pub const CONSTITUTIONAL_MAX_BLOCK_BYTES: usize = 1_048_576;
pub const CONSTITUTIONAL_MAX_TX_COUNT: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockLimits {
    pub max_transactions: u32,
    pub max_block_bytes: u32,
}

impl BlockLimits {
    pub const fn constitutional() -> Self {
        Self {
            max_transactions: CONSTITUTIONAL_MAX_TX_COUNT as u32,
            max_block_bytes: CONSTITUTIONAL_MAX_BLOCK_BYTES as u32,
        }
    }
}
