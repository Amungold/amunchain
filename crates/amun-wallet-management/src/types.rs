use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A wallet address derived from a public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletAddress(pub String);

impl std::fmt::Display for WalletAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A 32-byte seed for key derivation.
///
/// **Security**: This type does NOT implement Serialize or Deserialize.
/// Export is only possible via explicit `to_hex()` / `from_hex()` methods.
/// Memory is zeroized on drop.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct WalletSeed(pub [u8; 32]);

impl WalletSeed {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn from_hex(hex_str: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_str).map_err(|e| format!("invalid hex seed: {}", e))?;
        if bytes.len() != 32 {
            return Err("seed must be 32 bytes".into());
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&bytes);
        Ok(Self(seed))
    }
}

impl std::fmt::Debug for WalletSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletSeed").field("length", &32).finish()
    }
}

/// An Ed25519 keypair.
///
/// **Security**: This type does NOT implement Serialize or Deserialize.
/// The private key is never exported except through an encrypted keystore.
/// Memory is zeroized on drop. The secret key is `pub(crate)` to prevent
/// external access outside this crate.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct WalletKeypair {
    pub public_key: [u8; 32],
    pub(crate) secret_key: [u8; 32],
}

impl WalletKeypair {
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key)
    }

    pub fn address(&self) -> WalletAddress {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_ADDRESS_V1");
        hasher.update(&self.public_key);
        let hash = hasher.finalize();
        WalletAddress(hex::encode(hash.as_bytes()))
    }

    /// Compute a verification hash that can be used to detect
    /// incorrect password decryption in a keystore.
    pub fn verification_hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_KEYSTORE_VERIFY_V1");
        hasher.update(&self.public_key);
        hasher.update(&self.secret_key);
        hex::encode(hasher.finalize().as_bytes())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystore {
    pub version: u32,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub salt: String,
    /// A hash of (public_key || secret_key) used to verify that the
    /// correct password was supplied during decryption.
    pub verification_hash: String,
}
