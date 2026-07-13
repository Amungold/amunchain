use crate::handshake::HandshakeError;
use crate::validator_certificate::ValidatorCertificate;

#[derive(Debug, Clone)]
pub struct VerificationContext {
    pub current_height: u64,
    pub constitution_hash: [u8; 32],
    pub network_id: [u8; 32],
}

/// Abstract certificate verification interface.
///
/// The networking layer must not depend directly on governance or
/// authority-registry implementations. Concrete verification is injected
/// by higher layers (e.g. amun-node).
pub trait CertificateVerifier: Send + Sync {
    fn verify(
        &self,
        certificate: &ValidatorCertificate,
        ctx: &VerificationContext,
    ) -> Result<(), HandshakeError>;
}

/// Default verifier used by tests and legacy callers.
/// Performs only cryptographic/local validation.
pub trait VerificationContextProvider: Send + Sync {
    fn current_context(&self) -> VerificationContext;
}

#[derive(Default)]
pub struct StaticVerificationContextProvider;

impl VerificationContextProvider for StaticVerificationContextProvider {
    fn current_context(&self) -> VerificationContext {
        VerificationContext {
            current_height: 0,
            constitution_hash: [0u8; 32],
            network_id: [0u8; 32],
        }
    }
}

#[derive(Default)]
pub struct LocalCertificateVerifier;

impl CertificateVerifier for LocalCertificateVerifier {
    fn verify(
        &self,
        certificate: &ValidatorCertificate,
        ctx: &VerificationContext,
    ) -> Result<(), HandshakeError> {
        if !certificate.is_valid_at(ctx.current_height) {
            return Err(HandshakeError::AuthenticationFailed);
        }

        Ok(())
    }
}
