use crate::cert::CertificateManager;
use crate::pinning::CertificatePinning;
use rustls::server::WebPkiClientVerifier;
use rustls::ServerConfig;
use std::sync::Arc;

pub struct TlsServerConfig {
    pub config: Arc<ServerConfig>,
    pub require_client_auth: bool,
}

impl TlsServerConfig {
    pub fn new(
        cert_path: &str,
        key_path: &str,
        require_client_auth: bool,
        _pinning: Option<&CertificatePinning>,
    ) -> std::io::Result<Self> {
        let certs = CertificateManager::load_certificates(cert_path)?;
        let key = CertificateManager::load_private_key(key_path)?;

        let config = if require_client_auth {
            let root_store = rustls::RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            };

            let verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
                .build()
                .map_err(|e| std::io::Error::other(format!("TLS verifier: {}", e)))?;

            ServerConfig::builder()
                .with_client_cert_verifier(verifier)
                .with_single_cert(certs, key)
                .map_err(|e| std::io::Error::other(format!("TLS: {}", e)))?
        } else {
            ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(certs, key)
                .map_err(|e| std::io::Error::other(format!("TLS: {}", e)))?
        };

        Ok(Self {
            config: Arc::new(config),
            require_client_auth,
        })
    }
}
