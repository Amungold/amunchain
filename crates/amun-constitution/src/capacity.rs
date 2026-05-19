// Protocol capacity types — re-exports from constitutional capacity constants.

pub use amun_kernel_types::constitutional_capacity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolCapacities {
    pub max_set_items: u32,
    pub max_map_entries: u32,
    pub max_sequence_length: u32,
    pub max_nesting_depth: u8,
    pub max_message_bytes: u32,
}

impl ProtocolCapacities {
    pub const fn constitutional() -> Self {
        Self {
            max_set_items: constitutional_capacity::MAX_SET_ITEMS,
            max_map_entries: constitutional_capacity::MAX_MAP_ENTRIES,
            max_sequence_length: constitutional_capacity::MAX_SEQUENCE_LENGTH,
            max_nesting_depth: constitutional_capacity::MAX_NESTING_DEPTH,
            max_message_bytes: constitutional_capacity::MAX_MESSAGE_BYTES,
        }
    }

    pub fn verify_compatible(&self, other: &Self) -> Result<(), &'static str> {
        if self.max_set_items != other.max_set_items
            || self.max_map_entries != other.max_map_entries
            || self.max_sequence_length != other.max_sequence_length
            || self.max_nesting_depth != other.max_nesting_depth
            || self.max_message_bytes != other.max_message_bytes
        {
            return Err("Protocol capacities mismatch");
        }
        Ok(())
    }
}

// Runtime overlay limits
pub const MAX_STATE_ENTRIES: usize = 256;
pub const MAX_OVERLAY_WRITES: usize = 256;
pub const MAX_OVERLAY_MERGED_ENTRIES: usize = MAX_STATE_ENTRIES + MAX_OVERLAY_WRITES;
