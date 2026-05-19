pub struct StorageLaw;

impl StorageLaw {
    pub const WAL_MAX_ENTRIES: usize = if cfg!(test) { 4 } else { 10_000 };
    pub const WAL_CHECKPOINT_INTERVAL: u64 = 1000;
    pub const SNAPSHOT_MAX_SIZE: usize = 1_048_576;
    pub const SNAPSHOT_INTERVAL_BLOCKS: u64 = 100;
    pub const PAGE_SIZE: usize = 4096;
    pub const MAX_PAGES: usize = 256;
    pub const MAX_ENTRIES_PER_COMMIT: usize = if cfg!(test) { 16 } else { 500 };
}
