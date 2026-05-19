use std::fs;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

pub struct CertificateManager;

impl CertificateManager {
    pub fn load_certificates(path: &str) -> std::io::Result<Vec<CertificateDer<'static>>> {
        let content = fs::read(path)?;
        let mut reader = std::io::BufReader::new(&content[..]);
        rustls_pemfile::certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("cert: {}", e)))
    }

    pub fn load_private_key(path: &str) -> std::io::Result<PrivateKeyDer<'static>> {
        let content = fs::read(path)?;
        let mut reader = std::io::BufReader::new(&content[..]);
        
        // Try PKCS8 first
        let pkcs8_keys: Vec<PrivatePkcs8KeyDer> = rustls_pemfile::pkcs8_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("key: {}", e)))?;
        
        if let Some(key) = pkcs8_keys.into_iter().next() {
            return Ok(PrivateKeyDer::from(key));
        }
        
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no private key found"))
    }
}
