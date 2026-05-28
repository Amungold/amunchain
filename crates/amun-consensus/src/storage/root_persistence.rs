use std::sync::atomic::{AtomicU64, Ordering};
use crate::state_tree::StateRoot;
use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::storage::atomic_file::atomic_write;

static ROOT_FILE: &str = "state_root.current";

pub struct RootPersistence;

impl RootPersistence {
    pub fn save(root: StateRoot) -> std::io::Result<()> {
        let mut enc = CCBFEncoder::new();
        enc.write_fixed_hash(&root.0);
        atomic_write(ROOT_FILE, &enc.into_bytes())
    }
    
    pub fn load() -> std::io::Result<StateRoot> {
        let data = std::fs::read(ROOT_FILE)?;
        let mut dec = CCBFDecoder::new(&data);
        let hash = dec.read_fixed_hash().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid root file"))?;
        Ok(StateRoot(hash))
    }
}
