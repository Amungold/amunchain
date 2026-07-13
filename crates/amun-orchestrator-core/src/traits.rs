use async_trait::async_trait;
use std::path::Path;
use std::path::PathBuf;

use crate::error::OrchestratorError;
use crate::types::{PublicKey, ValidatorId};

#[async_trait]
pub trait CertificateProvider: Send + Sync {
    async fn generate_validator_certificate(
        &self,
        validator_name: &str,
        public_key: &PublicKey,
    ) -> Result<PathBuf, OrchestratorError>;
    async fn verify_certificate(&self, path: &Path) -> Result<bool, OrchestratorError>;
}

#[async_trait]
pub trait GenesisProvider: Send + Sync {
    async fn generate_genesis(
        &self,
        chain_id: &str,
        validators: &[(ValidatorId, PublicKey, u64)],
    ) -> Result<PathBuf, OrchestratorError>;
    async fn validate_genesis(&self, path: &Path) -> Result<bool, OrchestratorError>;
}

#[async_trait]
pub trait NetworkAdapter: Send + Sync {
    async fn check_quorum(&self) -> Result<bool, OrchestratorError>;
    async fn peer_count(&self) -> Result<usize, OrchestratorError>;
    async fn connect_to_bootnode(&self, addr: &str) -> Result<(), OrchestratorError>;
}

#[async_trait]
pub trait ProcessManager: Send + Sync {
    async fn start(&self, service_name: &str, args: &[String]) -> Result<u32, OrchestratorError>;
    async fn stop(&self, service_name: &str) -> Result<(), OrchestratorError>;
    async fn restart(&self, service_name: &str) -> Result<u32, OrchestratorError>;
    async fn is_running(&self, service_name: &str) -> Result<bool, OrchestratorError>;
    async fn pid(&self, service_name: &str) -> Result<Option<u32>, OrchestratorError>;
}
