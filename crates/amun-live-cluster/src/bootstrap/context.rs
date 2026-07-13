use std::sync::Arc;
use std::time::Duration;

use amun_authority_registry::AuthorityRegistry;
use amun_consensus_network::engine::ConsensusEngine;

use crate::config::ValidatorConfig;

use super::{
    certificate::CertificateContext,
    cluster::{load_cluster_certificates, wait_for_cluster_certificates},
    identity::IdentityContext,
    registry::build_registry,
};

pub struct BootstrapContext {
    pub config: ValidatorConfig,
    pub identity: IdentityContext,
    pub certificate: CertificateContext,
    pub authority_registry: Arc<AuthorityRegistry>,
}

impl BootstrapContext {
    pub fn attach(&self, engine: &mut ConsensusEngine) -> Result<(), Box<dyn std::error::Error>> {
        // لا نفشل الإقلاع إذا لم تكن شهادات بقية العقد جاهزة بعد.
        let _ = wait_for_cluster_certificates(&self.config, Duration::from_secs(60));

        let certs = load_cluster_certificates(&self.config)?;

        build_registry(engine, &self.authority_registry, &certs)?;

        Ok(())
    }
}

impl BootstrapContext {
    pub fn prepare_local(
        &self,
        engine: &mut ConsensusEngine,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.attach(engine)
    }
}
