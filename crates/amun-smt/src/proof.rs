//! Merkle proof types for inclusion and absence verification.
//!
//! Proofs are self-describing and versioned. They do NOT depend on
//! the tree's in-memory representation — only on canonical hashes.

use crate::hash::Hash;
use crate::node::EMPTY_NODE_HASH;
use crate::error::SmtError;

/// Maximum number of proof steps (depth limit).
pub const MAX_PROOF_STEPS: usize = 256;

/// Proof format version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofVersion {
    V1 = 0x01,
}

/// Full witness of a leaf needed for `LeafDivergence` absence proofs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafWitness {
    pub key_hash: [u8; 32],
    pub value_hash: [u8; 32],
    pub version: u64,
}

/// Why does an absence proof terminate?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbsenceReason {
    /// The queried key diverges from the prefix of a branch.
    PrefixMismatch { mismatched_bit: u8 },
    /// The path leads to an empty child.
    EmptyChild,
    /// The proof stops at a leaf whose key differs from the queried key.
    LeafDivergence {
        leaf_witness: LeafWitness,
        divergence_depth: u8,
    },
    /// The entire tree is empty (specialized absence).
    EmptyTree,
}

/// One step along the proof path (from root to terminal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofStep {
    pub skip_len: u8,
    /// Packed prefix bits (big-endian within bytes) and the bit length.
    pub prefix_bits: (Vec<u8>, u8),
    pub sibling: Hash,
    /// If true, the sibling is on the left (the traversed child is on the right).
    pub is_left_sibling: bool,
}

/// A self-contained Merkle proof for a single key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    pub proof_version: ProofVersion,
    pub key_hash: [u8; 32],
    pub is_inclusion: bool,
    pub value_hash: Option<[u8; 32]>,
    pub leaf_version: Option<u64>,
    pub absence_reason: Option<AbsenceReason>,
    /// Steps from root to terminal (top-down).
    pub steps: Vec<ProofStep>,
}

impl MerkleProof {
    // ---- Constructors ----

    pub fn new_inclusion(
        key_hash: [u8; 32],
        value_hash: [u8; 32],
        version: u64,
        steps: Vec<ProofStep>,
    ) -> Result<Self, SmtError> {
        if steps.len() > MAX_PROOF_STEPS {
            return Err(SmtError::ProofTooLarge {
                steps: steps.len(),
                max: MAX_PROOF_STEPS,
            });
        }
        Ok(Self {
            proof_version: ProofVersion::V1,
            key_hash,
            is_inclusion: true,
            value_hash: Some(value_hash),
            leaf_version: Some(version),
            absence_reason: None,
            steps,
        })
    }

    pub fn new_absence(
        key_hash: [u8; 32],
        reason: AbsenceReason,
        steps: Vec<ProofStep>,
    ) -> Result<Self, SmtError> {
        if steps.len() > MAX_PROOF_STEPS {
            return Err(SmtError::ProofTooLarge {
                steps: steps.len(),
                max: MAX_PROOF_STEPS,
            });
        }
        Ok(Self {
            proof_version: ProofVersion::V1,
            key_hash,
            is_inclusion: false,
            value_hash: None,
            leaf_version: None,
            absence_reason: Some(reason),
            steps,
        })
    }

    pub fn prepend_step(&mut self, step: ProofStep) {
        self.steps.insert(0, step);
    }

    // ---- Verification ----

    /// Verify the proof against `state_root`.
    /// `state_root` is `H(AMUN_ROOT_V1 || internal_root_hash)`.
    pub fn verify(&self, state_root: &Hash) -> Result<bool, SmtError> {
        if self.steps.len() > MAX_PROOF_STEPS {
            return Err(SmtError::ProofTooLarge {
                steps: self.steps.len(),
                max: MAX_PROOF_STEPS,
            });
        }

        // Determine starting hash based on proof type
        let mut current = if self.is_inclusion {
            let vh = self.value_hash.ok_or(SmtError::MissingLeafWitness)?;
            let ver = self.leaf_version.ok_or(SmtError::MissingLeafWitness)?;
            crate::hash::hash_leaf(&self.key_hash, &vh, ver)
        } else {
            match &self.absence_reason {
                Some(AbsenceReason::LeafDivergence {
                    leaf_witness,
                    divergence_depth,
                }) => {
                    let div =
                        crate::hash::find_divergence(&self.key_hash, &leaf_witness.key_hash);
                    if div != *divergence_depth as usize {
                        return Ok(false);
                    }
                    crate::hash::hash_leaf(
                        &leaf_witness.key_hash,
                        &leaf_witness.value_hash,
                        leaf_witness.version,
                    )
                }
                Some(AbsenceReason::EmptyTree) if self.steps.is_empty() => {
                    return Ok(crate::hash::hash_root(&EMPTY_NODE_HASH) == *state_root);
                }
                _ => *EMPTY_NODE_HASH,
            }
        };

        let mut depth = 0usize;

        for (idx, step) in self.steps.iter().enumerate() {
            if depth > 255 {
                return Ok(false);
            }
            let skip = step.skip_len as usize;
            let (ref prefix_bytes, bit_len) = step.prefix_bits;
            if bit_len != step.skip_len {
                return Ok(false);
            }
            if depth + skip >= 256 {
                return Ok(false);
            }

            // Determine child/sibling orientation
            let (left, right) = if !self.is_inclusion {
                if let Some(AbsenceReason::PrefixMismatch { mismatched_bit }) = &self.absence_reason
                {
                    let is_last = idx == self.steps.len() - 1;
                    if is_last {
                        let mm = *mismatched_bit as usize;
                        let mismatch_depth = depth + mm;
                        if mismatch_depth >= 256 {
                            return Ok(false);
                        }
                        let mismatch_bit = crate::hash::bit(&self.key_hash, mismatch_depth);
                        if mismatch_bit == 0 {
                            if step.is_left_sibling {
                                return Ok(false);
                            }
                            (current, step.sibling)
                        } else {
                            if !step.is_left_sibling {
                                return Ok(false);
                            }
                            (step.sibling, current)
                        }
                    } else {
                        return Ok(false);
                    }
                } else {
                    let child_bit = crate::hash::bit(&self.key_hash, depth + skip);
                    if child_bit == 0 {
                        if step.is_left_sibling {
                            return Ok(false);
                        }
                        (current, step.sibling)
                    } else {
                        if !step.is_left_sibling {
                            return Ok(false);
                        }
                        (step.sibling, current)
                    }
                }
            } else {
                let child_bit = crate::hash::bit(&self.key_hash, depth + skip);
                if child_bit == 0 {
                    if step.is_left_sibling {
                        return Ok(false);
                    }
                    (current, step.sibling)
                } else {
                    if !step.is_left_sibling {
                        return Ok(false);
                    }
                    (step.sibling, current)
                }
            };

            // Reconstruct prefix array with canonical masking
            let mut prefix_arr = [0u8; 32];
            let bl = prefix_bytes.len().min(32);
            prefix_arr[..bl].copy_from_slice(&prefix_bytes[..bl]);
            crate::hash::canonicalize_prefix(&mut prefix_arr, step.skip_len);

            let branch_hash =
                crate::hash::hash_branch(step.skip_len, &prefix_arr, &left, &right);

            if self.is_inclusion {
                if !prefix_matches(&self.key_hash, depth, step.skip_len, &prefix_arr) {
                    return Ok(false);
                }
                current = branch_hash;
                depth += skip + 1;
                if idx == self.steps.len() - 1 {
                    return Ok(crate::hash::hash_root(&current) == *state_root);
                }
            } else {
                let is_last = idx == self.steps.len() - 1;
                match self.absence_reason.as_ref() {
                    Some(AbsenceReason::PrefixMismatch { mismatched_bit }) => {
                        if !is_last {
                            return Ok(false);
                        }
                        let mm = *mismatched_bit as usize;
                        if mm as u8 >= step.skip_len {
                            return Ok(false);
                        }
                        // Verify all bits before mismatch match
                        for i in 0..mm {
                            let kb = crate::hash::bit(&self.key_hash, depth + i);
                            let pb = (prefix_arr[i / 8] >> (7 - (i % 8))) & 1;
                            if kb != pb {
                                return Ok(false);
                            }
                        }
                        // Verify the mismatched bit differs
                        let kb = crate::hash::bit(&self.key_hash, depth + mm);
                        let pb = (prefix_arr[mm / 8] >> (7 - (mm % 8))) & 1;
                        if kb == pb {
                            return Ok(false);
                        }
                        current = branch_hash;
                        // PrefixMismatch: no edge traversed, depth unchanged
                        break;
                    }
                    Some(AbsenceReason::EmptyChild) => {
                        if !is_last {
                            return Ok(false);
                        }
                        if !prefix_matches(&self.key_hash, depth, step.skip_len, &prefix_arr) {
                            return Ok(false);
                        }
                        let child_bit = crate::hash::bit(&self.key_hash, depth + skip);
                        let child_hash = if child_bit == 0 { left } else { right };
                        if child_hash != *EMPTY_NODE_HASH {
                            return Ok(false);
                        }
                        current = branch_hash;
                        depth += skip + 1;
                        break;
                    }
                    Some(AbsenceReason::LeafDivergence { .. }) => {
                        if !prefix_matches(&self.key_hash, depth, step.skip_len, &prefix_arr) {
                            return Ok(false);
                        }
                        current = branch_hash;
                        depth += skip + 1;
                    }
                    Some(AbsenceReason::EmptyTree) => return Ok(false),
                    None => return Ok(false),
                }
            }
        }

        if depth > 255 {
            return Ok(false);
        }
        match &self.absence_reason {
            Some(AbsenceReason::EmptyTree) => {
                Ok(crate::hash::hash_root(&EMPTY_NODE_HASH) == *state_root
                    && self.steps.is_empty())
            }
            _ if self.is_inclusion => Ok(crate::hash::hash_root(&current) == *state_root),
            _ => Ok(crate::hash::hash_root(&current) == *state_root),
        }
    }
}

fn prefix_matches(key: &[u8; 32], depth: usize, skip_len: u8, prefix: &[u8; 32]) -> bool {
    for i in 0..skip_len as usize {
        let kb = crate::hash::bit(key, depth + i);
        let pb = (prefix[i / 8] >> (7 - (i % 8))) & 1;
        if kb != pb {
            return false;
        }
    }
    true
}
