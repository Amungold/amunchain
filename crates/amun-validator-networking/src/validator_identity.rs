use serde::{Deserialize, Serialize};

/// A validator's network identity, bound to its cryptographic keypair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorIdentity {
    pub validator_id: [u8; 32],
    pub public_key: [u8; 32],
    pub address: String,
    pub port: u16,
}

impl ValidatorIdentity {
    pub fn new(validator_id: [u8; 32], public_key: [u8; 32], address: String, port: u16) -> Self {
        Self {
            validator_id,
            public_key,
            address,
            port,
        }
    }

    /// Network fingerprint for peer discovery.
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_VALIDATOR_FINGERPRINT_V1");
        hasher.update(&self.validator_id);
        hasher.update(&self.public_key);
        hasher.update(self.address.as_bytes());
        hasher.update(&self.port.to_le_bytes());
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }
}
