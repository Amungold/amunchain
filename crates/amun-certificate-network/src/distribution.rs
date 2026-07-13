// ============================================================
// Phase N9: Certificate Distribution Layer
// ============================================================

use amun_constitutional_block::ConstitutionalBlock;
use amun_constitutional_state::{
    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
};
use serde::{Deserialize, Serialize};

// ============================================================
// N9A: Certificate Request/Response Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateMessage {
    RequestCertificate { certificate_hash: [u8; 32] },
    CertificateResponse { certificate: ReplayCertificate },
}

impl CertificateMessage {
    pub fn certificate_hash(&self) -> Option<[u8; 32]> {
        match self {
            CertificateMessage::RequestCertificate { certificate_hash } => Some(*certificate_hash),
            CertificateMessage::CertificateResponse { certificate } => {
                Some(certificate.certificate_hash())
            }
        }
    }
}

// ============================================================
// N9B: Inclusion Proof Request/Response Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InclusionProofMessage {
    RequestInclusionProof { certificate_hash: [u8; 32] },
    InclusionProofResponse { proof: CertificateInclusionProof },
}

// ============================================================
// N9C: Light Client Proof Bundle
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientProofBundle {
    pub block_header: ConstitutionalBlock,
    pub certificate: ReplayCertificate,
    pub inclusion_proof: CertificateInclusionProof,
}

impl LightClientProofBundle {
    pub fn new(
        block_header: ConstitutionalBlock,
        certificate: ReplayCertificate,
        inclusion_proof: CertificateInclusionProof,
    ) -> Self {
        Self {
            block_header,
            certificate,
            inclusion_proof,
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        amun_constitutional_block::verify_light_client_proof(
            &self.block_header,
            &self.certificate,
            &self.inclusion_proof,
        )
    }

    pub fn block_height(&self) -> u64 {
        self.block_header.block_height
    }
    pub fn certificate_hash(&self) -> [u8; 32] {
        self.certificate.certificate_hash()
    }
}

// ============================================================
// N9D: Bundle Distribution Protocol
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum ProofBundleMessage {
    RequestBundle { block_height: u64 },
    RequestBundleByHash { certificate_hash: [u8; 32] },
    BundleResponse { bundle: LightClientProofBundle },
    BundleNotFound { reason: String },
}

// ============================================================
// N9E: Bundle Builder
// ============================================================

pub struct BundleBuilder;

impl BundleBuilder {
    pub fn build_from_runtime(
        block: ConstitutionalBlock,
        rt: &ConstitutionalStateRuntime,
        pre_state_root: [u8; 32],
    ) -> Option<LightClientProofBundle> {
        let cert = rt.create_certificate(block.block_height, pre_state_root);
        let certs = vec![cert.clone()];
        let hash = cert.certificate_hash();
        let inclusion_proof =
            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash)?;
        Some(LightClientProofBundle {
            block_header: block,
            certificate: cert,
            inclusion_proof,
        })
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod n9_tests {
    use super::*;

    fn create_test_bundle() -> LightClientProofBundle {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let inclusion_proof =
            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(
            0,
            "0".repeat(64),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );
        LightClientProofBundle::new(block, cert, inclusion_proof)
    }

    #[test]
    fn n9a_certificate_request_response() {
        let bundle = create_test_bundle();
        let cert_hash = bundle.certificate_hash();
        let response = CertificateMessage::CertificateResponse {
            certificate: bundle.certificate.clone(),
        };
        match response {
            CertificateMessage::CertificateResponse { certificate } => {
                assert_eq!(certificate.certificate_hash(), cert_hash)
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n9b_inclusion_proof_request_response() {
        let bundle = create_test_bundle();
        let response = InclusionProofMessage::InclusionProofResponse {
            proof: bundle.inclusion_proof.clone(),
        };
        match response {
            InclusionProofMessage::InclusionProofResponse { proof } => assert!(proof.verify()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n9c_light_client_bundle_creation() {
        let bundle = create_test_bundle();
        assert_eq!(bundle.block_height(), 0);
        assert_eq!(
            bundle.certificate_hash(),
            bundle.inclusion_proof.certificate_hash
        );
    }

    #[test]
    fn n9c_bundle_verification() {
        let bundle = create_test_bundle();
        assert!(bundle.verify().is_ok());
    }

    #[test]
    fn n9c_tampered_bundle_fails() {
        let mut bundle = create_test_bundle();
        bundle.block_header.state_root = "tampered".into();
        assert!(bundle.verify().is_err());
    }

    #[test]
    fn n9d_bundle_response() {
        let bundle = create_test_bundle();
        let height = bundle.block_height();
        let response = ProofBundleMessage::BundleResponse {
            bundle: bundle.clone(),
        };
        match response {
            ProofBundleMessage::BundleResponse { bundle: received } => {
                assert!(received.verify().is_ok());
                assert_eq!(received.block_height(), height);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n9d_bundle_not_found() {
        let response = ProofBundleMessage::BundleNotFound {
            reason: "Not found".into(),
        };
        match response {
            ProofBundleMessage::BundleNotFound { reason } => assert!(!reason.is_empty()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn n9e_bundle_builder() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
            std::slice::from_ref(&cert),
        ));
        let block = ConstitutionalBlock::new(
            1,
            "parent_hash".into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );
        let bundle = BundleBuilder::build_from_runtime(block, &rt, [0u8; 32]).unwrap();
        assert!(bundle.verify().is_ok());
        assert_eq!(bundle.block_height(), 1);
    }

    #[test]
    fn n9_serialize_certificate_message() {
        let bundle = create_test_bundle();
        let request = CertificateMessage::RequestCertificate {
            certificate_hash: bundle.certificate_hash(),
        };
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CertificateMessage = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("distribution: invalid certificate message: {e}");
                return;
            }
        };
        match deserialized {
            CertificateMessage::RequestCertificate { certificate_hash } => {
                assert_eq!(certificate_hash, bundle.certificate_hash())
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn n9_serialize_bundle() {
        let bundle = create_test_bundle();
        let json = serde_json::to_string(&bundle).unwrap();
        let deserialized: LightClientProofBundle = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("distribution: invalid proof bundle: {e}");
                return;
            }
        };
        assert!(deserialized.verify().is_ok());
        assert_eq!(deserialized.block_height(), bundle.block_height());
        assert_eq!(deserialized.certificate_hash(), bundle.certificate_hash());
    }
}
