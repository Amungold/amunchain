use ed25519_dalek::Verifier;
use super::certificate::PeerCertificate;

pub struct IdentityVerifier;

impl IdentityVerifier {
    pub fn verify(cert: &PeerCertificate, trusted_genesis: &str) -> Result<(), String> {
        if cert.peer_id.genesis_hash != trusted_genesis {
            return Err("Peer does not belong to the trusted civilisation".into());
        }

        let bytes = serde_json::to_vec(&cert.peer_id)
            .map_err(|e| format!("Serialization error: {}", e))?;
        let mut h = blake3::Hasher::new();
        h.update(b"AMUN_PEER_CERT_V1");
        h.update(&bytes);
        let digest = *h.finalize().as_bytes();

        let vk_bytes = hex::decode(&cert.signature.verifying_key_hex)
            .map_err(|e| format!("Invalid key hex: {}", e))?;
        let arr: [u8; 32] = vk_bytes.try_into()
            .map_err(|_| "Key must be 32 bytes".to_string())?;
        let vk = ed25519_dalek::VerifyingKey::from_bytes(&arr)
            .map_err(|e| format!("Invalid verifying key: {}", e))?;

        vk.verify(&digest, cert.signature.dalek_signature())
            .map_err(|e| format!("Signature verification failed: {}", e))
    }
}
