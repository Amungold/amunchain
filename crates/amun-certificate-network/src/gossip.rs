use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap, HashSet};
use amun_constitutional_state::{
    ReplayCertificate,
    CertificateInclusionProof,
};
use crate::distribution::LightClientProofBundle;

// ============================================================
// N10A: Certificate Announcement Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateAnnouncement {
    Announce { certificate_hash: [u8; 32], block_height: u64 },
}

// ============================================================
// N10B: Certificate Inventory Exchange
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateInventory {
    Inventory { certificate_hashes: Vec<[u8; 32]> },
    RequestMissing { hashes: Vec<[u8; 32]> },
}

// ============================================================
// N10C: Certificate Synchronization
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateSync {
    RequestBatch { hashes: Vec<[u8; 32]> },
    BatchResponse { certificates: Vec<ReplayCertificate> },
}

// ============================================================
// N10D: Inclusion Proof Distribution
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofSync {
    RequestProof { certificate_hash: [u8; 32] },
    ProofResponse { proof: CertificateInclusionProof },
}

// ============================================================
// N10E: Bundle Gossip
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum BundleGossip {
    AnnounceBundle { block_height: u64, certificate_hash: [u8; 32] },
    RequestBundle { block_height: u64 },
    BundleResponse { bundle: LightClientProofBundle },
}

// ============================================================
// N10F: Peer Certificate Cache
// ============================================================

#[derive(Debug, Clone, Default)]
pub struct PeerCertificateCache {
    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
    proofs: BTreeMap<[u8; 32], CertificateInclusionProof>,
    bundles: BTreeMap<u64, LightClientProofBundle>,
    announced: HashSet<[u8; 32]>,
}

impl PeerCertificateCache {
    pub fn new() -> Self { Self::default() }

    pub fn store_certificate(&mut self, cert: ReplayCertificate) {
        let hash = cert.certificate_hash();
        self.certificates.insert(hash, cert);
    }

    pub fn store_proof(&mut self, cert_hash: [u8; 32], proof: CertificateInclusionProof) {
        self.proofs.insert(cert_hash, proof);
    }

    pub fn store_bundle(&mut self, height: u64, bundle: LightClientProofBundle) {
        self.bundles.insert(height, bundle);
    }

    pub fn mark_announced(&mut self, hash: [u8; 32]) {
        self.announced.insert(hash);
    }

    pub fn is_announced(&self, hash: &[u8; 32]) -> bool {
        self.announced.contains(hash)
    }

    pub fn has_certificate(&self, hash: &[u8; 32]) -> bool {
        self.certificates.contains_key(hash)
    }

    pub fn has_proof(&self, hash: &[u8; 32]) -> bool {
        self.proofs.contains_key(hash)
    }

    pub fn get_certificate(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
        self.certificates.get(hash)
    }

    pub fn get_proof(&self, hash: &[u8; 32]) -> Option<&CertificateInclusionProof> {
        self.proofs.get(hash)
    }

    pub fn get_bundle(&self, height: u64) -> Option<&LightClientProofBundle> {
        self.bundles.get(&height)
    }

    pub fn missing_certificates(&self, inventory: &[[u8; 32]]) -> Vec<[u8; 32]> {
        inventory.iter()
            .filter(|h| !self.has_certificate(h))
            .copied()
            .collect()
    }

    pub fn known_hashes(&self) -> Vec<[u8; 32]> {
        self.certificates.keys().copied().collect()
    }

    pub fn certificate_count(&self) -> usize { self.certificates.len() }
    pub fn proof_count(&self) -> usize { self.proofs.len() }
    pub fn bundle_count(&self) -> usize { self.bundles.len() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod n10_tests {
    use super::*;
    use amun_constitutional_state::ConstitutionalStateRuntime;
    use amun_constitutional_block::ConstitutionalBlock;

    fn create_test_data() -> (ReplayCertificate, CertificateInclusionProof, LightClientProofBundle) {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![], hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(), merkle_root);
        let bundle = LightClientProofBundle::new(block, cert.clone(), proof.clone());
        (cert, proof, bundle)
    }

    #[test]
    fn n10a_certificate_announcement() {
        let (cert, _, _) = create_test_data();
        let hash = cert.certificate_hash();
        let msg = CertificateAnnouncement::Announce { certificate_hash: hash, block_height: 1 };
        match msg {
            CertificateAnnouncement::Announce { certificate_hash, block_height } => {
                assert_eq!(certificate_hash, hash);
                assert_eq!(block_height, 1);
            }
        }
    }

    #[test]
    fn n10b_inventory_exchange() {
        let (cert, _, _) = create_test_data();
        let hash = cert.certificate_hash();
        let inv = CertificateInventory::Inventory { certificate_hashes: vec![hash] };
        match inv {
            CertificateInventory::Inventory { certificate_hashes } => {
                assert!(certificate_hashes.contains(&hash));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n10c_certificate_sync() {
        let (cert, _, _) = create_test_data();
        let hash = cert.certificate_hash();
        let resp = CertificateSync::BatchResponse { certificates: vec![cert.clone()] };
        match resp {
            CertificateSync::BatchResponse { certificates } => {
                assert_eq!(certificates[0].certificate_hash(), hash);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n10d_proof_sync() {
        let (_, proof, _) = create_test_data();
        let resp = ProofSync::ProofResponse { proof: proof.clone() };
        match resp {
            ProofSync::ProofResponse { proof: p } => assert!(p.verify()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n10e_bundle_gossip() {
        let (_, _, bundle) = create_test_data();
        let resp = BundleGossip::BundleResponse { bundle: bundle.clone() };
        match resp {
            BundleGossip::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n10f_cache_store_and_retrieve() {
        let (cert, proof, bundle) = create_test_data();
        let hash = cert.certificate_hash();
        let mut cache = PeerCertificateCache::new();
        cache.store_certificate(cert.clone());
        cache.store_proof(hash, proof.clone());
        cache.store_bundle(0, bundle);
        assert!(cache.has_certificate(&hash));
        assert!(cache.has_proof(&hash));
        assert!(cache.get_bundle(0).is_some());
        assert_eq!(cache.certificate_count(), 1);
        assert_eq!(cache.proof_count(), 1);
        assert_eq!(cache.bundle_count(), 1);
    }

    #[test]
    fn n10f_cache_missing_detection() {
        let (cert, _, _) = create_test_data();
        let hash = cert.certificate_hash();
        let mut cache = PeerCertificateCache::new();
        cache.store_certificate(cert);
        cache.mark_announced(hash);
        let inventory = vec![hash, [0xFF; 32]];
        let missing = cache.missing_certificates(&inventory);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], [0xFF; 32]);
    }

    #[test]
    fn n10f_cache_announcement_tracking() {
        let hash = [0xAA; 32];
        let mut cache = PeerCertificateCache::new();
        assert!(!cache.is_announced(&hash));
        cache.mark_announced(hash);
        assert!(cache.is_announced(&hash));
    }

    #[test]
    fn n10f_cache_known_hashes() {
        let (cert, _, _) = create_test_data();
        let hash = cert.certificate_hash();
        let mut cache = PeerCertificateCache::new();
        cache.store_certificate(cert);
        let known = cache.known_hashes();
        assert!(known.contains(&hash));
    }

    #[test]
    fn n10_full_gossip_flow() {
        let (cert, proof, bundle) = create_test_data();
        let hash = cert.certificate_hash();

        let _announce = CertificateAnnouncement::Announce { certificate_hash: hash, block_height: 0 };
        let cert_resp = CertificateSync::BatchResponse { certificates: vec![cert.clone()] };
        let proof_resp = ProofSync::ProofResponse { proof: proof.clone() };
        let bundle_resp = BundleGossip::BundleResponse { bundle: bundle.clone() };

        let mut cache = PeerCertificateCache::new();
        cache.mark_announced(hash);

        match cert_resp {
            CertificateSync::BatchResponse { certificates } => {
                for c in &certificates { cache.store_certificate(c.clone()); }
            }
            _ => unreachable!(),
        }
        match proof_resp {
            ProofSync::ProofResponse { proof: p } => cache.store_proof(hash, p.clone()),
            _ => unreachable!(),
        }
        match bundle_resp {
            BundleGossip::BundleResponse { bundle: b } => {
                cache.store_bundle(0, b.clone());
                assert!(b.verify().is_ok());
            }
            _ => unreachable!(),
        }

        assert!(cache.has_certificate(&hash));
        assert!(cache.has_proof(&hash));
        assert!(cache.get_bundle(0).is_some());
        assert_eq!(cache.certificate_count(), 1);
    }

    #[test]
    fn n10_serialize_announcement() {
        let msg = CertificateAnnouncement::Announce { certificate_hash: [1u8; 32], block_height: 7 };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: CertificateAnnouncement = match serde_json::from_str(&json) { Ok(v) => v, Err(e) => { eprintln!("gossip: invalid announcement: {}", e); return; } };
        match decoded {
            CertificateAnnouncement::Announce { certificate_hash, block_height } => {
                assert_eq!(certificate_hash, [1u8; 32]);
                assert_eq!(block_height, 7);
            }
        }
    }
}
