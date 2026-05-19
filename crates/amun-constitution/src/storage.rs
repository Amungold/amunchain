// Write-Ahead Log (WAL) storage model.

use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WALEntryState {
    Writing = 0,
    Written = 1,
    Synced = 2,
    Applied = 3,
}

#[derive(Clone, Debug)]
pub struct WALEntry {
    pub sequence: u64,
    pub operation: Vec<u8, 256>,
    pub checksum: u32,
    pub state: WALEntryState,
}

impl WALEntry {
    pub fn new(sequence: u64, operation: &[u8]) -> Self {
        let mut op_vec = Vec::new();
        let len = operation.len().min(256);
        op_vec.extend_from_slice(&operation[..len]).ok();
        Self {
            sequence,
            operation: op_vec,
            checksum: 0,
            state: WALEntryState::Writing,
        }
    }
}
