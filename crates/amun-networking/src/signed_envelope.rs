use crate::crypto_identity::{PeerKeyPair, SignedMessage};
use crate::envelope::Envelope;
use crate::peer_identity::PeerId;
use serde::{Deserialize, Serialize};

/// A cryptographically signed network message.
/// Wraps a SignedMessage for transmission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEnvelope {
    pub sender_peer_id: PeerId,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
}

impl SignedEnvelope {
    /// Create a signed envelope using a key pair.
    pub fn sign(keypair: &PeerKeyPair, envelope: &Envelope) -> Self {
        let payload = serde_json::to_vec(envelope).unwrap_or_default();
        let signed = SignedMessage::sign(keypair, payload);
        Self {
            sender_peer_id: signed.sender_peer_id(),
            payload: signed.payload,
            signature: signed.signature,
        }
    }

    /// Verify the envelope signature.
    pub fn verify(&self) -> bool {
        PeerKeyPair::verify(&self.sender_peer_id.0, &self.payload, &self.signature)
    }
}

/// A message directed to a specific peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedMessage {
    pub to: PeerId,
    pub signed_envelope: SignedEnvelope,
}

impl DirectedMessage {
    pub fn new(to: PeerId, signed_envelope: SignedEnvelope) -> Self {
        Self {
            to,
            signed_envelope,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_identity::PeerKeyPair;
    use crate::envelope::Envelope;

    #[test]
    fn n20_6_signed_envelope_verification() {
        let keypair = PeerKeyPair::generate();
        let envelope = Envelope {
            sender: "test".into(),
            recipient: String::new(),
            sequence: 1,
            timestamp: 1000,
            message_type: "test".into(),
            payload: b"hello".to_vec(),
        };
        let signed = SignedEnvelope::sign(&keypair, &envelope);
        assert!(signed.verify());
    }

    #[test]
    fn n20_6_tampered_envelope_rejected() {
        let keypair = PeerKeyPair::generate();
        let envelope = Envelope {
            sender: "test".into(),
            recipient: String::new(),
            sequence: 1,
            timestamp: 1000,
            message_type: "test".into(),
            payload: b"hello".to_vec(),
        };
        let mut signed = SignedEnvelope::sign(&keypair, &envelope);
        signed.payload = b"tampered".to_vec();
        assert!(!signed.verify());
    }

    #[test]
    fn n20_6_directed_message_roundtrip() {
        let alice = PeerKeyPair::generate();
        let bob = PeerKeyPair::generate();
        let bob_id = bob.peer_id();

        let envelope = Envelope {
            sender: "alice".into(),
            recipient: "bob".into(),
            sequence: 1,
            timestamp: 1000,
            message_type: "test".into(),
            payload: b"hello bob".to_vec(),
        };
        let signed = SignedEnvelope::sign(&alice, &envelope);
        let directed = DirectedMessage::new(bob_id, signed.clone());

        assert_eq!(directed.to, bob_id);
        assert!(directed.signed_envelope.verify());
    }
}
