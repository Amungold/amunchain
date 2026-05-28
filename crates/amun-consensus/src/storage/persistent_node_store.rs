use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::state_tree::{NodeHash, Node};
use crate::storage::codec::NodeCodec;
use crate::storage::atomic_file::atomic_write;
use crate::storage::lru::LruCache;
use crate::storage::constants::LRU_CACHE_SIZE;

pub struct PersistentNodeStore {
    root_dir: String,
    cache: RwLock<LruCache<NodeHash, Node>>,
}

impl PersistentNodeStore {
    pub fn new(root_dir: &str) -> Self {
        fs::create_dir_all(root_dir).ok();
        Self {
            root_dir: root_dir.to_string(),
            cache: RwLock::new(LruCache::new(LRU_CACHE_SIZE)),
        }
    }
    
    fn path_for(&self, hash: &NodeHash) -> PathBuf {
        let hex = hex::encode(hash.0);
        let dir = &hex[0..2];
        let subdir = &hex[2..4];
        let file = &hex[4..];
        let full_dir = format!("{}/{}/{}", self.root_dir, dir, subdir);
        fs::create_dir_all(&full_dir).ok();
        PathBuf::from(format!("{}/{}.node", full_dir, file))
    }
    
    pub fn get(&self, hash: &NodeHash) -> Option<Node> {
        if let Some(node) = self.cache.write().unwrap().get(hash) {
            return Some(node.clone());
        }
        let path = self.path_for(hash);
        let data = fs::read(&path).ok()?;
        let mut dec = CCBFDecoder::new(&data);
        let node = NodeCodec::decode(&mut dec)?;
        self.cache.write().unwrap().put(*hash, node.clone());
        Some(node)
    }
    
    pub fn put(&self, hash: NodeHash, node: Node) -> Result<(), &'static str> {
        let mut enc = CCBFEncoder::new();
        NodeCodec::encode(&node, &mut enc);
        let bytes = enc.into_bytes();
        let path = self.path_for(&hash);
        atomic_write(path.to_str().unwrap(), &bytes).map_err(|_| "Node write failed")?;
        self.cache.write().unwrap().put(hash, node);
        Ok(())
    }
}
