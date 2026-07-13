pub mod store;

use amun_networking::validator_certificate::ValidatorCertificate;
use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::event::EventBus;
use amun_orchestrator_core::traits::CertificateProvider;
use amun_orchestrator_core::types::PublicKey;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CertificateAuthority {
    base_dir: PathBuf,
    event_bus: Arc<EventBus>,
    certs: tokio::sync::RwLock<Vec<ValidatorCertificate>>,
}

impl CertificateAuthority {
    pub fn new(base_dir: PathBuf, event_bus: Arc<EventBus>) -> Self {
        Self {
            base_dir,
            event_bus,
            certs: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    pub fn certs_dir(&self) -> PathBuf {
        self.base_dir.join("certificates")
    }

    pub fn cert_path(&self, validator_name: &str) -> PathBuf {
        self.certs_dir().join(format!("{}.crt", validator_name))
    }

    pub async fn issue_certificate(
        &self,
        validator_name: &str,
        public_key: &PublicKey,
    ) -> Result<ValidatorCertificate, OrchestratorError> {
        if let Some(existing) = self.find_cert(validator_name).await {
            tracing::info!(%validator_name, "Certificate already exists");
            return Ok(existing);
        }

        // إنشاء شهادة متوافقة مع amun-networking
        let cert = ValidatorCertificate {
            validator_id: amun_networking::peer_identity::PeerId(public_key.0),
            public_key: public_key.0,
            issuer: amun_networking::peer_identity::PeerId(public_key.0),
            authority_id: [0u8; 32],
            authority_version: 0,
            valid_from: 0,
            valid_until: 0,
            authority_signature: vec![],
        };

        let path = self.cert_path(validator_name);
        store::save_certificate(&cert, &path).await?;
        self.certs.write().await.push(cert.clone());

        self.event_bus.emit(
            "certificate-authority",
            amun_orchestrator_core::event::OrchestratorEvent::CertificateGenerated {
                validator: validator_name.to_string(),
                path: path.to_string_lossy().to_string(),
            },
        );

        tracing::info!(%validator_name, "Certificate issued");
        Ok(cert)
    }

    async fn find_cert(&self, name: &str) -> Option<ValidatorCertificate> {
        self.certs
            .read()
            .await
            .iter()
            .find(|c| hex::encode(c.validator_id.0) == name || hex::encode(c.public_key) == name)
            .cloned()
    }
}

#[async_trait]
impl CertificateProvider for CertificateAuthority {
    async fn generate_validator_certificate(
        &self,
        validator_name: &str,
        public_key: &PublicKey,
    ) -> Result<PathBuf, OrchestratorError> {
        self.issue_certificate(validator_name, public_key).await?;
        Ok(self.cert_path(validator_name))
    }

    async fn verify_certificate(&self, path: &std::path::Path) -> Result<bool, OrchestratorError> {
        if !path.exists() {
            return Ok(false);
        }
        let _ = store::load_certificate(path).await?;
        Ok(true)
    }
}
