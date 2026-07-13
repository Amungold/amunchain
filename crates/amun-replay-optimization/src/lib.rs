use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub struct CachedCertificate {
    pub cert_hash: [u8; 32],
    pub height: u64,
    pub block_hash: [u8; 32],
    pub verified: bool,
}

pub struct CachedHeader {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
}

pub struct ReplayCache {
    pub certificates: BTreeMap<[u8; 32], CachedCertificate>,
    pub headers: BTreeMap<u64, CachedHeader>,
    pub hits: u64,
    pub misses: u64,
}

impl ReplayCache {
    pub fn new() -> Self {
        Self {
            certificates: BTreeMap::new(),
            headers: BTreeMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn store_certificate(&mut self, cert: CachedCertificate) {
        self.certificates.insert(cert.cert_hash, cert);
    }

    pub fn check_certificate(&mut self, cert_hash: &[u8; 32]) -> bool {
        if let Some(cert) = self.certificates.get(cert_hash) {
            self.hits += 1;
            cert.verified
        } else {
            self.misses += 1;
            false
        }
    }

    pub fn store_header(&mut self, header: CachedHeader) {
        self.headers.insert(header.height, header);
    }

    pub fn get_header(&mut self, height: u64) -> Option<&CachedHeader> {
        if self.headers.contains_key(&height) {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        self.headers.get(&height)
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn compute_cache_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_REPLAY_CACHE_V1");
        hasher.update(self.hits.to_le_bytes());
        hasher.update(self.misses.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn batch_verify_certificates(
        &mut self,
        cert_hashes: &[[u8; 32]],
        expected_valid: bool,
    ) -> u64 {
        let mut valid_count = 0u64;
        for cert_hash in cert_hashes {
            if self.check_certificate(cert_hash) == expected_valid {
                valid_count += 1;
            }
        }
        valid_count
    }
}

impl Default for ReplayCache {
    fn default() -> Self {
        Self::new()
    }
}
