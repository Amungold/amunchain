// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Deterministic, append-only revocation registry.
///
/// Uses `BTreeSet` so that serialised output is always ordered,
/// guaranteeing replay stability.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RevocationRegistry {
    revoked: BTreeSet<String>,
}

impl RevocationRegistry {
    pub fn new() -> Self {
        Self { revoked: BTreeSet::new() }
    }

    pub fn revoke(&mut self, certificate_id: String) {
        self.revoked.insert(certificate_id);
    }

    pub fn is_revoked(&self, certificate_id: &str) -> bool {
        self.revoked.contains(certificate_id)
    }
}
