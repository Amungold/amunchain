// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use amun_constitution_builder::{
    canonical_bytes::CanonicalSerialize,
    digest::ArtifactDigest,
};

/// A constitutional certificate whose identity is derived from a
/// stable subset of its fields (everything except `certificate_id`,
/// `issuer`, and `subject`).  This guarantees that the certificate
/// can reference its own identity without creating a circular
/// dependency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ConstitutionalCertificate {
    pub schema_version: u32,
    pub certificate_id: String,          // hex BLAKE3 of identity bytes
    pub issuer: String,                  // certificate_id of the issuer
    pub subject: String,                 // certificate_id of the subject
    pub subject_verifying_key_hex: String,
    pub lineage_parent_hash: Option<String>,
    pub epoch_start: String,
    pub epoch_end: String,
    pub constitutional_scope: String,
    pub timestamp: String,
}

impl ArtifactDigest for ConstitutionalCertificate {
    fn domain_separator(&self) -> &'static [u8] {
        b"AMUN_CERTIFICATE_V1"
    }

    fn constitutional_digest(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.domain_separator());
        hasher.update(&self.identity_bytes());
        *hasher.finalize().as_bytes()
    }
}

impl ConstitutionalCertificate {
    // ---------------------------------------------------------------
    // Constructors – roles are separated so invariants are clear
    // ---------------------------------------------------------------

    /// Create a self-signed root certificate.
    pub fn new_root(
        subject_public_key_hex: String,
        epoch_start: String,
        epoch_end: String,
        scope: String,
        timestamp: String,
    ) -> Self {
        let mut tmp = Self::empty();
        tmp.subject_verifying_key_hex = subject_public_key_hex;
        tmp.epoch_start = epoch_start;
        tmp.epoch_end = epoch_end;
        tmp.constitutional_scope = scope;
        tmp.timestamp = timestamp;

        let id = tmp.compute_id();
        tmp.certificate_id = id.clone();
        tmp.issuer = id.clone();
        tmp.subject = id;
        tmp
    }

    /// Create a child certificate issued by a parent.
    pub fn new_child(
        issuer_id: String,               // parent certificate_id
        subject_public_key_hex: String,
        lineage_parent: String,          // same as issuer_id
        epoch_start: String,
        epoch_end: String,
        scope: String,
        timestamp: String,
    ) -> Self {
        let mut tmp = Self::empty();
        tmp.issuer = issuer_id;
        tmp.subject_verifying_key_hex = subject_public_key_hex;
        tmp.lineage_parent_hash = Some(lineage_parent);
        tmp.epoch_start = epoch_start;
        tmp.epoch_end = epoch_end;
        tmp.constitutional_scope = scope;
        tmp.timestamp = timestamp;

        let id = tmp.compute_id();
        tmp.certificate_id = id.clone();
        tmp.subject = id;
        tmp
    }

    // ---------------------------------------------------------------
    // Identity helpers
    // ---------------------------------------------------------------

    /// Build a bare certificate used as a template for identity computation.
    fn empty() -> Self {
        Self {
            schema_version: 1,
            certificate_id: String::new(),
            issuer: String::new(),
            subject: String::new(),
            subject_verifying_key_hex: String::new(),
            lineage_parent_hash: None,
            epoch_start: String::new(),
            epoch_end: String::new(),
            constitutional_scope: String::new(),
            timestamp: String::new(),
        }
    }

    /// Compute the certificate id from the canonical identity bytes.
    fn compute_id(&self) -> String {
        let bytes = self.identity_bytes();
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CERTIFICATE_V1");
        hasher.update(&bytes);
        hex::encode(hasher.finalize().as_bytes())
    }

    /// Canonical bytes used for identity hashing.
    ///
    /// This deliberately omits `certificate_id`, `issuer`, and `subject`
    /// because those fields depend on the identity itself for root
    /// certificates, which would create a circular dependency.
    fn identity_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.certificate_id = String::new();
        c.issuer = String::new();
        c.subject = String::new();
        serde_json::to_vec(&c).expect("Identity serialization must not fail")
    }

    /// Convenience: return the full artifact digest (including all fields).
    pub fn full_digest_hex(&self) -> String {
        self.digest_hex()
    }
}

impl CanonicalSerialize for ConstitutionalCertificate {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Canonical serialization must not fail")
    }
}
