// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::canonical_bytes::CanonicalSerialize;

pub struct VerificationEngine;

impl VerificationEngine {
    /// Verifies two artifacts are structurally identical and produce identical byte representations.
    pub fn verify_replay<T: CanonicalSerialize + PartialEq>(
        artifact: &T,
        recreated: &T,
    ) -> Result<(), String> {
        if artifact != recreated {
            return Err("Artifacts differ structurally".into());
        }
        let bytes_a = artifact.canonical_bytes();
        let bytes_b = recreated.canonical_bytes();
        if bytes_a != bytes_b {
            return Err("Canonical byte representations differ".into());
        }
        Ok(())
    }
}
