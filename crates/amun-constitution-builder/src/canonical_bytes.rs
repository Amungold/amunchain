// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

pub trait CanonicalSerialize {
    fn canonical_bytes(&self) -> Vec<u8>;
}
