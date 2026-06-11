// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

pub mod canonical_bytes;
pub mod certificate;
pub mod digest;
pub mod emitter;
pub mod federation;
pub mod hashing;
pub mod manifest;
pub mod normalize;
pub mod treaty;
pub mod types;
pub mod verify;

pub use canonical_bytes::CanonicalSerialize;
pub use certificate::FreezeCertificate;
pub use digest::ArtifactDigest;
pub use emitter::CanonicalEmit;
pub use federation::FederationArtifact;
pub use hashing::{compute_all_hashes, compute_specification_hash, SpecificationHashes};
pub use manifest::ConstitutionalManifest;
pub use normalize::DeterministicNormalizer;
pub use treaty::TreatyArtifact;
pub use types::*;
pub use verify::VerificationEngine;
