use crate::authority_store::AuthorityStore;
use crate::certificate_store::CertificateStore;
use crate::identity_service::IdentityService;
use crate::key_store::KeyStore;
use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::PublicKey;
use std::sync::Arc;

/// Factory for creating IdentityService from files.
/// Separates construction logic from the service itself.
pub struct IdentityLoader;

impl IdentityLoader {
    /// Load identity from certificate file, key file, and authority.
    pub fn load(
        cert_path: &str,
        key_path: &str,
        authority_key: PublicKey,
    ) -> PlatformResult<IdentityService> {
        let cert_data = CertificateStore::load_from_file(cert_path)?;
        let keys = KeyStore::load_from_file(key_path)?;
        let authority_id = cert_data.authority_id;
        let authority_version = cert_data.authority_version;

        let cert = Arc::new(CertificateStore::new(cert_data));
        let keys = Arc::new(keys);
        let auth = Arc::new(AuthorityStore::from_single(
            authority_id,
            authority_key,
            authority_version,
        ));

        Ok(IdentityService::new(cert, keys, auth))
    }
}
