// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

pub mod manifest;
pub mod certificate;
pub mod federation;
pub mod treaty;
pub mod hashing;
pub mod emitter;
pub mod canonical_bytes;
pub mod normalize;
pub mod verify;
pub mod types;
pub mod digest;

pub use manifest::ConstitutionalManifest;
pub use certificate::FreezeCertificate;
pub use federation::FederationArtifact;
pub use treaty::TreatyArtifact;
pub use hashing::{compute_specification_hash, compute_all_hashes, SpecificationHashes};
pub use emitter::CanonicalEmit;
pub use canonical_bytes::CanonicalSerialize;
pub use normalize::DeterministicNormalizer;
pub use verify::VerificationEngine;
pub use types::*;
pub use digest::ArtifactDigest;
