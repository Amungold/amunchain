// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use amun_constitutional_kernel::receipt::ExecutionReceipt;
use amun_constitutional_commitments::MerkleProof;

/// A proof-carrying receipt binds an execution receipt with the Merkle
/// proofs that attest its inclusion in the constitutional state,
/// governance, and execution domains.  With these proofs, any observer
/// can verify the receipt without trusting the block proposer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofCarryingReceipt {
    pub receipt: ExecutionReceipt,
    /// Merkle proof of inclusion in the state tree (mandatory).
    pub state_proof: MerkleProof,
    /// Optional proof of inclusion in the governance tree.
    pub governance_proof: Option<MerkleProof>,
    /// Optional proof of inclusion in the execution tree.
    pub execution_proof: Option<MerkleProof>,
    /// Roots as published in the block header.
    pub state_root: String,
    pub governance_root: String,
    pub execution_root: String,
    /// The hash of the block that contains this receipt.
    pub block_hash: String,
}

impl ProofCarryingReceipt {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        receipt: ExecutionReceipt,
        state_proof: MerkleProof,
        governance_proof: Option<MerkleProof>,
        execution_proof: Option<MerkleProof>,
        state_root: String,
        governance_root: String,
        execution_root: String,
        block_hash: String,
    ) -> Self {
        Self {
            receipt,
            state_proof,
            governance_proof,
            execution_proof,
            state_root,
            governance_root,
            execution_root,
            block_hash,
        }
    }
}
