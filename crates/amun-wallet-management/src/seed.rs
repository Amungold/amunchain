use rand::rngs::OsRng;
use rand::RngCore;

use crate::types::WalletSeed;

/// Generate a new random 32-byte seed.
pub fn generate_seed() -> WalletSeed {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    WalletSeed(bytes)
}

/// Import a seed from a hex string.
pub fn import_seed_from_hex(hex_str: &str) -> Result<WalletSeed, String> {
    WalletSeed::from_hex(hex_str)
}

/// Export a seed to a hex string. Use with caution — the caller
/// is responsible for secure handling.
pub fn export_seed_to_hex(seed: &WalletSeed) -> String {
    seed.to_hex()
}
