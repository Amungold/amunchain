use std::sync::Arc;
use crate::storage::persistent_node_store::PersistentNodeStore;
use crate::storage::persistent_value_store::PersistentValueStore;
use crate::state_tree::{Key256, ValueBlob, Node, NodeHash, ValueKey, StateRoot};

pub struct SparseMerkleTree {
    node_store: Arc<PersistentNodeStore>,
    value_store: Arc<PersistentValueStore>,
    root_hash: NodeHash,
    empty_hashes: Vec<NodeHash>,
}

impl SparseMerkleTree {
    pub const DEPTH: usize = 256;
    
    pub fn new(node_store: Arc<PersistentNodeStore>, value_store: Arc<PersistentValueStore>) -> Self {
        let mut empty_hashes = vec![NodeHash::ZERO; Self::DEPTH + 1];
        empty_hashes[Self::DEPTH] = NodeHash::ZERO;
        for d in (0..Self::DEPTH).rev() {
            let empty_node = Node::Branch { left: empty_hashes[d+1], right: empty_hashes[d+1] };
            let h = empty_node.hash();
            empty_hashes[d] = h;
            node_store.put(h, empty_node).ok();
        }
        let root_hash = empty_hashes[0];
        Self { node_store, value_store, root_hash, empty_hashes }
    }
    
    pub fn root(&self) -> StateRoot { StateRoot(self.root_hash.0) }
    
    pub fn insert(&self, key: Key256, value: ValueBlob) -> Result<(Self, StateRoot), &'static str> {
        let value_key = value.key();
        self.value_store.put(value_key, value)?;
        let new_root = self.insert_at_depth(0, self.root_hash, key, value_key)?;
        let mut new_tree = Self {
            node_store: self.node_store.clone(),
            value_store: self.value_store.clone(),
            root_hash: new_root,
            empty_hashes: self.empty_hashes.clone(),
        };
        Ok((new_tree, StateRoot(new_root.0)))
    }
    
    fn insert_at_depth(&self, depth: usize, hash: NodeHash, key: Key256, value_key: ValueKey) -> Result<NodeHash, &'static str> {
        if depth == Self::DEPTH {
            let leaf = Node::Leaf { key: key.0, value_key };
            let lh = leaf.hash();
            self.node_store.put(lh, leaf)?;
            return Ok(lh);
        }
        
        let current = self.node_store.get(&hash);
        if hash == self.empty_hashes[depth] {
            let leaf = Node::Leaf { key: key.0, value_key };
            let lh = leaf.hash();
            self.node_store.put(lh, leaf)?;
            let bit = key.bit(depth);
            let (left, right) = if bit == 0 { (lh, self.empty_hashes[depth+1]) } else { (self.empty_hashes[depth+1], lh) };
            let branch = Node::Branch { left, right };
            let bh = branch.hash();
            self.node_store.put(bh, branch)?;
            return Ok(bh);
        }
        
        match current {
            Some(Node::Leaf { key: ek, value_key: vk }) if ek == key.0 => {
                let leaf = Node::Leaf { key: key.0, value_key };
                let lh = leaf.hash();
                self.node_store.put(lh, leaf)?;
                Ok(lh)
            }
            Some(Node::Leaf { key: ek, value_key: vk }) => {
                let divergence = self.find_divergence(&Key256(ek), &key);
                self.replace_with_branch(depth, hash, key, value_key, Key256(ek), vk, divergence)
            }
            Some(Node::Branch { left, right }) => {
                let bit = key.bit(depth);
                let new_child = if bit == 0 {
                    self.insert_at_depth(depth+1, left, key, value_key)?
                } else {
                    self.insert_at_depth(depth+1, right, key, value_key)?
                };
                let (new_left, new_right) = if bit == 0 { (new_child, right) } else { (left, new_child) };
                let branch = Node::Branch { left: new_left, right: new_right };
                let bh = branch.hash();
                self.node_store.put(bh, branch)?;
                Ok(bh)
            }
            _ => Err("Invalid node state"),
        }
    }
    
    fn find_divergence(&self, k1: &Key256, k2: &Key256) -> usize {
        for d in 0..Self::DEPTH { if k1.bit(d) != k2.bit(d) { return d; } }
        Self::DEPTH
    }
    
    fn replace_with_branch(&self, depth: usize, hash: NodeHash, nk: Key256, nvk: ValueKey, ok: Key256, ovk: ValueKey, div: usize) -> Result<NodeHash, &'static str> {
        if depth == div {
            let bn = nk.bit(depth);
            let bo = ok.bit(depth);
            let left = if bn == 0 {
                self.insert_at_depth(depth+1, self.empty_hashes[depth+1], nk, nvk)?
            } else if bo == 0 {
                self.insert_at_depth(depth+1, self.empty_hashes[depth+1], ok, ovk)?
            } else {
                self.empty_hashes[depth+1]
            };
            let right = if bn == 1 {
                self.insert_at_depth(depth+1, self.empty_hashes[depth+1], nk, nvk)?
            } else if bo == 1 {
                self.insert_at_depth(depth+1, self.empty_hashes[depth+1], ok, ovk)?
            } else {
                self.empty_hashes[depth+1]
            };
            let branch = Node::Branch { left, right };
            let bh = branch.hash();
            self.node_store.put(bh, branch)?;
            Ok(bh)
        } else {
            let bit = nk.bit(depth);
            let child = self.replace_with_branch(depth+1, hash, nk, nvk, ok, ovk, div)?;
            let (left, right) = if bit == 0 { (child, self.empty_hashes[depth+1]) } else { (self.empty_hashes[depth+1], child) };
            let branch = Node::Branch { left, right };
            let bh = branch.hash();
            self.node_store.put(bh, branch)?;
            Ok(bh)
        }
    }
}
