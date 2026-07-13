use amun_constitutional_signing::{ConstitutionalKeyPair, ConstitutionalSignature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerCertificate {
    pub peer_id: super::peer_id::ConstitutionalPeerId,
    pub signature: ConstitutionalSignature,
}

impl PeerCertificate {
    pub fn self_sign(
        peer_id: super::peer_id::ConstitutionalPeerId,
        keypair: &ConstitutionalKeyPair,
    ) -> Self {
        let bytes = serde_json::to_vec(&peer_id).expect("PeerId serialization must not fail");
        let mut h = blake3::Hasher::new();
        h.update(b"AMUN_PEER_CERT_V1");
        h.update(&bytes);
        let digest = *h.finalize().as_bytes();
        let sig = keypair.sign(&digest);
        let vk_hex = keypair.verifying_key_hex();
        Self {
            peer_id,
            signature: ConstitutionalSignature::new(sig, vk_hex),
        }
    }
}
