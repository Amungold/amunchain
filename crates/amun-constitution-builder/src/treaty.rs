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
pub struct TreatyArtifact {
    pub schema_version: u32,
    pub treaty_id: String,
    pub civilizations: Vec<String>,
    pub scope: String,
    pub replay_boundaries: Vec<String>,
    pub compatibility_guarantees: Vec<String>,
    pub revocation_conditions: Vec<String>,
    pub timestamp: String,
}

impl ArtifactDigest for TreatyArtifact {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_TREATY_V1"
    }
}

impl TreatyArtifact {
    pub fn new(
        treaty_id: String,
        mut civilizations: Vec<String>,
        timestamp: String,
    ) -> Self {
        civilizations.sort(); // Deterministic ordering
        Self {
            schema_version: 1,
            treaty_id,
            civilizations,
            scope: "Bilateral".into(),
            replay_boundaries: vec!["Treaty-scoped".into()],
            compatibility_guarantees: vec!["Sovereign execution preserved".into()],
            revocation_conditions: vec![
                "Bilateral agreement".into(),
                "Lineage discontinuity".into(),
            ],
            timestamp,
        }
    }
}

impl CanonicalEmit for TreatyArtifact {
    fn emit_canonical(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("CONSTITUTIONAL TREATY v1".to_string());
        lines.push("".to_string());
        lines.push(format!("Schema Version: {}", self.schema_version));
        lines.push(format!("Treaty ID: {}", self.treaty_id));
        lines.push(format!("Timestamp: {}", self.timestamp));
        lines.push(format!("Scope: {}", self.scope));
        lines.push("".to_string());
        lines.push("Participating Civilizations:".to_string());
        for c in &self.civilizations {
            lines.push(format!("- {}", c));
        }
        lines.push("".to_string());
        lines.push("Replay Boundaries:".to_string());
        for b in &self.replay_boundaries {
            lines.push(format!("- {}", b));
        }
        lines.push("".to_string());
        lines.push("Compatibility Guarantees:".to_string());
        for g in &self.compatibility_guarantees {
            lines.push(format!("- {}", g));
        }
        lines.push("".to_string());
        lines.push("Revocation Conditions:".to_string());
        for c in &self.revocation_conditions {
            lines.push(format!("- {}", c));
        }
        DeterministicNormalizer::normalize(&lines.join("\n"))
    }
}

impl CanonicalSerialize for TreatyArtifact {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
