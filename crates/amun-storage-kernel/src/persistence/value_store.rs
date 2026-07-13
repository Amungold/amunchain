use crate::lru::LruCache;
use parking_lot::RwLock;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueKey {
    pub hash: [u8; 32],
    pub length: u64,
}

pub struct ValueStore {
    root_dir: String,
    cache: RwLock<LruCache<ValueKey, Vec<u8>>>,
}

impl ValueStore {
    pub fn new(root_dir: &str) -> Self {
        fs::create_dir_all(root_dir).unwrap();
        Self {
            root_dir: root_dir.to_string(),
            cache: RwLock::new(LruCache::new(10000)),
        }
    }

    fn path(&self, key: &ValueKey) -> PathBuf {
        let hex = hex::encode(key.hash);
        PathBuf::from(format!(
            "{}/{}/{}/{}.val",
            self.root_dir,
            &hex[0..2],
            &hex[2..4],
            &hex[4..]
        ))
    }

    fn ensure_dir(&self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
    }

    pub fn get(&self, key: &ValueKey) -> Option<Vec<u8>> {
        {
            let mut cache = self.cache.write();
            if let Some(v) = cache.get(key) {
                return Some(v.clone());
            }
        }

        let path = self.path(key);
        let data = fs::read(&path).ok()?;
        self.cache.write().put(key.clone(), data.clone());
        Some(data)
    }

    pub fn put(&self, key: ValueKey, value: Vec<u8>) -> Result<(), &'static str> {
        let path = self.path(&key);
        self.ensure_dir(&path);
        let tmp = format!("{}.tmp", path.display());
        fs::write(&tmp, &value).map_err(|_| "write failed")?;
        fs::rename(&tmp, &path).map_err(|_| "rename failed")?;
        self.cache.write().put(key, value);
        Ok(())
    }
}
