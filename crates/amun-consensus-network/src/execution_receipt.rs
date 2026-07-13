// N109.7A: Execution receipt — decouples verification layer from execution engine.
// Every validator produces this after re-executing a block.
// state_root is the SMT root after applying all transactions in the block.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionReceipt {
    pub state_root: [u8; 32],
    pub height: u64,
    pub block_hash: [u8; 32],
    pub success: bool,
}
