use crate::canonical::{Decoder, Encoder};
use crate::lru::LruCache;
use crate::smt::{Node, NodeHash};
use parking_lot::RwLock;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct NodeStore {
    root_dir: String,
    cache: RwLock<LruCache<NodeHash, Arc<Node>>>,
}

impl NodeStore {
    pub fn new(root_dir: &str) -> Self {
        fs::create_dir_all(root_dir).unwrap();
        Self {
            root_dir: root_dir.to_string(),
            cache: RwLock::new(LruCache::new(100000)),
        }
    }

    fn path_for_hash(&self, hash: &NodeHash) -> PathBuf {
        let hex = hex::encode(hash.0);
        PathBuf::from(format!(
            "{}/{}/{}/{}.node",
            self.root_dir,
            &hex[0..2],
            &hex[2..4],
            &hex[4..]
        ))
    }

    fn ensure_parent_dirs(&self, path: &PathBuf) -> Result<PathBuf, &'static str> {
        let parent = path.parent().ok_or("no parent")?;
        fs::create_dir_all(parent).map_err(|_| "mkdir failed")?;
        Ok(parent.to_path_buf())
    }

    fn decode_node(&self, data: &[u8]) -> Option<Node> {
        let mut dec = Decoder::new(data);
        let tag = dec.read_u8()?;
        match tag {
            0x01 => {
                let key_hash = dec.read_bytes()?;
                let value_hash = dec.read_bytes()?;
                let version = dec.read_u64()?;
                if key_hash.len() != 32 || value_hash.len() != 32 {
                    return None;
                }
                let mut kh = [0u8; 32];
                let mut vh = [0u8; 32];
                kh.copy_from_slice(&key_hash);
                vh.copy_from_slice(&value_hash);
                Some(Node::Leaf {
                    key_hash: kh,
                    value_hash: vh,
                    version,
                })
            }
            0x02 => {
                let left_bytes = dec.read_bytes()?;
                let right_bytes = dec.read_bytes()?;
                if left_bytes.len() != 32 || right_bytes.len() != 32 {
                    return None;
                }
                let mut l = [0u8; 32];
                let mut r = [0u8; 32];
                l.copy_from_slice(&left_bytes);
                r.copy_from_slice(&right_bytes);
                Some(Node::Branch {
                    left: NodeHash(l),
                    right: NodeHash(r),
                })
            }
            _ => None,
        }
    }

    fn encode_node(&self, node: &Node) -> Vec<u8> {
        let mut enc = Encoder::new();
        match node {
            Node::Leaf {
                key_hash,
                value_hash,
                version,
            } => {
                enc.write_u8(0x01);
                enc.write_bytes(key_hash);
                enc.write_bytes(value_hash);
                enc.write_u64(*version);
            }
            Node::Branch { left, right } => {
                enc.write_u8(0x02);
                enc.write_bytes(&left.0);
                enc.write_bytes(&right.0);
            }
        }
        enc.into_bytes()
    }

    pub fn get(&self, hash: &NodeHash) -> Option<Arc<Node>> {
        {
            let mut cache = self.cache.write();
            if let Some(node) = cache.get(hash) {
                return Some(node.clone());
            }
        }
        let path = self.path_for_hash(hash);
        let data = fs::read(&path).ok()?;
        let node = self.decode_node(&data)?;
        let arc_node = Arc::new(node);
        self.cache.write().put(*hash, arc_node.clone());
        Some(arc_node)
    }

    // Atomic write with full fsync protocol
    pub fn put(&self, hash: NodeHash, node: Arc<Node>) -> Result<(), &'static str> {
        let path = self.path_for_hash(&hash);
        let parent_dir = self.ensure_parent_dirs(&path)?;
        let bytes = self.encode_node(&node);
        let tmp = format!("{}.tmp", path.display());

        // 1. Write to temp file
        fs::write(&tmp, &bytes).map_err(|_| "write tmp failed")?;

        // 2. fsync temp file
        let tmp_file = std::fs::File::open(&tmp).map_err(|_| "open tmp for fsync failed")?;
        tmp_file.sync_all().map_err(|_| "fsync tmp failed")?;

        // 3. Atomic rename
        fs::rename(&tmp, &path).map_err(|_| "rename failed")?;

        // 4. fsync parent directory (critical for durability)
        let dir_file = std::fs::File::open(&parent_dir).map_err(|_| "open dir for fsync failed")?;
        dir_file.sync_all().map_err(|_| "fsync dir failed")?;

        self.cache.write().put(hash, node);
        Ok(())
    }
}
