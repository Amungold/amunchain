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
pub struct FederationArtifact {
    pub schema_version: u32,
    pub civilization_a: String,
    pub civilization_b: String,
    pub genesis_hash_a: String,
    pub genesis_hash_b: String,
    pub specification_hash_a: String,
    pub specification_hash_b: String,
    pub treaty_ids: Vec<String>,
    pub compatibility_level: String,
    pub replay_boundary: String,
    pub sovereignty_constraints: Vec<String>,
    pub timestamp: String,
}

impl ArtifactDigest for FederationArtifact {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_FEDERATION_V1"
    }
}

impl FederationArtifact {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        civ_a: String,
        civ_b: String,
        gen_a: String,
        gen_b: String,
        spec_a: String,
        spec_b: String,
        mut treaties: Vec<String>,
        timestamp: String,
    ) -> Self {
        treaties.sort(); // Deterministic ordering
        let constraints = vec![
            "No authority escalation".into(),
            "No state injection".into(),
            "Replay isolation preserved".into(),
        ];
        Self {
            schema_version: 1,
            civilization_a: civ_a,
            civilization_b: civ_b,
            genesis_hash_a: gen_a,
            genesis_hash_b: gen_b,
            specification_hash_a: spec_a,
            specification_hash_b: spec_b,
            treaty_ids: treaties,
            compatibility_level: "Compatible".into(),
            replay_boundary: "Treaty-scoped".into(),
            sovereignty_constraints: constraints,
            timestamp,
        }
    }
}

impl CanonicalEmit for FederationArtifact {
    fn emit_canonical(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("FEDERATION CERTIFICATE v1".to_string());
        lines.push("".to_string());
        lines.push(format!("Schema Version: {}", self.schema_version));
        lines.push(format!("Civilization A: {}", self.civilization_a));
        lines.push(format!("Civilization B: {}", self.civilization_b));
        lines.push(format!("Genesis Hash A: {}", self.genesis_hash_a));
        lines.push(format!("Genesis Hash B: {}", self.genesis_hash_b));
        lines.push(format!(
            "Specification Hash A: {}",
            self.specification_hash_a
        ));
        lines.push(format!(
            "Specification Hash B: {}",
            self.specification_hash_b
        ));
        lines.push(format!("Compatibility Level: {}", self.compatibility_level));
        lines.push(format!("Replay Boundary: {}", self.replay_boundary));
        lines.push(format!("Timestamp: {}", self.timestamp));
        lines.push("".to_string());
        lines.push("Treaties:".to_string());
        for t in &self.treaty_ids {
            lines.push(format!("- {}", t));
        }
        lines.push("".to_string());
        lines.push("Sovereignty Constraints:".to_string());
        for c in &self.sovereignty_constraints {
            lines.push(format!("- {}", c));
        }
        DeterministicNormalizer::normalize(&lines.join("\n"))
    }
}

impl CanonicalSerialize for FederationArtifact {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
