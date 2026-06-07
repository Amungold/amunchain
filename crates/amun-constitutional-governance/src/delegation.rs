// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use amun_constitutional_signing::SignedArtifact;
use crate::capability::Capability;

pub type DelegateCertificate = SignedArtifact<Capability>;

pub fn verify_delegation_chain(
    chain: &[DelegateCertificate],
    root_public_key_hex: &str,
) -> Result<(), String> {
    if chain.is_empty() {
        return Ok(());
    }

    let first = &chain[0];
    first.verify().map_err(|e| format!("First delegation invalid: {}", e))?;
    if first.signature.verifying_key_hex != root_public_key_hex {
        return Err("First delegation not signed by root authority".into());
    }

    for i in 1..chain.len() {
        let prev = &chain[i - 1];
        let curr = &chain[i];

        // The current certificate must be signed by the previous subject's key.
        let prev_subject_key = &prev.artifact.subject_verifying_key_hex;
        if &curr.signature.verifying_key_hex != prev_subject_key {
            return Err(format!(
                "Delegation {} not signed by previous subject key",
                curr.artifact.capability_id
            ));
        }

        curr.verify()
            .map_err(|e| format!("Delegation {} invalid: {}", curr.artifact.capability_id, e))?;

        // Epoch containment
        if curr.artifact.epoch_start < prev.artifact.epoch_start
            || curr.artifact.epoch_end > prev.artifact.epoch_end
        {
            return Err(format!(
                "Delegation {} epoch exceeds parent epoch",
                curr.artifact.capability_id
            ));
        }
    }
    Ok(())
}
