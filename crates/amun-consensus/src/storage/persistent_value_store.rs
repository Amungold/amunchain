use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::state_tree::{ValueKey, ValueBlob};
use crate::storage::atomic_file::atomic_write;
use crate::storage::lru::LruCache;
use crate::storage::constants::LRU_CACHE_SIZE;

pub struct PersistentValueStore {
    root_dir: String,
    cache: RwLock<LruCache<ValueKey, ValueBlob>>,
}

impl PersistentValueStore {
    pub fn new(root_dir: &str) -> Self {
        fs::create_dir_all(root_dir).ok();
        Self {
            root_dir: root_dir.to_string(),
            cache: RwLock::new(LruCache::new(LRU_CACHE_SIZE)),
        }
    }
    
    fn path_for(&self, key: &ValueKey) -> PathBuf {
        let hex = hex::encode(key.hash);
        let dir = &hex[0..2];
        let subdir = &hex[2..4];
        let file = &hex[4..];
        let full_dir = format!("{}/{}/{}", self.root_dir, dir, subdir);
        fs::create_dir_all(&full_dir).ok();
        PathBuf::from(format!("{}/{}.val", full_dir, file))
    }
    
    pub fn get(&self, key: &ValueKey) -> Option<ValueBlob> {
        if let Some(v) = self.cache.write().unwrap().get(key) {
            return Some(v.clone());
        }
        let path = self.path_for(key);
        let data = fs::read(&path).ok()?;
        let mut dec = CCBFDecoder::new(&data);
        let val = ValueBlob::decode(&mut dec)?;
        self.cache.write().unwrap().put(*key, val.clone());
        Some(val)
    }
    
    pub fn put(&self, key: ValueKey, value: ValueBlob) -> Result<(), &'static str> {
        let mut enc = CCBFEncoder::new();
        value.encode(&mut enc);
        let bytes = enc.into_bytes();
        let path = self.path_for(&key);
        atomic_write(path.to_str().unwrap(), &bytes).map_err(|_| "Value write failed")?;
        self.cache.write().unwrap().put(key, value);
        Ok(())
    }
}
