// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use amun_constitutional_authority::ConstitutionalCertificate;
use serde::{Deserialize, Serialize};

/// The ExecutionContext binds an operation to the constitutional authority
/// that authorises it and the epoch in which it executes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// The certificate chain that proves the caller's authority.
    pub authority_chain: Vec<ConstitutionalCertificate>,
    /// The epoch during which the operation is valid.
    pub epoch: String,
    /// Optional metadata for auditing.
    pub metadata: serde_json::Value,
}

impl ExecutionContext {
    pub fn new(authority_chain: Vec<ConstitutionalCertificate>, epoch: String) -> Self {
        Self {
            authority_chain,
            epoch,
            metadata: serde_json::Value::Null,
        }
    }
}
