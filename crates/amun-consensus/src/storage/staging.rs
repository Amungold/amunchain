use std::collections::BTreeMap;
use crate::state_tree::{Key256, ValueBlob, StateRoot};
use crate::storage::smt::SparseMerkleTree;

pub struct StagingArea {
    writes: BTreeMap<Key256, ValueBlob>,
    deletes: BTreeMap<Key256, ()>,
}

impl StagingArea {
    pub fn new() -> Self { Self { writes: BTreeMap::new(), deletes: BTreeMap::new() } }
    pub fn write(&mut self, key: Key256, value: ValueBlob) { self.writes.insert(key, value); }
    pub fn delete(&mut self, key: Key256) { self.deletes.insert(key, ()); }
    pub fn is_empty(&self) -> bool { self.writes.is_empty() && self.deletes.is_empty() }
    pub fn apply(self, tree: &SparseMerkleTree) -> Result<(SparseMerkleTree, StateRoot), &'static str> {
        let mut current = tree.clone();
        for (k, v) in self.writes {
            let (new_tree, _) = current.insert(k, v)?;
            current = new_tree;
        }
        Ok((current, current.root()))
    }
}
