// Constitutional capacity constants.
// These are the SINGLE SOURCE OF TRUTH for all size limits in the kernel.
// Defined here (in amun-kernel-types) to avoid circular dependencies.

pub mod constitutional_capacity {
    // Container limits
    pub const MAX_SET_ITEMS: u32 = 256;
    pub const MAX_MAP_ENTRIES: u32 = 256;
    pub const MAX_SEQUENCE_LENGTH: u32 = 10_000;
    pub const MAX_NESTING_DEPTH: u8 = 8;
    pub const MAX_MESSAGE_BYTES: u32 = 1_048_576;

    // Canonical encoding limits
    pub const MAX_CANONICAL_ENCODING_SIZE: usize = 256;

    // Sort buffer size
    pub const SORT_BUFFER_SIZE: usize = 256;
}
