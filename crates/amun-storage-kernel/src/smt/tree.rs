use super::{Node, NodeHash};
use crate::Key256;
use crate::StateRoot;
use im::OrdMap;
use std::sync::Arc;

type NodeMap = OrdMap<NodeHash, Arc<Node>>;
pub const MAX_DEPTH: usize = 256;

pub struct SparseMerkleTree {
    pub nodes: Arc<NodeMap>,
    pub root: NodeHash,
    pub empty_ladder: Arc<Vec<NodeHash>>,
}

pub struct WitnessLeaf {
    pub key_hash: [u8; 32],
    pub value_hash: [u8; 32],
    pub version: u64,
}

impl SparseMerkleTree {
    fn build_empty_ladder() -> Vec<NodeHash> {
        let mut ladder = vec![NodeHash::ZERO; 257];
        for d in (0..256).rev() {
            let empty_node = Node::Branch {
                left: ladder[d + 1],
                right: ladder[d + 1],
            };
            ladder[d] = empty_node.hash();
        }
        ladder
    }

    pub fn empty() -> Self {
        let empty_ladder = Arc::new(Self::build_empty_ladder());
        Self {
            nodes: Arc::new(OrdMap::new()),
            root: empty_ladder[0],
            empty_ladder,
        }
    }

    pub fn canonical_empty_root() -> [u8; 32] {
        Self::empty().root.0
    }
    pub fn root(&self) -> StateRoot {
        StateRoot(self.root.0)
    }
    pub fn root_node(&self) -> NodeHash {
        self.root
    }

    fn direction(&self, key_hash: &[u8; 32], depth: usize) -> u8 {
        (key_hash[depth / 8] >> (7 - (depth % 8))) & 1
    }

    fn store_node(node: &Node, new_nodes: &mut NodeMap) -> NodeHash {
        let bh = node.hash();
        #[cfg(debug_assertions)]
        if let Some(existing) = new_nodes.get(&bh) {
            if **existing != *node {
                panic!("CONSTITUTIONAL CRISIS: NodeHash collision");
            }
        }
        new_nodes.insert(bh, Arc::new(node.clone()));
        bh
    }

    pub fn insert(&self, key: &Key256, value_hash: &[u8; 32], version: u64) -> Self {
        let key_hash: [u8; 32] = blake3::hash(&key.0).into();
        let mut new_nodes = OrdMap::new();
        let leaf = Node::Leaf {
            key_hash,
            value_hash: *value_hash,
            version,
        };
        let leaf_hash = Self::store_node(&leaf, &mut new_nodes);
        let new_root = self.insert_at(0, self.root, &key_hash, leaf_hash, &mut new_nodes);
        let merged = OrdMap::clone(&self.nodes).union(new_nodes);
        Self {
            nodes: Arc::new(merged),
            root: new_root,
            empty_ladder: self.empty_ladder.clone(),
        }
    }

    fn insert_at(
        &self,
        depth: usize,
        current: NodeHash,
        key_hash: &[u8; 32],
        leaf_hash: NodeHash,
        new_nodes: &mut NodeMap,
    ) -> NodeHash {
        if depth >= MAX_DEPTH {
            return leaf_hash;
        }
        if current == self.empty_ladder[depth] {
            let bit = self.direction(key_hash, depth);
            let child = self.insert_at(
                depth + 1,
                self.empty_ladder[depth + 1],
                key_hash,
                leaf_hash,
                new_nodes,
            );
            let (left, right) = if bit == 0 {
                (child, self.empty_ladder[depth + 1])
            } else {
                (self.empty_ladder[depth + 1], child)
            };
            return Self::store_node(&Node::Branch { left, right }, new_nodes);
        }
        if let Some(node) = self.nodes.get(&current).cloned() {
            match &*node {
                Node::Leaf {
                    key_hash: existing_key,
                    ..
                } => {
                    if existing_key == key_hash {
                        return leaf_hash;
                    }
                    let div = self.find_divergence(depth, key_hash, existing_key);
                    if div >= MAX_DEPTH {
                        panic!("CONSTITUTIONAL CRISIS: hash collision");
                    }
                    return self.build_canonical_branch(
                        depth,
                        div,
                        key_hash,
                        leaf_hash,
                        existing_key,
                        current,
                        new_nodes,
                    );
                }
                Node::Branch { left, right } => {
                    let bit = self.direction(key_hash, depth);
                    let child = self.insert_at(
                        depth + 1,
                        if bit == 0 { *left } else { *right },
                        key_hash,
                        leaf_hash,
                        new_nodes,
                    );
                    let (new_left, new_right) = if bit == 0 {
                        (child, *right)
                    } else {
                        (*left, child)
                    };
                    return Self::store_node(
                        &Node::Branch {
                            left: new_left,
                            right: new_right,
                        },
                        new_nodes,
                    );
                }
            }
        }
        self.empty_ladder[depth]
    }

    fn find_divergence(&self, start: usize, a: &[u8; 32], b: &[u8; 32]) -> usize {
        (start..MAX_DEPTH)
            .find(|&d| self.direction(a, d) != self.direction(b, d))
            .unwrap_or(MAX_DEPTH)
    }

    fn build_canonical_branch(
        &self,
        depth: usize,
        divergence: usize,
        key_a: &[u8; 32],
        hash_a: NodeHash,
        key_b: &[u8; 32],
        hash_b: NodeHash,
        new_nodes: &mut NodeMap,
    ) -> NodeHash {
        if depth == divergence {
            let bit_a = self.direction(key_a, depth);
            let bit_b = self.direction(key_b, depth);
            let (left_hash, right_hash) = if bit_a < bit_b {
                (hash_a, hash_b)
            } else {
                (hash_b, hash_a)
            };
            return Self::store_node(
                &Node::Branch {
                    left: left_hash,
                    right: right_hash,
                },
                new_nodes,
            );
        }
        let bit = self.direction(key_a, depth);
        let child = self.build_canonical_branch(
            depth + 1,
            divergence,
            key_a,
            hash_a,
            key_b,
            hash_b,
            new_nodes,
        );
        let (left, right) = if bit == 0 {
            (child, self.empty_ladder[depth + 1])
        } else {
            (self.empty_ladder[depth + 1], child)
        };
        Self::store_node(&Node::Branch { left, right }, new_nodes)
    }

    pub fn delete(&self, key: &Key256) -> Self {
        let key_hash: [u8; 32] = blake3::hash(&key.0).into();
        let mut new_nodes = OrdMap::new();
        let new_root = self.delete_at(0, self.root, &key_hash, &mut new_nodes);
        let merged = OrdMap::clone(&self.nodes).union(new_nodes);
        Self {
            nodes: Arc::new(merged),
            root: new_root,
            empty_ladder: self.empty_ladder.clone(),
        }
    }

    fn delete_at(
        &self,
        depth: usize,
        current: NodeHash,
        key_hash: &[u8; 32],
        new_nodes: &mut NodeMap,
    ) -> NodeHash {
        if depth >= MAX_DEPTH {
            return self.empty_ladder[MAX_DEPTH];
        }
        if current == self.empty_ladder[depth] {
            return current;
        }
        if let Some(node) = self.nodes.get(&current).cloned() {
            match &*node {
                Node::Leaf {
                    key_hash: existing, ..
                } if existing == key_hash => {
                    return self.empty_ladder[depth];
                }
                Node::Branch { left, right } => {
                    let bit = self.direction(key_hash, depth);
                    let (new_left, new_right) = if bit == 0 {
                        (
                            self.delete_at(depth + 1, *left, key_hash, new_nodes),
                            *right,
                        )
                    } else {
                        (
                            *left,
                            self.delete_at(depth + 1, *right, key_hash, new_nodes),
                        )
                    };
                    if new_left == self.empty_ladder[depth + 1]
                        && new_right == self.empty_ladder[depth + 1]
                    {
                        return self.empty_ladder[depth];
                    }
                    return Self::store_node(
                        &Node::Branch {
                            left: new_left,
                            right: new_right,
                        },
                        new_nodes,
                    );
                }
                _ => return current,
            }
        }
        current
    }

    pub fn get(&self, key: &Key256) -> Option<([u8; 32], u64)> {
        let key_hash: [u8; 32] = blake3::hash(&key.0).into();
        self.get_at(0, self.root, &key_hash)
    }

    fn get_at(
        &self,
        depth: usize,
        current: NodeHash,
        key_hash: &[u8; 32],
    ) -> Option<([u8; 32], u64)> {
        if current == self.empty_ladder[depth] {
            return None;
        }
        if depth >= MAX_DEPTH {
            return None;
        }
        if let Some(node) = self.nodes.get(&current) {
            match &**node {
                Node::Leaf {
                    key_hash: k,
                    value_hash,
                    version,
                } if k == key_hash => Some((*value_hash, *version)),
                Node::Branch { left, right } => {
                    let bit = self.direction(key_hash, depth);
                    self.get_at(depth + 1, if bit == 0 { *left } else { *right }, key_hash)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    // ============================================================
    // Inclusion Proofs
    // ============================================================
    pub fn generate_inclusion_proof(&self, key: &Key256) -> Option<super::proof::MerkleProof> {
        let key_hash: [u8; 32] = blake3::hash(&key.0).into();
        let mut steps = Vec::with_capacity(MAX_DEPTH);
        let (value_hash, version) =
            self.collect_inclusion_steps(0, self.root, &key_hash, &mut steps)?;
        if steps.len() != MAX_DEPTH {
            return None;
        }
        steps.reverse();
        Some(super::proof::MerkleProof::new_inclusion(
            key_hash, value_hash, version, steps,
        ))
    }

    fn collect_inclusion_steps(
        &self,
        depth: usize,
        current: NodeHash,
        key_hash: &[u8; 32],
        steps: &mut Vec<super::proof::ProofStep>,
    ) -> Option<([u8; 32], u64)> {
        if depth >= MAX_DEPTH {
            if let Some(node) = self.nodes.get(&current) {
                if let Node::Leaf {
                    key_hash: k,
                    value_hash,
                    version,
                } = &**node
                {
                    if k == key_hash {
                        return Some((*value_hash, *version));
                    }
                }
            }
            return None;
        }
        if current == self.empty_ladder[depth] {
            return None;
        }
        if let Some(node) = self.nodes.get(&current) {
            match &**node {
                Node::Leaf {
                    key_hash: k,
                    value_hash,
                    version,
                } if k == key_hash => {
                    for d in depth..MAX_DEPTH {
                        let bit = self.direction(key_hash, d);
                        if bit == 0 {
                            steps.push(super::proof::ProofStep::SiblingOnRight(
                                self.empty_ladder[d + 1],
                            ));
                        } else {
                            steps.push(super::proof::ProofStep::SiblingOnLeft(
                                self.empty_ladder[d + 1],
                            ));
                        }
                    }
                    Some((*value_hash, *version))
                }
                Node::Branch { left, right } => {
                    let bit = self.direction(key_hash, depth);
                    if bit == 0 {
                        steps.push(super::proof::ProofStep::SiblingOnRight(*right));
                        self.collect_inclusion_steps(depth + 1, *left, key_hash, steps)
                    } else {
                        steps.push(super::proof::ProofStep::SiblingOnLeft(*left));
                        self.collect_inclusion_steps(depth + 1, *right, key_hash, steps)
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    // ============================================================
    // Absence Proofs
    // ============================================================
    pub fn generate_absence_proof(&self, key: &Key256) -> Option<super::proof::MerkleProof> {
        let key_hash: [u8; 32] = blake3::hash(&key.0).into();
        let witness = self.find_absence_witness(0, self.root, &key_hash);
        match witness {
            None => {
                if self.root == self.empty_ladder[0] {
                    let mut steps = Vec::with_capacity(MAX_DEPTH);
                    for d in 0..MAX_DEPTH {
                        let bit = self.direction(&key_hash, d);
                        if bit == 0 {
                            steps.push(super::proof::ProofStep::SiblingOnRight(
                                self.empty_ladder[d + 1],
                            ));
                        } else {
                            steps.push(super::proof::ProofStep::SiblingOnLeft(
                                self.empty_ladder[d + 1],
                            ));
                        }
                    }
                    if steps.len() != MAX_DEPTH {
                        return None;
                    }
                    steps.reverse();
                    Some(super::proof::MerkleProof::new_empty_tree_absence(
                        key_hash, steps,
                    ))
                } else {
                    None
                }
            }
            Some(witness_leaf) => {
                let mut steps = Vec::with_capacity(MAX_DEPTH);
                let collected =
                    self.collect_inclusion_steps(0, self.root, &witness_leaf.key_hash, &mut steps);
                if steps.len() != MAX_DEPTH {
                    return None;
                }
                steps.reverse();

                if let Some((val_hash, ver)) = collected {
                    if val_hash == witness_leaf.value_hash && ver == witness_leaf.version {
                        let div = (0..MAX_DEPTH)
                            .find(|&d| {
                                self.direction(&key_hash, d)
                                    != self.direction(&witness_leaf.key_hash, d)
                            })
                            .unwrap_or(MAX_DEPTH);
                        return Some(super::proof::MerkleProof::new_absence(
                            key_hash,
                            div,
                            witness_leaf.key_hash,
                            witness_leaf.value_hash,
                            witness_leaf.version,
                            steps,
                        ));
                    }
                }
                None
            }
        }
    }

    fn find_absence_witness(
        &self,
        depth: usize,
        current: NodeHash,
        key_hash: &[u8; 32],
    ) -> Option<WitnessLeaf> {
        if depth >= MAX_DEPTH {
            if let Some(node) = self.nodes.get(&current) {
                if let Node::Leaf {
                    key_hash: leaf_key,
                    value_hash,
                    version,
                } = &**node
                {
                    if leaf_key == key_hash {
                        return None;
                    }
                    return Some(WitnessLeaf {
                        key_hash: *leaf_key,
                        value_hash: *value_hash,
                        version: *version,
                    });
                }
            }
            return None;
        }
        if current == self.empty_ladder[depth] {
            return None;
        }
        if let Some(node) = self.nodes.get(&current) {
            match &**node {
                Node::Leaf {
                    key_hash: leaf_key,
                    value_hash,
                    version,
                } => {
                    if leaf_key == key_hash {
                        return None;
                    }
                    Some(WitnessLeaf {
                        key_hash: *leaf_key,
                        value_hash: *value_hash,
                        version: *version,
                    })
                }
                Node::Branch { left, right } => {
                    let bit = self.direction(key_hash, depth);
                    if bit == 0 {
                        if let Some(witness) = self.find_absence_witness(depth + 1, *left, key_hash)
                        {
                            return Some(witness);
                        }
                        self.find_any_leaf(depth + 1, *right)
                    } else {
                        if let Some(witness) =
                            self.find_absence_witness(depth + 1, *right, key_hash)
                        {
                            return Some(witness);
                        }
                        self.find_any_leaf(depth + 1, *left)
                    }
                }
            }
        } else {
            None
        }
    }

    fn find_any_leaf(&self, depth: usize, current: NodeHash) -> Option<WitnessLeaf> {
        if depth >= MAX_DEPTH {
            if let Some(node) = self.nodes.get(&current) {
                if let Node::Leaf {
                    key_hash,
                    value_hash,
                    version,
                } = &**node
                {
                    return Some(WitnessLeaf {
                        key_hash: *key_hash,
                        value_hash: *value_hash,
                        version: *version,
                    });
                }
            }
            return None;
        }
        if current == self.empty_ladder[depth] {
            return None;
        }
        if let Some(node) = self.nodes.get(&current) {
            match &**node {
                Node::Leaf {
                    key_hash,
                    value_hash,
                    version,
                } => Some(WitnessLeaf {
                    key_hash: *key_hash,
                    value_hash: *value_hash,
                    version: *version,
                }),
                Node::Branch { left, right } => {
                    if *left != self.empty_ladder[depth + 1] {
                        return self.find_any_leaf(depth + 1, *left);
                    }
                    self.find_any_leaf(depth + 1, *right)
                }
            }
        } else {
            None
        }
    }
}
