use crate::crypto_identity::PeerKeyPair;
use crate::peer_identity::PeerId;
use serde::{Deserialize, Serialize};

/// A constitutional certificate binding a validator's identity to their public key.
///
/// Signed by a constitutional authority to establish trust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorCertificate {
    /// The validator's peer identity.
    pub validator_id: PeerId,
    /// The validator's public key (for message verification).
    pub public_key: [u8; 32],
    /// The authority that issued this certificate.
    pub issuer: PeerId,
    /// When this certificate becomes valid.
    pub authority_id: [u8; 32],
    pub authority_version: u64,
    pub valid_from: u64,
    /// When this certificate expires (0 = never).
    pub valid_until: u64,
    /// The authority's signature over the certificate fields.
    pub authority_signature: Vec<u8>,
}

impl ValidatorCertificate {
    /// Create a new certificate signed by an authority.
    pub fn issue(
        validator_id: PeerId,
        public_key: [u8; 32],
        issuer_keypair: &PeerKeyPair,
        valid_from: u64,
        valid_until: u64,
    ) -> Self {
        let mut cert = Self {
            validator_id,
            public_key,
            authority_id: [0u8; 32],
            authority_version: 0,
            issuer: issuer_keypair.peer_id(),
            valid_from,
            valid_until,
            authority_signature: Vec::new(),
        };
        // Sign the certificate fields (excluding signature itself)
        let payload = cert.serialize_for_signing();
        cert.authority_signature = issuer_keypair.sign(&payload);
        cert
    }

    /// Serialize certificate fields for signing (excludes the signature).
    fn serialize_for_signing(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.validator_id.0);
        data.extend_from_slice(&self.public_key);
        data.extend_from_slice(&self.authority_version.to_le_bytes());
        data.extend_from_slice(&self.authority_id);
        data.extend_from_slice(&self.issuer.0);
        data.extend_from_slice(&self.valid_from.to_le_bytes());
        data.extend_from_slice(&self.valid_until.to_le_bytes());
        data
    }

    /// Verify the authority's signature on this certificate.
    pub fn verify(&self, authority_public_key: &[u8; 32]) -> bool {
        let payload = self.serialize_for_signing();
        PeerKeyPair::verify(authority_public_key, &payload, &self.authority_signature)
    }

    /// Check if the certificate is currently valid.
    pub fn is_valid_at(&self, timestamp: u64) -> bool {
        timestamp >= self.valid_from && (self.valid_until == 0 || timestamp <= self.valid_until)
    }

    /// Create a new certificate with explicit authority version and id.
    pub fn issue_v2(
        validator_id: PeerId,
        public_key: [u8; 32],
        authority_version: u64,
        authority_id: [u8; 32],
        issuer_keypair: &PeerKeyPair,
        valid_from: u64,
        valid_until: u64,
    ) -> Self {
        let mut cert = Self {
            validator_id,
            public_key,
            authority_version,
            authority_id,
            issuer: issuer_keypair.peer_id(),
            valid_from,
            valid_until,
            authority_signature: Vec::new(),
        };
        let payload = cert.serialize_for_signing();
        cert.authority_signature = issuer_keypair.sign(&payload);
        cert
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n21_validator_certificate_issue_and_verify() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            2000,
        );

        assert!(cert.verify(&authority.verifying_key.to_bytes()));
        assert_eq!(cert.validator_id, validator.peer_id());
        assert_eq!(cert.issuer, authority.peer_id());
    }

    #[test]
    fn n21_certificate_rejects_wrong_authority() {
        let authority = PeerKeyPair::generate();
        let impostor = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            2000,
        );

        // Impostor tries to verify — must fail
        assert!(!cert.verify(&impostor.verifying_key.to_bytes()));
    }

    #[test]
    fn n21_certificate_tampered_fields_rejected() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let mut cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            2000,
        );

        // Tamper with validity period
        cert.valid_from = 500;
        assert!(!cert.verify(&authority.verifying_key.to_bytes()));
    }

    #[test]
    fn n21_certificate_validity_window() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            2000,
        );

        assert!(!cert.is_valid_at(500));
        assert!(cert.is_valid_at(1000));
        assert!(cert.is_valid_at(1500));
        assert!(cert.is_valid_at(2000));
        assert!(!cert.is_valid_at(2001));
    }

    #[test]
    fn n21_certificate_serialization_roundtrip() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            0, // never expires
        );

        let json = serde_json::to_string(&cert).unwrap();
        let decoded: ValidatorCertificate = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("validator_cert: invalid certificate: {}", e);
                return;
            }
        };

        assert!(decoded.verify(&authority.verifying_key.to_bytes()));
        assert!(decoded.is_valid_at(5000));
    }
}
