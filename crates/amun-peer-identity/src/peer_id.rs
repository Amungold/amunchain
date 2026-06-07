use serde::{Deserialize, Serialize};

/// A constitutional peer identity is derived from the peer's public key
/// and the genesis hash of the civilisation it belongs to.  This binds
/// the identity to a specific constitutional lineage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstitutionalPeerId {
    pub peer_id: String,         // hex-encoded BLAKE3 digest
    pub public_key_hex: String,
    pub genesis_hash: String,    // lineage anchor
}

impl ConstitutionalPeerId {
    /// Create a new peer identity.  The `peer_id` is computed as
    /// BLAKE3(public_key_hex || genesis_hash).
    pub fn new(public_key_hex: String, genesis_hash: String) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(b"AMUN_PEER_ID_V1");
        h.update(public_key_hex.as_bytes());
        h.update(genesis_hash.as_bytes());
        let id = hex::encode(h.finalize().as_bytes());
        Self { peer_id: id, public_key_hex, genesis_hash }
    }
}
