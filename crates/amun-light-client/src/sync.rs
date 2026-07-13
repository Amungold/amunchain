/// Batch sync result for light client.
#[derive(Debug, Clone)]
pub struct SyncBatch {
    pub block_count: usize,
    pub synced_height: u64,
}

impl SyncBatch {
    pub fn new(block_count: usize) -> Self {
        Self {
            block_count,
            synced_height: block_count as u64,
        }
    }
}
