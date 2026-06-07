// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

pub struct DeterministicNormalizer;

impl DeterministicNormalizer {
    /// Ensures uniform newlines (LF), trims trailing whitespace,
    /// and guarantees a trailing newline.
    pub fn normalize(input: &str) -> String {
        let mut out = input
            .replace("\r\n", "\n")
            .lines()
            .map(|line| line.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out
    }
}
