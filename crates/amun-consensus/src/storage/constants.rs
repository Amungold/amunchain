pub const MAX_WAL_ENTRY_SIZE: usize = 1024 * 1024;
pub const MAX_TRANSACTION_WRITES: usize = 8192;
pub const MAX_SNAPSHOT_SIZE: usize = 256 * 1024 * 1024;
pub const STORAGE_VERSION: u64 = 1;
pub const WAL_MAGIC: u32 = 0x414D554E; // "AMUN"
pub const LRU_CACHE_SIZE: usize = 10000;
pub const MAX_OPEN_FILES: usize = 1000;
