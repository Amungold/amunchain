use amun_constitutional_state::ReplayCertificate;
use std::collections::BTreeMap;

/// Stores ReplayCertificates keyed by their certificate_hash.
/// Enables any node or light client to retrieve and verify state provenance.
pub struct ReplayStore {
    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
}

impl ReplayStore {
    pub fn new() -> Self {
        Self { certificates: BTreeMap::new() }
    }

    /// Insert a certificate. Key is certificate.certificate_hash().
    pub fn insert(&mut self, cert: ReplayCertificate) {
        let hash = cert.certificate_hash();
        self.certificates.insert(hash, cert);
    }

    /// Look up a certificate by its hash.
    pub fn get(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
        self.certificates.get(hash)
    }

    /// Check if a certificate with the given hash exists.
    pub fn contains(&self, hash: &[u8; 32]) -> bool {
        self.certificates.contains_key(hash)
    }

    /// Number of stored certificates.
    pub fn len(&self) -> usize {
        self.certificates.len()
    }

    /// Check if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.certificates.is_empty()
    }
}

impl Default for ReplayStore {
    fn default() -> Self {
        Self::new()
    }
}

/// A generic interface for retrieving ReplayCertificates by hash.
pub trait CertificateProvider {
    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate>;
}

impl CertificateProvider for ReplayStore {
    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate> {
        self.certificates.get(hash).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    #[test]
    fn test_store_and_retrieve() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let hash = cert.certificate_hash();

        let mut store = ReplayStore::new();
        store.insert(cert);

        assert!(store.contains(&hash));
        assert_eq!(store.get(&hash).unwrap().certificate_hash(), hash);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_missing_certificate() {
        let store = ReplayStore::new();
        assert!(!store.contains(&[0u8; 32]));
        assert!(store.get(&[0u8; 32]).is_none());
    }

    #[test]
    fn test_unique_hashes() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert1 = rt.create_certificate(1, [0u8; 32]);

        let mut rt2 = ConstitutionalStateRuntime::new();
        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let cert2 = rt2.create_certificate(1, [0u8; 32]);

        let mut store = ReplayStore::new();
        store.insert(cert1);
        store.insert(cert2);
        assert_eq!(store.len(), 2);
    }
}

