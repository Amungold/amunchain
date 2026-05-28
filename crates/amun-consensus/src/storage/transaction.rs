use std::collections::BTreeMap;
use crate::state_tree::{Key256, ValueBlob, StateRoot};
use crate::storage::smt::SparseMerkleTree;
use crate::storage::wal::WALWriter;
use crate::storage::wal::codec::WALOp;
use crate::storage::root_persistence::RootPersistence;
use crate::ccbf::CCBFEncoder;

pub struct AtomicTransaction {
    tx_id: u64,
    writes: BTreeMap<Key256, ValueBlob>,
    deletes: BTreeMap<Key256, ()>,
    applied: bool,
}

impl AtomicTransaction {
    pub fn begin(tx_id: u64) -> Self {
        Self { tx_id, writes: BTreeMap::new(), deletes: BTreeMap::new(), applied: false }
    }
    
    pub fn write(&mut self, key: Key256, value: ValueBlob) {
        self.writes.insert(key, value);
    }
    
    pub fn delete(&mut self, key: Key256) {
        self.deletes.insert(key, ());
    }
    
    pub fn commit(mut self, wal: &WALWriter, tree: &SparseMerkleTree) -> Result<(SparseMerkleTree, StateRoot), &'static str> {
        if self.applied { return Err("Transaction already applied"); }
        // Write to WAL first (atomicity)
        wal.append(WALOp::Begin { tx_id: self.tx_id }).map_err(|_| "WAL begin failed")?;
        for (k, v) in &self.writes {
            let mut enc = CCBFEncoder::new();
            k.encode(&mut enc);
            let key_bytes = enc.into_bytes();
            let mut enc2 = CCBFEncoder::new();
            v.encode(&mut enc2);
            let val_bytes = enc2.into_bytes();
            wal.append(WALOp::Write { tx_id: self.tx_id, key: key_bytes, value: val_bytes }).map_err(|_| "WAL write failed")?;
        }
        // Apply to tree (immutable, returns new tree)
        let mut current_tree = tree.clone();
        for (k, v) in self.writes {
            let (new_tree, _) = current_tree.insert(k, v)?;
            current_tree = new_tree;
        }
        let new_root = current_tree.root();
        wal.append(WALOp::Commit { tx_id: self.tx_id, state_root: new_root.0 }).map_err(|_| "WAL commit failed")?;
        wal.append(WALOp::Checkpoint { state_root: new_root.0, version: self.tx_id }).map_err(|_| "WAL checkpoint failed")?;
        RootPersistence::save(new_root).map_err(|_| "Root save failed")?;
        self.applied = true;
        Ok((current_tree, new_root))
    }
    
    pub fn rollback(self, wal: &WALWriter) -> Result<(), &'static str> {
        if self.applied { return Err("Cannot rollback committed transaction"); }
        wal.append(WALOp::Abort { tx_id: self.tx_id }).map_err(|_| "WAL abort failed")?;
        Ok(())
    }
}
