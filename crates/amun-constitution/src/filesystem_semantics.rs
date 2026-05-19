// Filesystem assumptions for WAL durability.

#[derive(Clone, Copy, Debug)]
pub struct FilesystemAssumptions {
    pub fdatasync_is_durable: bool,
    pub sector_atomicity_bytes: u32,
    pub independent_sectors: bool,
    pub crash_consistency: bool,
}

pub const LINUX_FILESYSTEM_ASSUMPTIONS: FilesystemAssumptions = FilesystemAssumptions {
    fdatasync_is_durable: true,
    sector_atomicity_bytes: 512,
    independent_sectors: true,
    crash_consistency: true,
};
