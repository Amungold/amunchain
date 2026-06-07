// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use amun_constitutional_signing::{ConstitutionalKeyPair, SignedArtifact};
use crate::certificate::ConstitutionalCertificate;

/// Enforces cryptographically verified key rotation.
pub struct KeyRotationLaw;

impl KeyRotationLaw {
    /// Validate that `new_signed_cert` is a valid rotation from the
    /// authority identified by `old_public_key_hex`.
    ///
    /// The new certificate must be signed by the old authority key
    /// and its subject must differ from the old key.
    pub fn validate_rotation(
        old_public_key_hex: &str,
        new_signed_cert: &SignedArtifact<ConstitutionalCertificate>,
        old_keypair: &ConstitutionalKeyPair,
    ) -> Result<(), String> {
        new_signed_cert
            .verify()
            .map_err(|e| format!("Rotation signature invalid: {}", e))?;

        if new_signed_cert.artifact.subject_verifying_key_hex == old_public_key_hex {
            return Err("Rotated key must differ from previous key".into());
        }

        if old_keypair.verifying_key_hex() != old_public_key_hex {
            return Err("Provided old key does not match expected public key".into());
        }

        Ok(())
    }
}
