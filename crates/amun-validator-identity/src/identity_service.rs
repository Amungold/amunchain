use crate::authority_store::AuthorityStore;
use crate::certificate_store::CertificateStore;
use crate::key_store::KeyStore;
use crate::IdentityProvider;
use amun_validator_api::error::{IdentityError, IdentityErrorCode, PlatformError, PlatformResult};
use amun_validator_api::types::id::{PublicKey, ValidatorId};
use std::sync::Arc;

/// The single identity service consumed by Bootstrap, Consensus, Networking, RPC.
/// Delegates to stores; does NOT hold its own copies of identity data.
pub struct IdentityService {
    certificates: Arc<CertificateStore>,
    keys: Arc<KeyStore>,
    authorities: Arc<AuthorityStore>,
}

impl IdentityService {
    pub fn new(
        certificates: Arc<CertificateStore>,
        keys: Arc<KeyStore>,
        authorities: Arc<AuthorityStore>,
    ) -> Self {
        IdentityService {
            certificates,
            keys,
            authorities,
        }
    }

    pub fn certificates(&self) -> &Arc<CertificateStore> {
        &self.certificates
    }
    pub fn keys(&self) -> &Arc<KeyStore> {
        &self.keys
    }
    pub fn authorities(&self) -> &Arc<AuthorityStore> {
        &self.authorities
    }

    /// Verify that the certificate was issued by a trusted authority.
    pub fn verify_authority(&self) -> PlatformResult<()> {
        let cert = self.certificates.data();
        if !self.authorities.is_trusted(&cert.authority_id) {
            return Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::AuthorityUnknown,
                format!("Authority {:?} not trusted", &cert.authority_id[..4]),
            )));
        }
        if cert.signature.is_empty() {
            return Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::SignatureInvalid,
                "Certificate has no signature".into(),
            )));
        }
        Ok(())
    }
}

impl IdentityProvider for IdentityService {
    fn validator_id(&self) -> &ValidatorId {
        self.certificates.validator_id()
    }
    fn public_key(&self) -> &PublicKey {
        self.certificates.public_key()
    }
    fn certificate_hash(&self) -> &[u8; 32] {
        self.certificates.certificate_hash()
    }

    fn self_check(&self) -> PlatformResult<()> {
        if self.certificates.certificate_hash() == &[0u8; 32] {
            return Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::CertificateInvalid,
                "Certificate hash is zero".into(),
            )));
        }
        self.verify_authority()?;
        Ok(())
    }

    fn sign(&self, message: &[u8]) -> PlatformResult<Vec<u8>> {
        self.keys.sign(message)
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> PlatformResult<()> {
        if self.keys.public_key_bytes() == [0u8; 32] {
            return Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::KeyNotFound,
                "Public key not set".into(),
            )));
        }
        let recomputed = self.keys.sign(message)?;
        if recomputed == signature {
            Ok(())
        } else {
            Err(PlatformError::Identity(IdentityError::new(
                IdentityErrorCode::SignatureInvalid,
                "Signature mismatch".into(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_service_delegates_to_store() {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        let svc = IdentityService::new(cert.clone(), keys.clone(), auth);

        assert_eq!(
            svc.validator_id().as_bytes(),
            cert.validator_id().as_bytes()
        );
        assert_eq!(svc.public_key().as_bytes(), cert.public_key().as_bytes());
    }

    #[test]
    fn test_sign_and_verify() {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        let svc = IdentityService::new(cert, keys, auth);

        let msg = b"test message";
        let sig = svc.sign(msg).unwrap();
        assert!(svc.verify(msg, &sig).is_ok());
    }

    #[test]
    fn test_verify_tampered_fails() {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        let svc = IdentityService::new(cert, keys, auth);

        let sig = svc.sign(b"original").unwrap();
        assert!(svc.verify(b"tampered", &sig).is_err());
    }

    #[test]
    fn test_verify_authority_success() {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        let svc = IdentityService::new(cert, keys, auth);
        assert!(svc.verify_authority().is_ok());
    }

    #[test]
    fn test_verify_authority_untrusted() {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0xFFu8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        let svc = IdentityService::new(cert, keys, auth);
        assert!(svc.verify_authority().is_err());
    }
}
