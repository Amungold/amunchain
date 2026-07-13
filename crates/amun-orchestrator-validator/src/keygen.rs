use amun_orchestrator_core::types::{PeerId, PublicKey, ValidatorId};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

/// Generated keypair for a validator.
pub struct ValidatorKeypair {
    pub secret_key: [u8; 32],
    pub public_key: [u8; 32],
}

/// Generate a new Ed25519 keypair.
pub fn generate_keypair() -> ValidatorKeypair {
    let signing_key = SigningKey::generate(&mut OsRng);
    let secret = signing_key.to_bytes();
    let public = signing_key.verifying_key().to_bytes();
    ValidatorKeypair {
        secret_key: secret,
        public_key: public,
    }
}

/// Derive PeerId from public key: SHA-256(pubkey).
pub fn derive_peer_id(keypair: &ValidatorKeypair) -> PeerId {
    let hash = Sha256::digest(keypair.public_key);
    let mut id = [0u8; 32];
    id.copy_from_slice(&hash);
    PeerId(id)
}

/// Derive ValidatorId from public key: SHA-256("validator" || pubkey).
pub fn derive_validator_id(keypair: &ValidatorKeypair) -> ValidatorId {
    let mut hasher = Sha256::new();
    hasher.update(b"validator");
    hasher.update(keypair.public_key);
    let hash = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&hash);
    ValidatorId(id)
}

/// Get the raw public key.
pub fn derive_public_key(keypair: &ValidatorKeypair) -> PublicKey {
    PublicKey(keypair.public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = generate_keypair();
        assert_eq!(kp.public_key.len(), 32);
        assert_eq!(kp.secret_key.len(), 32);
    }

    #[test]
    fn test_peer_id_determinism() {
        let kp = generate_keypair();
        let id1 = derive_peer_id(&kp);
        let id2 = derive_peer_id(&kp);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_validator_id_differs_from_peer_id() {
        let kp = generate_keypair();
        let peer = derive_peer_id(&kp);
        let validator = derive_validator_id(&kp);
        // PeerId and ValidatorId are different types by design
        assert_ne!(peer.0, validator.0);
    }
}
