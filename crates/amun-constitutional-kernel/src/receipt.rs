// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use amun_constitution_builder::{
    canonical_bytes::CanonicalSerialize,
    digest::ArtifactDigest,
};

/// An execution receipt proves that a state transition occurred under
/// constitutional authority.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionReceipt {
    pub action: String,
    pub scope: String,
    pub old_version: u64,
    pub new_version: u64,
    pub epoch: String,
    pub receipt_id: String,
}

impl ArtifactDigest for ExecutionReceipt {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_RECEIPT_V1"
    }
}

impl ExecutionReceipt {
    pub fn new(
        action: String,
        scope: String,
        old_version: u64,
        new_version: u64,
        epoch: String,
    ) -> Self {
        let mut tmp = Self {
            action,
            scope,
            old_version,
            new_version,
            epoch,
            receipt_id: String::new(),
        };
        let id = tmp.compute_id();
        tmp.receipt_id = id;
        tmp
    }

    fn identity_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.receipt_id = String::new();
        serde_json::to_vec(&c).expect("Receipt serialization must not fail")
    }

    fn compute_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_RECEIPT_V1");
        hasher.update(&self.identity_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
}

impl CanonicalSerialize for ExecutionReceipt {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
