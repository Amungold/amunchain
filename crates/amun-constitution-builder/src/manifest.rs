// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use crate::emitter::CanonicalEmit;
use crate::canonical_bytes::CanonicalSerialize;
use crate::normalize::DeterministicNormalizer;
use crate::digest::ArtifactDigest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ConstitutionalManifest {
    pub schema_version: u32,
    pub civilization: String,
    pub genesis_hash: String,
    pub specification_hash: String,
    pub constitution_version: u32,
    pub codec_version: u32,
    pub proof_version: u32,
    pub hash_algorithm: String,
    pub endianness: String,
    pub smt_depth: u32,
    pub timestamp: String,
}

impl ArtifactDigest for ConstitutionalManifest {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_MANIFEST_V1"
    }
}

impl ConstitutionalManifest {
    pub fn new(
        genesis_hash: String,
        specification_hash: String,
        timestamp: String,
    ) -> Self {
        Self {
            schema_version: 1,
            civilization: "AmunChain".into(),
            genesis_hash,
            specification_hash,
            constitution_version: 1,
            codec_version: 1,
            proof_version: 1,
            hash_algorithm: "BLAKE3".into(),
            endianness: "little".into(),
            smt_depth: 256,
            timestamp,
        }
    }
}

impl CanonicalEmit for ConstitutionalManifest {
    fn emit_canonical(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("# Genesis Constitutional Manifest".to_string());
        lines.push("".to_string());
        lines.push("## Civilization Identity".to_string());
        lines.push("".to_string());
        lines.push("| Property | Value |".to_string());
        lines.push("|---|---|".to_string());
        lines.push(format!("| Schema Version | {} |", self.schema_version));
        lines.push(format!("| Civilization Name | {} |", self.civilization));
        lines.push(format!("| Genesis Hash | {} |", self.genesis_hash));
        lines.push(format!("| Specification Hash | {} |", self.specification_hash));
        lines.push(format!("| Constitution Version | {} |", self.constitution_version));
        lines.push(format!("| Codec Version | {} |", self.codec_version));
        lines.push(format!("| Proof Version | {} |", self.proof_version));
        lines.push(format!("| Hash Algorithm | {} |", self.hash_algorithm));
        lines.push(format!("| Endianness | {} |", self.endianness));
        lines.push(format!("| SMT Depth | {} |", self.smt_depth));
        lines.push(format!("| Timestamp | {} |", self.timestamp));
        lines.push("".to_string());
        DeterministicNormalizer::normalize(&lines.join("\n"))
    }
}

impl CanonicalSerialize for ConstitutionalManifest {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
