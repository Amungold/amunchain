// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProofType {
    Inclusion,
    Exclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MerkleProof {
    pub siblings: Vec<[u8; 32]>,
    pub directions: Vec<u8>,
    pub proof_type: ProofType,
    pub leaf_key_hash: [u8; 32],
    pub leaf_value: Option<[u8; 32]>,
}

/// A canonical sparse Merkle tree with fixed depth 256.
/// Proofs are always exactly 256 siblings long, produced by full-depth
/// recursion even for empty subtrees (exclusion proofs).
pub struct SparseMerkleTree {
    domain: Vec<u8>,
    leaves: BTreeMap<[u8; 32], [u8; 32]>,
    sorted_keys: Vec<[u8; 32]>,
    defaults: Vec<[u8; 32]>, // 257 entries: 0..=256
}

impl SparseMerkleTree {
    pub fn new(domain_separator: &[u8]) -> Self {
        let domain = domain_separator.to_vec();
        let mut defaults = vec![[0u8; 32]; 257];
        defaults[256] = Self::hash_leaf(&domain, &[0u8; 32], &[0u8; 32]);
        for d in (0..256).rev() {
            defaults[d] = Self::hash_internal(&domain, &defaults[d + 1], &defaults[d + 1]);
        }
        Self {
            domain,
            leaves: BTreeMap::new(),
            sorted_keys: Vec::new(),
            defaults,
        }
    }

    pub fn insert(&mut self, logical_key: &[u8], value: &[u8; 32]) -> [u8; 32] {
        let kh = Self::hash_key(b"AMUN_SMT_KEY", logical_key);
        if self.leaves.insert(kh, *value).is_none() {
            let pos = self.sorted_keys.binary_search(&kh).unwrap_err();
            self.sorted_keys.insert(pos, kh);
        } else {
            self.leaves.insert(kh, *value);
        }
        self.root()
    }

    pub fn root(&self) -> [u8; 32] {
        self.build(0, &self.sorted_keys)
    }

    pub fn prove(&self, logical_key: &[u8]) -> MerkleProof {
        let kh = Self::hash_key(b"AMUN_SMT_KEY", logical_key);
        let (ptype, val) = if self.leaves.contains_key(&kh) {
            (ProofType::Inclusion, Some(self.leaves[&kh]))
        } else {
            (ProofType::Exclusion, None)
        };

        let mut siblings = Vec::with_capacity(256);
        let mut directions = Vec::with_capacity(256);
        self.prove_inner(0, &self.sorted_keys, &kh, &mut siblings, &mut directions);

        MerkleProof {
            siblings,
            directions,
            proof_type: ptype,
            leaf_key_hash: kh,
            leaf_value: val,
        }
    }

    pub fn verify(&self, root: &[u8; 32], proof: &MerkleProof) -> bool {
        let leaf = match proof.leaf_value {
            Some(ref v) => Self::hash_leaf(&self.domain, &proof.leaf_key_hash, v),
            None => self.defaults[256],
        };

        let mut current = leaf;
        for i in (0..proof.siblings.len()).rev() {
            let sib = &proof.siblings[i];
            let dir = proof.directions[i];
            let (left, right) = if dir == 0 {
                (&current, sib)
            } else {
                (sib, &current)
            };
            current = Self::hash_internal(&self.domain, left, right);
        }
        &current == root
    }

    // ----------------------------------------------------------------
    fn hash_key(dom: &[u8], key: &[u8]) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(dom);
        h.update(key);
        *h.finalize().as_bytes()
    }
    fn hash_leaf(dom: &[u8], k: &[u8; 32], v: &[u8; 32]) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(dom);
        h.update(b"leaf");
        h.update(k);
        h.update(v);
        *h.finalize().as_bytes()
    }
    fn hash_internal(dom: &[u8], l: &[u8; 32], r: &[u8; 32]) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(dom);
        h.update(b"internal");
        h.update(l);
        h.update(r);
        *h.finalize().as_bytes()
    }
    fn bit_at(key: &[u8; 32], depth: usize) -> u8 {
        (key[depth / 8] >> (7 - (depth % 8))) & 1
    }

    fn build(&self, depth: usize, keys: &[[u8; 32]]) -> [u8; 32] {
        if keys.is_empty() {
            return self.defaults[depth];
        }
        if depth == 256 {
            let k = keys[0];
            let v = self.leaves.get(&k).unwrap_or(&[0u8; 32]);
            return Self::hash_leaf(&self.domain, &k, v);
        }
        let split = keys.partition_point(|k| Self::bit_at(k, depth) == 0);
        let left = self.build(depth + 1, &keys[..split]);
        let right = self.build(depth + 1, &keys[split..]);
        Self::hash_internal(&self.domain, &left, &right)
    }

    fn prove_inner(
        &self,
        depth: usize,
        keys: &[[u8; 32]],
        target: &[u8; 32],
        siblings: &mut Vec<[u8; 32]>,
        directions: &mut Vec<u8>,
    ) {
        if depth == 256 {
            return;
        }
        if keys.is_empty() {
            let bit = Self::bit_at(target, depth);
            siblings.push(self.defaults[depth + 1]);
            directions.push(bit);
            self.prove_inner(depth + 1, &[], target, siblings, directions);
            return;
        }

        let bit = Self::bit_at(target, depth);
        let split = keys.partition_point(|k| Self::bit_at(k, depth) == 0);

        if bit == 0 {
            let sibling = self.build(depth + 1, &keys[split..]);
            siblings.push(sibling);
            directions.push(bit);
            self.prove_inner(depth + 1, &keys[..split], target, siblings, directions);
        } else {
            let sibling = self.build(depth + 1, &keys[..split]);
            siblings.push(sibling);
            directions.push(bit);
            self.prove_inner(depth + 1, &keys[split..], target, siblings, directions);
        }
    }
}
