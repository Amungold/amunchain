/// Unique identifier of a validator, derived from its public key.
pub type ValidatorId = [u8; 32];

/// Derive a ValidatorId from a 32-byte Ed25519 public key.
/// Uses BLAKE3 with domain separator "AMUN_VALIDATOR_ID_V1".
pub fn derive_validator_id(public_key: &[u8; 32]) -> ValidatorId {
    let mut h = blake3::Hasher::new();
    h.update(b"AMUN_VALIDATOR_ID_V1");
    h.update(public_key);
    let mut out = [0u8; 32];
    out.copy_from_slice(h.finalize().as_bytes());
    out
}

/// Derive a PeerId (network identifier) from the public key and genesis hash.
/// This mirrors the derivation in amun-networking.
pub fn derive_peer_id(public_key: &[u8; 32], genesis_hash: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(b"AMUN_PEER_ID_V1");
    h.update(public_key);
    h.update(genesis_hash);
    let mut out = [0u8; 32];
    out.copy_from_slice(h.finalize().as_bytes());
    out
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n105_validator_id_deterministic() {
        let pk = [0x42u8; 32];
        let id1 = derive_validator_id(&pk);
        let id2 = derive_validator_id(&pk);
        assert_eq!(id1, id2);
        // not equal to raw public key
        assert_ne!(id1, pk);
    }
}
