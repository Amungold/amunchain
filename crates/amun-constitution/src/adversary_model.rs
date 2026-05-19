// Byzantine adversary capabilities and limitations.

#[derive(Clone, Debug)]
pub struct AdversaryCapabilities {
    pub max_corrupt_stake: u64,
    pub max_delay_rounds: u32,
    pub can_reorder: bool,
    pub can_drop: bool,
    pub can_partition: bool,
    pub can_eclipse: bool,
}

#[derive(Clone, Debug)]
pub struct AdversaryLimitations {
    pub cannot_forge_signatures: bool,
    pub cannot_find_hash_collisions: bool,
    pub cannot_break_blake3: bool,
    pub cannot_exceed_corruption_limit: bool,
    pub cannot_delay_indefinitely: bool,
    pub cannot_corrupt_retroactively: bool,
}
