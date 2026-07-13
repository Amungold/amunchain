#![allow(dead_code)]
use amun_networking::crypto_identity::PeerKeyPair;
use amun_networking::peer_identity::PeerId;
use amun_networking::validator_certificate::ValidatorCertificate;
use serde::{Deserialize, Serialize};

/// Handshake message sent when two nodes establish a connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub peer_id: PeerId,
    pub public_key: [u8; 32],
    pub certificate: ValidatorCertificate,
    pub genesis_hash: [u8; 32],
    pub protocol_version: u32,
    pub node_name: String,
    pub listen_port: u16,
}

impl HandshakeMessage {
    pub fn new(
        keypair: &PeerKeyPair,
        certificate: &ValidatorCertificate,
        genesis_hash: [u8; 32],
        node_name: &str,
        listen_port: u16,
    ) -> Self {
        Self {
            peer_id: keypair.peer_id(),
            public_key: keypair.verifying_key.to_bytes(),
            certificate: certificate.clone(),
            genesis_hash,
            protocol_version: 1,
            node_name: node_name.to_string(),
            listen_port,
        }
    }

    /// Verify the handshake message against our genesis and certificate expectations.
    pub fn verify(
        &self,
        our_genesis_hash: &[u8; 32],
        authority_public_key: &[u8; 32],
    ) -> Result<(), String> {
        // Verify genesis hash matches
        if self.genesis_hash != *our_genesis_hash {
            return Err(format!(
                "Genesis hash mismatch: peer has {} but we have {}",
                hex::encode(self.genesis_hash),
                hex::encode(*our_genesis_hash)
            ));
        }

        // Verify certificate is self-consistent
        if self.certificate.validator_id != self.peer_id {
            return Err("Certificate validator_id does not match peer_id".into());
        }

        if self.certificate.public_key != self.public_key {
            return Err("Certificate public_key does not match handshake public_key".into());
        }

        // Verify certificate signature
        if !self.certificate.verify(authority_public_key) {
            return Err("Certificate signature verification failed".into());
        }

        Ok(())
    }
}

/// Result of a successful peer handshake.
#[derive(Debug, Clone)]
pub struct AuthenticatedPeer {
    pub peer_id: PeerId,
    pub public_key: [u8; 32],
    pub node_name: String,
    pub listen_port: u16,
    pub protocol_version: u32,
}

impl AuthenticatedPeer {
    pub fn from_handshake(handshake: &HandshakeMessage) -> Self {
        Self {
            peer_id: handshake.peer_id,
            public_key: handshake.public_key,
            node_name: handshake.node_name.clone(),
            listen_port: handshake.listen_port,
            protocol_version: handshake.protocol_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_networking::crypto_identity::PeerKeyPair;
    use amun_networking::genesis_authority::{
        genesis_authority_keypair, genesis_authority_public_key,
    };

    fn create_test_handshake() -> (HandshakeMessage, [u8; 32]) {
        let keypair = PeerKeyPair::generate();
        let genesis_hash = [0x42u8; 32];
        let authority = genesis_authority_keypair();
        let cert = ValidatorCertificate::issue_v2(
            keypair.peer_id(),
            keypair.verifying_key.to_bytes(),
            0,
            [0u8; 32],
            &authority,
            0,
            0,
        );
        let handshake = HandshakeMessage::new(&keypair, &cert, genesis_hash, "test-node", 4001);
        (handshake, genesis_hash)
    }

    #[test]
    fn n22_5_valid_handshake_accepted() {
        let (handshake, genesis_hash) = create_test_handshake();
        assert!(handshake
            .verify(&genesis_hash, &genesis_authority_public_key())
            .is_ok());
    }

    #[test]
    fn n22_5_genesis_mismatch_rejected() {
        let (handshake, _) = create_test_handshake();
        let wrong_genesis = [0x99u8; 32];
        assert!(handshake
            .verify(&wrong_genesis, &genesis_authority_public_key())
            .is_err());
    }

    #[test]
    fn n22_5_tampered_certificate_rejected() {
        let (mut handshake, genesis_hash) = create_test_handshake();
        handshake.certificate.valid_from = 99999;
        assert!(handshake
            .verify(&genesis_hash, &genesis_authority_public_key())
            .is_err());
    }

    #[test]
    fn n22_5_authenticated_peer_creation() {
        let (handshake, genesis_hash) = create_test_handshake();
        handshake
            .verify(&genesis_hash, &genesis_authority_public_key())
            .unwrap();
        let peer = AuthenticatedPeer::from_handshake(&handshake);
        assert_eq!(peer.peer_id, handshake.peer_id);
        assert_eq!(peer.node_name, "test-node");
    }
}
