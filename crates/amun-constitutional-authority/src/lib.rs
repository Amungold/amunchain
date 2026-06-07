// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

//! Constitutional authority layer.
//!
//! This crate provides the primitives for building replay-safe,
//! self-certifying authority chains on top of the constitutional
//! signing infrastructure.  Every certificate is content-addressed
//! via a domain-separated BLAKE3 digest computed over a stable
//! subset of its fields, avoiding circular self-reference.
//!
//! # Organisation
//! - `certificate`  – the core `ConstitutionalCertificate` type
//! - `chain`        – cryptographically verified certificate chains
//! - `revocation`   – deterministic, append-only revocation registry
//! - `rotation`     – key-rotation semantics with signed proofs
//! - `trust`        – frozen trust anchors

pub mod certificate;
pub mod chain;
pub mod revocation;
pub mod rotation;
pub mod trust;

pub use certificate::ConstitutionalCertificate;
pub use chain::CertificateChain;
pub use revocation::RevocationRegistry;
pub use rotation::KeyRotationLaw;
pub use trust::TrustAnchor;
