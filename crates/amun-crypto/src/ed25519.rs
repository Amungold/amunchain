use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use zeroize::Zeroize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    InvalidDomain,
    MalformedSignature,
    VerificationFailed,
    InvalidPublicKey,
}

pub struct Ed25519Signer {
    secret_bytes: [u8; 32],
    public: VerifyingKey,
}

impl Ed25519Signer {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        Self {
            secret_bytes: sk.to_bytes(),
            public: pk,
        }
    }

    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let sk = SigningKey::from_bytes(seed);
        let pk = sk.verifying_key();
        Self {
            secret_bytes: *seed,
            public: pk,
        }
    }

    fn signing_key(&self) -> SigningKey {
        SigningKey::from_bytes(&self.secret_bytes)
    }

    pub fn sign(&self, msg: &[u8], domain: &[u8], chain_id: u64) -> Result<[u8; 64], CryptoError> {
        if domain.is_empty() || domain.len() > 128 {
            return Err(CryptoError::InvalidDomain);
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain);
        hasher.update(&chain_id.to_le_bytes());
        hasher.update(msg);
        let digest = hasher.finalize();
        Ok(self.signing_key().sign(digest.as_bytes()).to_bytes())
    }

    pub fn verify(
        pk: &[u8; 32],
        msg: &[u8],
        sig: &[u8; 64],
        domain: &[u8],
        chain_id: u64,
    ) -> Result<(), CryptoError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain);
        hasher.update(&chain_id.to_le_bytes());
        hasher.update(msg);
        let digest = hasher.finalize();

        let vk = VerifyingKey::from_bytes(pk).map_err(|_| CryptoError::InvalidPublicKey)?;
        let s = Signature::from_slice(sig).map_err(|_| CryptoError::MalformedSignature)?;
        vk.verify(digest.as_bytes(), &s)
            .map_err(|_| CryptoError::VerificationFailed)
    }

    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.secret_bytes
    }
}

impl Zeroize for Ed25519Signer {
    fn zeroize(&mut self) {
        self.secret_bytes.zeroize();
    }
}

impl Drop for Ed25519Signer {
    fn drop(&mut self) {
        self.secret_bytes.zeroize();
    }
}

impl std::fmt::Debug for Ed25519Signer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ed25519Signer")
            .field("public", &hex::encode(self.public.as_bytes()))
            .field("secret_bytes", &"[REDACTED]")
            .finish()
    }
}
