// Protocol Constants - FROZEN for version 1
// All domain separators and chain identity constants.
pub const PROTOCOL_DOMAIN_LEAF: &[u8] = b"AMUN_LEAF_V1";
pub const PROTOCOL_DOMAIN_BRANCH: &[u8] = b"AMUN_BRANCH_V1";
pub const PROTOCOL_DOMAIN_WAL: &[u8] = b"AMUN_WAL_FRAME_V1";
pub const PROTOCOL_DOMAIN_LINEAGE: &[u8] = b"AMUN_LINEAGE_V1";
pub const PROTOCOL_DOMAIN_SNAPSHOT: &[u8] = b"AMUN_SNAPSHOT_V1";
pub const PROTOCOL_DOMAIN_MANIFEST: &[u8] = b"AMUN_MANIFEST_V1";
pub const PROTOCOL_DOMAIN_CHUNK: &[u8] = b"AMUN_CHUNK_V1";
pub const PROTOCOL_DOMAIN_CHUNK_MERKLE: &[u8] = b"AMUN_CHUNK_MERKLE_V1";
pub const PROTOCOL_DOMAIN_CONSTITUTION: &[u8] = b"AMUN_CONSTITUTION_V1";
pub const PROTOCOL_DOMAIN_CHUNK_PROOF: &[u8] = b"AMUN_CHUNK_PROOF_V1";
pub const PROTOCOL_CHAIN_ID: [u8; 32] = [
    // "AMUNCHAIN_MAINNET_V1" padded to 32 bytes
    0x41, 0x4d, 0x55, 0x4e, 0x43, 0x48, 0x41, 0x49, // AMUNCHAI
    0x4e, 0x5f, 0x4d, 0x41, 0x49, 0x4e, 0x4e, 0x45, // N_MAINNE
    0x54, 0x5f, 0x56, 0x31, 0x00, 0x00, 0x00, 0x00, // T_V1....
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ........
];
