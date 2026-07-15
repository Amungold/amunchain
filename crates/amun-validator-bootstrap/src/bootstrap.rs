use amun_validator_api::error::{
    EnrollmentError, EnrollmentErrorCode, PlatformError, PlatformResult,
};
use amun_validator_api::types::state::RuntimeState;
use amun_validator_identity::IdentityProvider;
use amun_validator_runtime::ValidatorRuntime;
use std::sync::Arc;

/// BootstrapService coordinates identity verification and runtime startup.
/// It owns NO identity logic, NO state transition logic, and NO key material.
/// All decisions are delegated to the injected services via their traits.
pub struct BootstrapService {
    identity: Arc<dyn IdentityProvider>,
    runtime: Arc<ValidatorRuntime>,
}

impl BootstrapService {
    /// Create a new BootstrapService with the given identity provider and runtime.
    pub fn new(identity: Arc<dyn IdentityProvider>, runtime: Arc<ValidatorRuntime>) -> Self {
        BootstrapService { identity, runtime }
    }

    /// Execute the full bootstrap sequence:
    /// 1. Delegate identity validation to the identity provider.
    /// 2. Delegate state transition to the runtime.
    /// 3. Verify the runtime is healthy after startup.
    pub fn bootstrap(&self) -> PlatformResult<()> {
        self.identity.self_check()?;
        self.runtime.start()?;

        let state = self.runtime.state();
        if state != RuntimeState::Provisioning {
            return Err(PlatformError::Enrollment(EnrollmentError::new(
                EnrollmentErrorCode::PhaseFailed,
                format!("Expected Provisioning after start, got {:?}", state),
            )));
        }

        if !self.runtime.is_healthy() {
            return Err(PlatformError::Enrollment(EnrollmentError::new(
                EnrollmentErrorCode::ChecklistNotMet,
                "Runtime not healthy after bootstrap".into(),
            )));
        }

        Ok(())
    }

    pub fn identity(&self) -> &Arc<dyn IdentityProvider> {
        &self.identity
    }
    pub fn runtime(&self) -> &Arc<ValidatorRuntime> {
        &self.runtime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_validator_api::types::id::PublicKey;
    use amun_validator_api::types::id::ValidatorId;
    use amun_validator_identity::authority_store::AuthorityStore;
    use amun_validator_identity::certificate_store::CertificateStore;
    use amun_validator_identity::key_store::KeyStore;
    use amun_validator_identity::IdentityService;
    use std::sync::Arc;

    fn create_test_identity() -> Arc<dyn IdentityProvider> {
        let keys = Arc::new(KeyStore::generate());
        let cert_data = CertificateStore::load_from_file("dummy.crt").unwrap();
        let cert = Arc::new(CertificateStore::new(cert_data));
        let auth = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        Arc::new(IdentityService::new(cert, keys, auth))
    }

    #[test]
    fn test_bootstrap_success() {
        let identity = create_test_identity();
        let rt = Arc::new(ValidatorRuntime::new(ValidatorId([1u8; 32])).unwrap());
        let bs = BootstrapService::new(identity, rt.clone());
        assert!(bs.bootstrap().is_ok());
        assert!(rt.is_healthy());
    }

    #[test]
    fn test_bootstrap_double_start_fails() {
        let identity = create_test_identity();
        let rt = Arc::new(ValidatorRuntime::new(ValidatorId([1u8; 32])).unwrap());
        let bs = BootstrapService::new(identity, rt.clone());
        bs.bootstrap().unwrap();
        assert!(bs.bootstrap().is_err());
    }

    #[test]
    fn test_bootstrap_identity_delegation() {
        let identity = create_test_identity();
        let rt = Arc::new(ValidatorRuntime::new(ValidatorId([1u8; 32])).unwrap());
        let bs = BootstrapService::new(identity, rt.clone());
        bs.bootstrap().unwrap();
        assert_eq!(bs.identity().validator_id().as_bytes(), &[1u8; 32]);
    }
}
