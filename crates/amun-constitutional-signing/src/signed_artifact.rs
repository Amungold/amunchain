use crate::keys::ConstitutionalKeyPair;
use crate::signature::ConstitutionalSignature;
use amun_constitution_builder::digest::ArtifactDigest;
use ed25519_dalek::{Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SignedArtifact<T: ArtifactDigest> {
    pub artifact: T,
    pub signature: ConstitutionalSignature,
}

impl<T: ArtifactDigest> SignedArtifact<T> {
    pub fn sign(artifact: T, keypair: &ConstitutionalKeyPair) -> Self {
        let digest = artifact.constitutional_digest();
        let sig = keypair.sign(&digest);
        let verifying_hex = keypair.verifying_key_hex();
        Self {
            artifact,
            signature: ConstitutionalSignature::new(sig, verifying_hex),
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let key_bytes = hex::decode(&self.signature.verifying_key_hex)
            .map_err(|e| format!("Invalid verifying key hex: {}", e))?;
        let arr: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| "Verifying key must be 32 bytes".to_string())?;
        let verifying_key =
            VerifyingKey::from_bytes(&arr).map_err(|e| format!("Invalid verifying key: {}", e))?;

        let digest = self.artifact.constitutional_digest();
        verifying_key
            .verify(&digest, self.signature.dalek_signature())
            .map_err(|e| format!("Signature verification failed: {}", e))
    }
}

// Manual PartialEq + Debug to satisfy DelegationChain derives
impl<T: ArtifactDigest + PartialEq> PartialEq for SignedArtifact<T> {
    fn eq(&self, other: &Self) -> bool {
        self.artifact == other.artifact
            && self.signature.verifying_key_hex == other.signature.verifying_key_hex
            && self.signature.to_hex() == other.signature.to_hex()
    }
}

impl<T: ArtifactDigest + std::fmt::Debug> std::fmt::Debug for SignedArtifact<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignedArtifact")
            .field("artifact", &self.artifact)
            .field("signature", &self.signature.verifying_key_hex)
            .finish()
    }
}
