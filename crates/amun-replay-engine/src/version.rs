/// Replay protocol version - FROZEN for protocol v1.
/// Any change requires constitutional amendment and golden value regeneration.
pub const REPLAY_PROTOCOL_VERSION: u32 = 1;

/// Constitutional hash domain for protocol version binding.
pub const PROTOCOL_VERSION_DOMAIN: &[u8] = b"AMUN_REPLAY_PROTOCOL_V1";
