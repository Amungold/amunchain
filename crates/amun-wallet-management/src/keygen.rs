use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::types::{WalletKeypair, WalletSeed};

/// Generate a new Ed25519 keypair from OS randomness.
pub fn generate_keypair() -> WalletKeypair {
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    WalletKeypair {
        public_key: verifying_key.to_bytes(),
        secret_key: signing_key.to_bytes(),
    }
}

/// Derive a keypair from a 32-byte seed deterministically.
pub fn keypair_from_seed(seed: &WalletSeed) -> WalletKeypair {
    let signing_key = SigningKey::from_bytes(&seed.0);
    let verifying_key = signing_key.verifying_key();
    WalletKeypair {
        public_key: verifying_key.to_bytes(),
        secret_key: signing_key.to_bytes(),
    }
}
