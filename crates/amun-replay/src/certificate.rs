use crate::validation::ReplayResult;
use blake3::Hasher;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCertificate {
    pub certificate_id: [u8; 32],
    pub previous_certificate: [u8; 32],
    pub commit_height: u64,
    pub state_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub commits_checked: usize,
    pub validator_hash: [u8; 32],
    pub timestamp: u64,
}

impl ReplayCertificate {
    pub fn issue(
        result: &ReplayResult,
        commit_height: u64,
        previous_certificate: [u8; 32],
        validator_hash: [u8; 32],
        timestamp: u64,
    ) -> Option<Self> {
        if !result.valid {
            return None;
        }
        let mut cert = Self {
            certificate_id: [0u8; 32],
            previous_certificate,
            commit_height,
            state_root: result.expected_root,
            replay_root: [0u8; 32],
            commits_checked: result.commits_checked,
            validator_hash,
            timestamp,
        };
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_REPLAY_ROOT");
        hasher.update(&result.expected_root);
        hasher.update(&result.commits_checked.to_le_bytes());
        cert.replay_root = hasher.finalize().into();
        cert.certificate_id = cert.compute_id();
        Some(cert)
    }

    pub fn genesis(validator_hash: [u8; 32], timestamp: u64) -> Self {
        let mut cert = Self {
            certificate_id: [0u8; 32],
            previous_certificate: [0u8; 32],
            commit_height: 0,
            state_root: [0u8; 32],
            replay_root: [0u8; 32],
            commits_checked: 0,
            validator_hash,
            timestamp,
        };
        cert.certificate_id = cert.compute_id();
        cert
    }

    fn compute_id(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_REPLAY_CERTIFICATE_V2");
        hasher.update(&self.previous_certificate);
        hasher.update(&self.commit_height.to_le_bytes());
        hasher.update(&self.state_root);
        hasher.update(&self.replay_root);
        hasher.update(&self.commits_checked.to_le_bytes());
        hasher.update(&self.validator_hash);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.certificate_id == self.compute_id()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReplayCertificateStore {
    certificates: HashMap<[u8; 32], ReplayCertificate>,
    latest_id: Option<[u8; 32]>,
}

impl ReplayCertificateStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, cert: ReplayCertificate) -> Result<(), &'static str> {
        if !cert.verify() {
            return Err("Certificate verification failed");
        }
        if let Some(ref latest) = self.latest_id {
            if cert.previous_certificate != *latest {
                return Err("Certificate chain broken");
            }
        } else {
            if cert.previous_certificate != [0u8; 32] {
                return Err("First certificate must have zero previous");
            }
        }
        self.latest_id = Some(cert.certificate_id);
        self.certificates.insert(cert.certificate_id, cert);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.certificates.is_empty()
    }
    pub fn len(&self) -> usize {
        self.certificates.len()
    }
    pub fn latest(&self) -> Option<&ReplayCertificate> {
        self.latest_id
            .as_ref()
            .and_then(|id| self.certificates.get(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commit_log::StateCommit;
    use crate::validation::{ReplayResult, ReplayValidator};

    fn make_commit(h: u64, prev: [u8; 32], new: [u8; 32]) -> StateCommit {
        StateCommit {
            height: h,
            block_hash: [h as u8; 32],
            previous_root: prev,
            new_root: new,
            tx_count: 1,
            timestamp: h * 1000,
        }
    }

    fn valid_result() -> ReplayResult {
        ReplayValidator::validate(&[
            make_commit(1, [0u8; 32], [10u8; 32]),
            make_commit(2, [10u8; 32], [20u8; 32]),
            make_commit(3, [20u8; 32], [30u8; 32]),
        ])
    }

    #[test]
    fn n37_genesis_certificate() {
        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
        assert!(genesis.verify());
        assert_ne!(genesis.certificate_id, [0u8; 32]);
        assert_eq!(genesis.previous_certificate, [0u8; 32]);
    }

    #[test]
    fn n37_valid_chain() {
        let mut store = ReplayCertificateStore::new();
        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
        assert!(store.store(genesis).is_ok());
        let prev_id = store.latest().unwrap().certificate_id;
        let result = valid_result();
        let cert1 = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
        assert!(store.store(cert1).is_ok());
        let prev_id2 = store.latest().unwrap().certificate_id;
        let cert2 = ReplayCertificate::issue(&result, 6, prev_id2, [0xAA; 32], 3000).unwrap();
        assert!(store.store(cert2).is_ok());
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn n37_broken_chain_rejected() {
        let mut store = ReplayCertificateStore::new();
        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
        store.store(genesis).unwrap();
        let result = valid_result();
        let mut cert = ReplayCertificate::issue(&result, 3, [0xFF; 32], [0xAA; 32], 2000).unwrap();
        cert.previous_certificate = [0xDE; 32];
        assert!(store.store(cert).is_err());
    }

    #[test]
    fn n37_tampered_replay_root_rejected() {
        let mut store = ReplayCertificateStore::new();
        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
        store.store(genesis).unwrap();
        let prev_id = store.latest().unwrap().certificate_id;
        let result = valid_result();
        let mut cert = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
        cert.replay_root = [0xFF; 32];
        assert!(store.store(cert).is_err());
    }

    #[test]
    fn n37_certificate_chain_verification() {
        let mut store = ReplayCertificateStore::new();
        store
            .store(ReplayCertificate::genesis([0xAA; 32], 1000))
            .unwrap();
        let r = valid_result();
        let id1 = store.latest().unwrap().certificate_id;
        store
            .store(ReplayCertificate::issue(&r, 3, id1, [0xAA; 32], 2000).unwrap())
            .unwrap();
        let id2 = store.latest().unwrap().certificate_id;
        store
            .store(ReplayCertificate::issue(&r, 6, id2, [0xAA; 32], 3000).unwrap())
            .unwrap();
        assert_eq!(store.len(), 3);
        for _ in 0..3 {
            let c = store.latest().unwrap();
            assert!(c.verify());
        }
    }
}
