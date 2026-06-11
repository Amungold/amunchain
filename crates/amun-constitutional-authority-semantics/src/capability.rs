use amun_constitution_builder::canonical_bytes::CanonicalSerialize;
use amun_constitution_builder::digest::ArtifactDigest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorityCapability {
    pub schema_version: u32,
    pub capability_id: String,
    pub action: String,
    pub scope: String,
    pub subject_verifying_key_hex: String,
    pub epoch_start: String,
    pub epoch_end: String,
    pub constraints: Vec<String>,
}

impl ArtifactDigest for AuthorityCapability {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_AUTH_CAPABILITY_V1"
    }
}

impl AuthorityCapability {
    pub fn new(
        action: String,
        scope: String,
        subject_key: String,
        epoch_start: String,
        epoch_end: String,
        constraints: Vec<String>,
    ) -> Self {
        let mut tmp = Self {
            schema_version: 1,
            capability_id: String::new(),
            action,
            scope,
            subject_verifying_key_hex: subject_key,
            epoch_start,
            epoch_end,
            constraints,
        };
        let id = tmp.compute_id();
        tmp.capability_id = id;
        tmp
    }

    fn identity_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.capability_id = String::new();
        serde_json::to_vec(&c).expect("serialize")
    }
    fn compute_id(&self) -> String {
        let mut h = blake3::Hasher::new();
        h.update(b"AMUN_AUTH_CAPABILITY_V1");
        h.update(&self.identity_bytes());
        hex::encode(h.finalize().as_bytes())
    }
}

impl CanonicalSerialize for AuthorityCapability {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serialize")
    }
}

pub type CapabilityWitness = amun_constitutional_signing::SignedArtifact<AuthorityCapability>;
