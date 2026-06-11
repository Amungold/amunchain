use amun_certificate_network::distribution::LightClientProofBundle;
use amun_constitutional_block::ConstitutionalBlock;
use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================
// N11A: Header-First Sync Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderSyncMessage {
    RequestHeaders { start_height: u64, end_height: u64 },
    HeaderBatch { headers: Vec<ConstitutionalBlock> },
    HeaderNotFound { height: u64 },
}

// ============================================================
// N11B: Certificate-First Sync Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateSyncMessage {
    RequestCertificatesForHeight {
        height: u64,
    },
    CertificateBatch {
        certificates: Vec<ReplayCertificate>,
    },
}

// ============================================================
// N11C: Proof Bundle Sync Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum ProofBundleSyncMessage {
    RequestBundle { height: u64 },
    BundleResponse { bundle: LightClientProofBundle },
}

// ============================================================
// N11D: Stateless Node
// ============================================================

pub struct StatelessNode {
    headers: BTreeMap<u64, ConstitutionalBlock>,
    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
    inclusion_proofs: BTreeMap<[u8; 32], CertificateInclusionProof>,
    bundles: BTreeMap<u64, LightClientProofBundle>,
    tip_height: u64,
}

impl StatelessNode {
    pub fn new() -> Self {
        Self {
            headers: BTreeMap::new(),
            certificates: BTreeMap::new(),
            inclusion_proofs: BTreeMap::new(),
            bundles: BTreeMap::new(),
            tip_height: 0,
        }
    }

    pub fn import_header(&mut self, header: ConstitutionalBlock) {
        let h = header.block_height;
        self.headers.insert(h, header);
        if h > self.tip_height {
            self.tip_height = h;
        }
    }

    pub fn import_certificate(&mut self, cert: ReplayCertificate) {
        let hash = cert.certificate_hash();
        self.certificates.insert(hash, cert);
    }

    pub fn import_inclusion_proof(&mut self, hash: [u8; 32], proof: CertificateInclusionProof) {
        self.inclusion_proofs.insert(hash, proof);
    }

    pub fn import_bundle(&mut self, height: u64, bundle: LightClientProofBundle) {
        self.bundles.insert(height, bundle);
    }

    pub fn verify_height(&self, height: u64) -> Result<(), String> {
        let header = self
            .headers
            .get(&height)
            .ok_or_else(|| format!("Missing header at height {}", height))?;
        let bundle = self
            .bundles
            .get(&height)
            .ok_or_else(|| format!("Missing bundle at height {}", height))?;
        let cert_hash = bundle.certificate.certificate_hash();
        let proof = self
            .inclusion_proofs
            .get(&cert_hash)
            .ok_or("Missing inclusion proof for certificate")?;
        amun_constitutional_block::verify_light_client_proof(header, &bundle.certificate, proof)
    }

    pub fn verify_chain(&self) -> Result<(), String> {
        for h in 0..=self.tip_height {
            self.verify_height(h)?;
        }
        Ok(())
    }

    pub fn has_header(&self, height: u64) -> bool {
        self.headers.contains_key(&height)
    }
    pub fn tip_height(&self) -> u64 {
        self.tip_height
    }
    pub fn header_count(&self) -> usize {
        self.headers.len()
    }
}

impl Default for StatelessNode {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// N11E: Trustless Bootstrap
// ============================================================

pub struct TrustlessBootstrap;

impl TrustlessBootstrap {
    pub fn bootstrap(bundles: Vec<LightClientProofBundle>) -> Result<StatelessNode, String> {
        let mut node = StatelessNode::new();
        for bundle in &bundles {
            let height = bundle.block_height();
            let cert_hash = bundle.certificate.certificate_hash();
            bundle.verify()?;
            node.import_header(bundle.block_header.clone());
            node.import_certificate(bundle.certificate.clone());
            node.import_inclusion_proof(cert_hash, bundle.inclusion_proof.clone());
            node.import_bundle(height, bundle.clone());
        }
        for h in 0..=node.tip_height() {
            if !node.has_header(h) {
                return Err(format!("Missing header at height {}", h));
            }
        }
        node.verify_chain()?;
        Ok(node)
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod n11_tests {
    use super::*;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    fn create_bundle(height: u64, parent_hash: &str) -> LightClientProofBundle {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );
        LightClientProofBundle::new(block, cert, proof)
    }

    #[test]
    fn n11a_header_sync() {
        let bundle = create_bundle(0, &"0".repeat(64));
        let msg = HeaderSyncMessage::HeaderBatch {
            headers: vec![bundle.block_header.clone()],
        };
        match msg {
            HeaderSyncMessage::HeaderBatch { headers } => assert_eq!(headers[0].block_height, 0),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n11b_certificate_sync() {
        let bundle = create_bundle(0, &"0".repeat(64));
        let msg = CertificateSyncMessage::CertificateBatch {
            certificates: vec![bundle.certificate.clone()],
        };
        match msg {
            CertificateSyncMessage::CertificateBatch { certificates } => {
                assert!(!certificates.is_empty())
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n11c_proof_bundle_sync() {
        let bundle = create_bundle(0, &"0".repeat(64));
        let msg = ProofBundleSyncMessage::BundleResponse {
            bundle: bundle.clone(),
        };
        match msg {
            ProofBundleSyncMessage::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n11d_stateless_node_import() {
        let bundle = create_bundle(0, &"0".repeat(64));
        let cert_hash = bundle.certificate.certificate_hash();
        let mut node = StatelessNode::new();
        node.import_header(bundle.block_header.clone());
        node.import_certificate(bundle.certificate.clone());
        node.import_inclusion_proof(cert_hash, bundle.inclusion_proof.clone());
        node.import_bundle(0, bundle);
        assert_eq!(node.tip_height(), 0);
        assert_eq!(node.header_count(), 1);
        assert!(node.verify_height(0).is_ok());
    }

    #[test]
    fn n11d_stateless_node_verify_chain() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);
        let mut node = StatelessNode::new();
        for (b, h) in [(b0, 0), (b1, 1)] {
            let cert_hash = b.certificate.certificate_hash();
            node.import_header(b.block_header.clone());
            node.import_certificate(b.certificate.clone());
            node.import_inclusion_proof(cert_hash, b.inclusion_proof.clone());
            node.import_bundle(h, b);
        }
        assert_eq!(node.tip_height(), 1);
        assert_eq!(node.header_count(), 2);
        assert!(node.verify_chain().is_ok());
    }

    #[test]
    fn n11e_trustless_bootstrap() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);
        let bundles = vec![b0, b1];
        let node = TrustlessBootstrap::bootstrap(bundles).unwrap();
        assert_eq!(node.tip_height(), 1);
        assert_eq!(node.header_count(), 2);
        assert!(node.verify_chain().is_ok());
    }

    #[test]
    fn n11e_tampered_bundle_rejected() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let mut bad = create_bundle(1, &b0.block_header.block_hash);
        bad.block_header.state_root = "tampered".into();
        let bundles = vec![b0, bad];
        assert!(TrustlessBootstrap::bootstrap(bundles).is_err());
    }

    #[test]
    fn n11_missing_header_detected() {
        let node = StatelessNode::new();
        assert!(node.verify_height(5).is_err());
    }
}
