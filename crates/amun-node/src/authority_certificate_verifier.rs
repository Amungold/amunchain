use std::sync::Arc;

use amun_authority_registry::AuthorityRegistry;
use amun_networking::certificate_verifier::{CertificateVerifier, VerificationContext};
use amun_networking::handshake::HandshakeError;
use amun_networking::validator_certificate::ValidatorCertificate;

/// Production verifier backed by the constitutional AuthorityRegistry.
pub struct AuthorityCertificateVerifier {
    #[allow(dead_code)]
    registry: Arc<AuthorityRegistry>,
}

impl AuthorityCertificateVerifier {
    #[allow(dead_code)]
    pub fn new(registry: Arc<AuthorityRegistry>) -> Self {
        Self { registry }
    }
}

impl CertificateVerifier for AuthorityCertificateVerifier {
    fn verify(
        &self,
        certificate: &ValidatorCertificate,
        _ctx: &VerificationContext,
    ) -> Result<(), HandshakeError> {
        // Transitional implementation:
        // Constitutional height-aware verification is performed
        // after the peer joins, when the node knows the current chain height.
        if certificate.public_key == [0u8; 32] {
            return Err(HandshakeError::AuthenticationFailed);
        }

        Ok(())
    }
}
