use sha2::{Sha256, Digest};
use std::collections::HashSet;

pub struct CertificatePinning {
    pub allowed_fingerprints: HashSet<[u8; 32]>,
}

impl CertificatePinning {
    pub fn new() -> Self {
        Self {
            allowed_fingerprints: HashSet::new(),
        }
    }

    pub fn allow_fingerprint(&mut self, fingerprint: &[u8; 32]) {
        self.allowed_fingerprints.insert(*fingerprint);
    }

    pub fn verify(&self, cert_der: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(cert_der);
        let mut fp = [0u8; 32];
        fp.copy_from_slice(&hasher.finalize());
        self.allowed_fingerprints.contains(&fp)
    }

    pub fn remove_fingerprint(&mut self, fingerprint: &[u8; 32]) {
        self.allowed_fingerprints.remove(fingerprint);
    }
}
