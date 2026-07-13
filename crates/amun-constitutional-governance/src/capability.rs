// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use amun_constitution_builder::{canonical_bytes::CanonicalSerialize, digest::ArtifactDigest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Capability {
    pub schema_version: u32,
    pub capability_id: String,
    pub action: String,
    pub scope: String,
    pub subject_verifying_key_hex: String, // who holds this capability
    pub epoch_start: String,
    pub epoch_end: String,
    pub parameters: serde_json::Value,
}

impl ArtifactDigest for Capability {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_CAPABILITY_V1"
    }

    fn constitutional_digest(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.domain_separator());
        hasher.update(&self.identity_bytes());
        *hasher.finalize().as_bytes()
    }
}

impl Capability {
    pub fn new(
        action: String,
        scope: String,
        subject_public_key_hex: String,
        epoch_start: String,
        epoch_end: String,
        parameters: serde_json::Value,
    ) -> Self {
        let mut tmp = Self {
            schema_version: 1,
            capability_id: String::new(),
            action,
            scope,
            subject_verifying_key_hex: subject_public_key_hex,
            epoch_start,
            epoch_end,
            parameters,
        };
        let id = tmp.compute_id();
        tmp.capability_id = id;
        tmp
    }

    fn identity_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.capability_id = String::new();
        serde_json::to_vec(&c).expect("Capability serialization must not fail")
    }

    fn compute_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CAPABILITY_V1");
        hasher.update(&self.identity_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
}

impl CanonicalSerialize for Capability {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}

pub type CapabilityCertificate = amun_constitutional_signing::SignedArtifact<Capability>;
