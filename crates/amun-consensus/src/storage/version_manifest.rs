use std::collections::BTreeMap;
use std::fs;
use std::sync::RwLock;
use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::storage::atomic_file::atomic_write;
use crate::state_tree::StateRoot;

#[derive(Debug, Clone)]
pub struct VersionEntry {
    pub version: u64,
    pub parent: u64,
    pub state_root: StateRoot,
    pub epoch: u64,
}

pub struct VersionManifest {
    path: String,
    entries: RwLock<BTreeMap<u64, VersionEntry>>,
}

impl VersionManifest {
    pub fn new(path: &str) -> Self {
        let manifest = Self { path: path.to_string(), entries: RwLock::new(BTreeMap::new()) };
        manifest.load().ok();
        manifest
    }
    
    pub fn add(&self, entry: VersionEntry) -> Result<(), &'static str> {
        self.entries.write().unwrap().insert(entry.version, entry);
        self.persist()
    }
    
    pub fn get(&self, version: u64) -> Option<VersionEntry> {
        self.entries.read().unwrap().get(&version).cloned()
    }
    
    pub fn latest(&self) -> Option<VersionEntry> {
        self.entries.read().unwrap().values().max_by_key(|e| e.version).cloned()
    }
    
    fn persist(&self) -> Result<(), &'static str> {
        let mut enc = CCBFEncoder::new();
        let entries = self.entries.read().unwrap();
        enc.write_u64(entries.len() as u64);
        for e in entries.values() {
            enc.write_u64(e.version);
            enc.write_u64(e.parent);
            enc.write_fixed_hash(&e.state_root.0);
            enc.write_u64(e.epoch);
        }
        atomic_write(&self.path, &enc.into_bytes()).map_err(|_| "Failed to write manifest")
    }
    
    fn load(&self) -> Result<(), &'static str> {
        let data = fs::read(&self.path).map_err(|_| "Manifest not found")?;
        let mut dec = CCBFDecoder::new(&data);
        let count = dec.read_u64().ok_or("Invalid manifest")?;
        let mut entries = BTreeMap::new();
        for _ in 0..count {
            let version = dec.read_u64().ok_or("Invalid entry")?;
            let parent = dec.read_u64().ok_or("Invalid entry")?;
            let root_bytes = dec.read_fixed_hash().ok_or("Invalid entry")?;
            let epoch = dec.read_u64().ok_or("Invalid entry")?;
            entries.insert(version, VersionEntry { version, parent, state_root: StateRoot(root_bytes), epoch });
        }
        *self.entries.write().unwrap() = entries;
        Ok(())
    }
}
