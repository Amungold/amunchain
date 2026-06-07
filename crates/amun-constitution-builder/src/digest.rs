// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use blake3::Hasher;
use crate::canonical_bytes::CanonicalSerialize;

pub trait ArtifactDigest: CanonicalSerialize {
    /// Domain separator for constitutional hashing (prevents cross-artifact collisions).
    fn domain_separator(&self) -> &'static [u8];

    /// Computes the BLAKE3 constitutional digest over domain_separator + canonical_bytes.
    fn constitutional_digest(&self) -> [u8; 32] {
        let bytes = self.canonical_bytes();
        let mut hasher = Hasher::new();
        hasher.update(self.domain_separator());
        hasher.update(&bytes);
        *hasher.finalize().as_bytes()
    }

    fn digest_hex(&self) -> String {
        hex::encode(self.constitutional_digest())
    }
}
