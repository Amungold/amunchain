// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::receipt::ProofCarryingReceipt;
use amun_constitutional_commitments::SparseMerkleTree;

/// Verifies proof-carrying receipts without access to the full chain state.
pub struct ProofVerifier;

impl ProofVerifier {
    /// Verify a single proof-carrying receipt against the published roots.
    /// Returns Ok(()) if all proofs are valid, or a descriptive error.
    pub fn verify_receipt(receipt: &ProofCarryingReceipt) -> Result<(), String> {
        // Verify state proof
        let state_root_bytes = hex::decode(&receipt.state_root)
            .map_err(|e| format!("Invalid state root hex: {}", e))?;
        let state_root: [u8; 32] = state_root_bytes
            .try_into()
            .map_err(|_| "State root must be 32 bytes".to_string())?;

        let state_tree = SparseMerkleTree::new(b"AMUN_STATE_DOMAIN");
        if !state_tree.verify(&state_root, &receipt.state_proof) {
            return Err("State proof verification failed".into());
        }

        // Verify governance proof if present
        if let Some(ref gov_proof) = receipt.governance_proof {
            let gov_root_bytes = hex::decode(&receipt.governance_root)
                .map_err(|e| format!("Invalid governance root hex: {}", e))?;
            let gov_root: [u8; 32] = gov_root_bytes
                .try_into()
                .map_err(|_| "Governance root must be 32 bytes".to_string())?;

            let gov_tree = SparseMerkleTree::new(b"AMUN_GOVERNANCE_DOMAIN");
            if !gov_tree.verify(&gov_root, gov_proof) {
                return Err("Governance proof verification failed".into());
            }
        }

        // Verify execution proof if present
        if let Some(ref exec_proof) = receipt.execution_proof {
            let exec_root_bytes = hex::decode(&receipt.execution_root)
                .map_err(|e| format!("Invalid execution root hex: {}", e))?;
            let exec_root: [u8; 32] = exec_root_bytes
                .try_into()
                .map_err(|_| "Execution root must be 32 bytes".to_string())?;

            let exec_tree = SparseMerkleTree::new(b"AMUN_EXECUTION_DOMAIN");
            if !exec_tree.verify(&exec_root, exec_proof) {
                return Err("Execution proof verification failed".into());
            }
        }

        Ok(())
    }
}
