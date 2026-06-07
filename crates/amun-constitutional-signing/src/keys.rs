use ed25519_dalek::{SigningKey, VerifyingKey, Signature};
use rand::RngCore;
use rand::rngs::OsRng;

pub struct ConstitutionalKeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl ConstitutionalKeyPair {
    pub fn generate() -> Self {
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();
        Self { signing_key, verifying_key }
    }

    pub fn verifying_key_hex(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    pub fn sign(&self, digest: &[u8; 32]) -> Signature {
        use ed25519_dalek::Signer;
        self.signing_key.sign(digest)
    }

    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
}
