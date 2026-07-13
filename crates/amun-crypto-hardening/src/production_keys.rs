use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct ValidatorKeypair {
    pub public_key: [u8; 32],
    secret_key: [u8; 32],
}

impl ValidatorKeypair {
    pub fn generate() -> Self {
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let public_key = signing_key.verifying_key().to_bytes();
        Self {
            public_key,
            secret_key: signing_key.to_bytes(),
        }
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let signing_key = SigningKey::from_bytes(&self.secret_key);
        signing_key.sign(message).to_bytes()
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorPublicKey {
    pub key_bytes: [u8; 32],
}

impl ValidatorPublicKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { key_bytes: bytes }
    }

    pub fn verify(&self, message: &[u8], signature: &[u8; 64]) -> bool {
        let verifying_key = match VerifyingKey::from_bytes(&self.key_bytes) {
            Ok(vk) => vk,
            Err(_) => return false,
        };
        let sig = Signature::from_bytes(signature);
        verifying_key.verify(message, &sig).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n58_generate_and_sign() {
        let kp = ValidatorKeypair::generate();
        let msg = b"AmunChain N58 cryptographic hardening";
        let sig = kp.sign(msg);
        let pubkey = ValidatorPublicKey::from_bytes(kp.public_key);
        assert!(pubkey.verify(msg, &sig));
    }

    #[test]
    fn n58_tampered_signature_rejected() {
        let kp = ValidatorKeypair::generate();
        let msg = b"AmunChain N58";
        let mut sig = kp.sign(msg);
        sig[0] ^= 1;
        let pubkey = ValidatorPublicKey::from_bytes(kp.public_key);
        assert!(!pubkey.verify(msg, &sig));
    }

    #[test]
    fn n58_different_message_rejected() {
        let kp = ValidatorKeypair::generate();
        let sig = kp.sign(b"message A");
        let pubkey = ValidatorPublicKey::from_bytes(kp.public_key);
        assert!(!pubkey.verify(b"message B", &sig));
    }
}
