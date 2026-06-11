// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::certificate::ConstitutionalCertificate;
use amun_constitutional_signing::SignedArtifact;

/// A frozen trust anchor representing the root of a constitutional
/// authority domain.
pub struct TrustAnchor {
    pub signed_cert: SignedArtifact<ConstitutionalCertificate>,
    pub genesis_hash: String,
    pub frozen_scope: String,
}

impl TrustAnchor {
    pub fn new(
        signed_cert: SignedArtifact<ConstitutionalCertificate>,
        genesis_hash: String,
        scope: String,
    ) -> Self {
        Self {
            signed_cert,
            genesis_hash,
            frozen_scope: scope,
        }
    }

    pub fn is_self_signed(&self) -> bool {
        self.signed_cert.artifact.issuer == self.signed_cert.artifact.subject
    }

    pub fn verify(&self) -> Result<(), String> {
        if !self.is_self_signed() {
            return Err("Trust anchor must be self-signed".into());
        }
        self.signed_cert.verify()
    }
}
