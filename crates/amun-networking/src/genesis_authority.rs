use crate::crypto_identity::PeerKeyPair;

/// Canonical authority used by networking/tests.
pub const GENESIS_AUTHORITY_SEED: [u8; 32] = [7u8; 32];

pub fn genesis_authority_keypair() -> PeerKeyPair {
    PeerKeyPair::from_seed(GENESIS_AUTHORITY_SEED)
}

pub fn genesis_authority_public_key() -> [u8; 32] {
    genesis_authority_keypair().verifying_key.to_bytes()
}
