use crate::peer_identity::PeerId;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

/// A cryptographic key pair for a network peer.
pub struct PeerKeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl PeerKeyPair {
    /// Generate a new random key pair.
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Create a keypair from a 32-byte seed.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Export the 32-byte seed for persistent storage.
    pub fn to_seed(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Derive the PeerId from the verifying key.
    pub fn peer_id(&self) -> PeerId {
        PeerId::from_bytes(self.verifying_key.to_bytes())
    }

    /// Sign a message and return the signature bytes.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// Verify a signature against a verifying key.
    pub fn verify(key: &[u8; 32], message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }
        if let Ok(verifying_key) = VerifyingKey::from_bytes(key) {
            let Ok(sig) = Signature::from_slice(signature) else {
                return false;
            };
            verifying_key.verify(message, &sig).is_ok()
        } else {
            false
        }
    }
}

/// A signed message ready for network transmission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMessage {
    pub sender_public_key: [u8; 32],
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
}

impl SignedMessage {
    /// Create a signed message using a key pair.
    pub fn sign(keypair: &PeerKeyPair, payload: Vec<u8>) -> Self {
        let signature = keypair.sign(&payload);
        Self {
            sender_public_key: keypair.verifying_key.to_bytes(),
            payload,
            signature,
        }
    }

    /// Verify the signature on this message.
    pub fn verify(&self) -> bool {
        PeerKeyPair::verify(&self.sender_public_key, &self.payload, &self.signature)
    }

    /// Get the PeerId of the sender.
    pub fn sender_peer_id(&self) -> PeerId {
        PeerId::from_bytes(self.sender_public_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n20_5_keypair_generation() {
        let keypair = PeerKeyPair::generate();
        let peer_id = keypair.peer_id();
        assert_eq!(peer_id.0, keypair.verifying_key.to_bytes());
    }

    #[test]
    fn n20_5_sign_and_verify() {
        let keypair = PeerKeyPair::generate();
        let message = b"AmunChain constitutional message";
        let signature = keypair.sign(message);
        assert!(PeerKeyPair::verify(
            &keypair.verifying_key.to_bytes(),
            message,
            &signature
        ));
    }

    #[test]
    fn n20_5_tampered_message_rejected() {
        let keypair = PeerKeyPair::generate();
        let message = b"original message";
        let signature = keypair.sign(message);
        assert!(!PeerKeyPair::verify(
            &keypair.verifying_key.to_bytes(),
            b"tampered message",
            &signature
        ));
    }

    #[test]
    fn n20_5_wrong_signer_rejected() {
        let alice = PeerKeyPair::generate();
        let bob = PeerKeyPair::generate();
        let message = b"message from alice";
        let signature = alice.sign(message);
        assert!(!PeerKeyPair::verify(
            &bob.verifying_key.to_bytes(),
            message,
            &signature
        ));
    }

    #[test]
    fn n20_5_signed_message_roundtrip() {
        let keypair = PeerKeyPair::generate();
        let payload = b"test payload".to_vec();
        let signed = SignedMessage::sign(&keypair, payload.clone());
        assert!(signed.verify());
        assert_eq!(signed.sender_peer_id().0, keypair.verifying_key.to_bytes());
    }

    #[test]
    fn n20_5_serialize_deserialize_signed_message() {
        let keypair = PeerKeyPair::generate();
        let signed = SignedMessage::sign(&keypair, b"serialization test".to_vec());
        let json = serde_json::to_string(&signed).unwrap();
        let decoded: SignedMessage = serde_json::from_str(&json).unwrap();
        assert!(decoded.verify());
    }
}
