use std::collections::BTreeMap;
use crate::peer_identity::PeerId;
use crate::validator_certificate::ValidatorCertificate;
use crate::trust_anchor::TrustAnchorRegistry;

/// Registry of active validators with verified certificates.
#[derive(Debug, Clone, Default)]
pub struct ValidatorRegistry {
    validators: BTreeMap<PeerId, ValidatorCertificate>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a validator after verifying their certificate against trust anchors.
    pub fn register(
        &mut self,
        certificate: ValidatorCertificate,
        trust_anchors: &TrustAnchorRegistry,
    ) -> Result<(), &'static str> {
        // Verify the certificate was issued by a trusted authority
        let authority_key = trust_anchors
            .get_key(&certificate.issuer)
            .ok_or("Issuer is not a trusted authority")?;

        if !certificate.verify(authority_key) {
            return Err("Certificate signature invalid");
        }

        self.validators.insert(certificate.validator_id, certificate);
        Ok(())
    }

    /// Remove a validator.
    pub fn remove(&mut self, validator_id: &PeerId) -> bool {
        self.validators.remove(validator_id).is_some()
    }

    /// Check if a peer is a registered validator.
    pub fn is_validator(&self, peer_id: &PeerId) -> bool {
        self.validators.contains_key(peer_id)
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Number of registered validators.
    pub fn len(&self) -> usize {
        self.validators.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_identity::PeerKeyPair;

    #[test]
    fn n21_validator_registration_with_valid_certificate() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let mut trust_anchors = TrustAnchorRegistry::new();
        trust_anchors.register(authority.peer_id(), authority.verifying_key.to_bytes());

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            0,
        );

        let mut registry = ValidatorRegistry::new();
        assert!(registry.register(cert, &trust_anchors).is_ok());
        assert!(registry.is_validator(&validator.peer_id()));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn n21_validator_rejected_without_trust_anchor() {
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let trust_anchors = TrustAnchorRegistry::new(); // empty

        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            1000,
            0,
        );

        let mut registry = ValidatorRegistry::new();
        assert!(registry.register(cert, &trust_anchors).is_err());
    }

    #[test]
    fn n21_validator_rejected_with_forged_certificate() {
        let authority = PeerKeyPair::generate();
        let impostor = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();

        let mut trust_anchors = TrustAnchorRegistry::new();
        trust_anchors.register(authority.peer_id(), authority.verifying_key.to_bytes());

        // Impostor tries to issue a certificate pretending to be the authority
        let forged_cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &impostor, // wrong key
            1000,
            0,
        );

        let mut registry = ValidatorRegistry::new();
        assert!(registry.register(forged_cert, &trust_anchors).is_err());
    }
}
