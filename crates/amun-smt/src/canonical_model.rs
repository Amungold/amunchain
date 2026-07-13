use std::collections::BTreeMap;
use crate::{Key256, SparseMerkleTree};
use crate::error::SmtError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafEntry {
    pub value_hash: [u8; 32],
    pub version: u64,
}

#[derive(Debug, Clone)]
pub struct CanonicalModel {
    pub leaves: BTreeMap<Key256, LeafEntry>,
}

impl CanonicalModel {
    pub fn new() -> Self {
        Self { leaves: BTreeMap::new() }
    }

    pub fn insert(&mut self, key: Key256, value_hash: [u8; 32], version: u64) {
        self.leaves.insert(key, LeafEntry { value_hash, version });
    }

    pub fn delete(&mut self, key: &Key256) {
        self.leaves.remove(key);
    }

    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    pub fn rebuild(&self) -> Result<SparseMerkleTree, SmtError> {
        let mut sorted: Vec<(&Key256, &LeafEntry)> = self.leaves.iter().collect();
        sorted.sort_by(|a, b| a.0.0.cmp(&b.0.0));
        
        let mut tree = SparseMerkleTree::empty();
        for (key, entry) in sorted {
            tree = tree.insert(key, &entry.value_hash, entry.version)?;
        }
        Ok(tree)
    }
}

pub fn assert_equivalent(
    incremental: &SparseMerkleTree,
    model: &CanonicalModel,
    _trace: &str,
) -> Result<(), String> {
    let canonical = model.rebuild().map_err(|e| format!("Rebuild failed: {:?}", e))?;
    
    if incremental.root() != canonical.root() {
        return Err("ROOT MISMATCH".into());
    }
    
    Ok(())
}
