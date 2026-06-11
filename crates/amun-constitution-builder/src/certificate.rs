// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::canonical_bytes::CanonicalSerialize;
use crate::digest::ArtifactDigest;
use crate::emitter::CanonicalEmit;
use crate::normalize::DeterministicNormalizer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct FreezeCertificate {
    pub schema_version: u32,
    pub civilization: String,
    pub release_version: String,
    pub specification_hash: String,
    pub genesis_hash: String,
    pub frozen_properties: Vec<String>,
    pub timestamp: String,
}

impl ArtifactDigest for FreezeCertificate {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_FREEZE_V1"
    }
}

impl FreezeCertificate {
    pub fn new(
        spec_hash: String,
        genesis_hash: String,
        version: String,
        timestamp: String,
    ) -> Self {
        let mut frozen = vec![
            "canonical serialization".into(),
            "domain separation".into(),
            "replay semantics".into(),
            "constitutional verification".into(),
            "deterministic execution".into(),
            "frozen endianness".into(),
            "frozen SMT depth".into(),
            "frozen proof semantics".into(),
        ];
        frozen.sort(); // Ensure deterministic ordering
        Self {
            schema_version: 1,
            civilization: "AmunChain".into(),
            release_version: version,
            specification_hash: spec_hash,
            genesis_hash,
            frozen_properties: frozen,
            timestamp,
        }
    }
}

impl CanonicalEmit for FreezeCertificate {
    fn emit_canonical(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("AMUNCHAIN FREEZE CERTIFICATE v1".to_string());
        lines.push("".to_string());
        lines.push(format!("Schema Version: {}", self.schema_version));
        lines.push(format!("Civilization: {}", self.civilization));
        lines.push(format!("Release: {}", self.release_version));
        lines.push(format!("Timestamp: {}", self.timestamp));
        lines.push("".to_string());
        lines.push(format!("Specification Hash: {}", self.specification_hash));
        lines.push("".to_string());
        lines.push("Frozen Constitutional Properties:".to_string());
        for prop in &self.frozen_properties {
            lines.push(format!("- {}", prop));
        }
        lines.push("".to_string());
        lines.push("Any semantic modification changes constitutional identity.".to_string());
        DeterministicNormalizer::normalize(&lines.join("\n"))
    }
}

impl CanonicalSerialize for FreezeCertificate {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
