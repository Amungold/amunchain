use std::collections::BTreeMap;
use crate::state_tree::StateRoot;
use crate::storage::atomic_file::atomic_write;
use crate::ccbf::{CCBFEncoder, CCBFDecoder};

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub version: u64,
    pub state_root: StateRoot,
    pub epoch: u64,
}

pub struct SnapshotEngine {
    path: String,
    snapshots: BTreeMap<u64, Snapshot>,
}

impl SnapshotEngine {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string(), snapshots: BTreeMap::new() }
    }
    
    pub fn create(&mut self, version: u64, state_root: StateRoot, epoch: u64) -> Result<(), &'static str> {
        let snap = Snapshot { version, state_root, epoch };
        self.snapshots.insert(version, snap);
        self.persist()
    }
    
    fn persist(&self) -> Result<(), &'static str> {
        let mut enc = CCBFEncoder::new();
        enc.write_u64(self.snapshots.len() as u64);
        for snap in self.snapshots.values() {
            enc.write_u64(snap.version);
            enc.write_fixed_hash(&snap.state_root.0);
            enc.write_u64(snap.epoch);
        }
        atomic_write(&format!("{}/snapshots.dat", self.path), &enc.into_bytes()).map_err(|_| "Snapshot write failed")
    }
}
