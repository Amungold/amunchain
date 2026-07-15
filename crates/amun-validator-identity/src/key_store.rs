use amun_validator_api::error::{IdentityError, IdentityErrorCode, PlatformError, PlatformResult};

/// Single source of truth for validator keys.
/// Handles signing only. Verification is done by AuthorityVerifier or external verifiers.
pub struct KeyStore {
    private_key: [u8; 32],
    public_key: [u8; 32],
}

impl KeyStore {
    pub fn new(private_key: [u8; 32], public_key: [u8; 32]) -> Self {
        KeyStore {
            private_key,
            public_key,
        }
    }

    /// Generate a new key pair (stub).
    pub fn generate() -> Self {
        let mut sk = [0u8; 32];
        let mut pk = [0u8; 32];
        for i in 0..32 {
            sk[i] = (i as u8).wrapping_add(1);
            pk[i] = (i as u8).wrapping_add(100);
        }
        KeyStore {
            private_key: sk,
            public_key: pk,
        }
    }

    /// TODO: Read actual file when key format is finalized.
    pub fn load_from_file(_path: &str) -> PlatformResult<Self> {
        Ok(Self::generate())
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key
    }

    /// Sign a message with the private key (stub implementation).
    pub fn sign(&self, message: &[u8]) -> PlatformResult<Vec<u8>> {
        if message.is_empty() {
            return Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::SignatureInvalid,
                "Cannot sign empty message".into(),
            )));
        }
        let mut sig = Vec::with_capacity(64);
        sig.extend_from_slice(&self.private_key);
        let len = message.len().min(32);
        sig.extend_from_slice(&message[..len]);
        while sig.len() < 64 {
            sig.push(0);
        }
        Ok(sig)
    }
}
