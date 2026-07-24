use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// A stored block in the chain history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredBlock {
    pub height: u64,
    pub hash: String,
    pub parent_hash: String,
    pub state_root: String,
    pub evidence_root: String,
}

/// Persistent store for block history.
pub struct BlockStore {
    pub path: String,
}

impl BlockStore {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Append a block to the store.
    pub fn append(&self, block: &StoredBlock) -> Result<(), String> {
        let mut blocks = self.load_all().unwrap_or_default();
        blocks.push(block.clone());
        let json = serde_json::to_string_pretty(&blocks).map_err(|e| e.to_string())?;
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    /// Load all stored blocks.
    pub fn load_all(&self) -> Result<Vec<StoredBlock>, String> {
        if !Path::new(&self.path).exists() {
            return Ok(Vec::new());
        }
        let json = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    /// Load a specific block by height.
    pub fn load_height(&self, height: u64) -> Result<Option<StoredBlock>, String> {
        let blocks = self.load_all()?;
        Ok(blocks.into_iter().find(|b| b.height == height))
    }

    /// Count of stored blocks.
    pub fn count(&self) -> Result<usize, String> {
        Ok(self.load_all()?.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_block(h: u64) -> StoredBlock {
        StoredBlock {
            height: h,
            hash: format!("hash{}", h),
            parent_hash: format!("parent{}", h.saturating_sub(1)),
            state_root: format!("state{}", h),
            evidence_root: format!("evidence{}", h),
        }
    }

    #[test]
    fn n44_append_and_load() {
        let path = "/tmp/n44_block_store.json";
        let _ = fs::remove_file(path);

        let store = BlockStore::new(path);
        let block = make_block(1);
        store.append(&block).unwrap();
        let blocks = store.load_all().unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].height, 1);
        assert_eq!(blocks[0].hash, "hash1");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n44_multiple_blocks() {
        let path = "/tmp/n44_multi_blocks.json";
        let _ = fs::remove_file(path);

        let store = BlockStore::new(path);
        for i in 1..=5 {
            store.append(&make_block(i)).unwrap();
        }
        let blocks = store.load_all().unwrap();
        assert_eq!(blocks.len(), 5);
        assert_eq!(store.count().unwrap(), 5);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n44_height_lookup() {
        let path = "/tmp/n44_lookup.json";
        let _ = fs::remove_file(path);

        let store = BlockStore::new(path);
        for i in 1..=3 {
            store.append(&make_block(i)).unwrap();
        }

        let block = store.load_height(2).unwrap();
        assert!(block.is_some());
        assert_eq!(block.unwrap().state_root, "state2");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n44_missing_height() {
        let path = "/tmp/n44_missing.json";
        let _ = fs::remove_file(path);

        let store = BlockStore::new(path);
        store.append(&make_block(1)).unwrap();

        let block = store.load_height(9999).unwrap();
        assert!(block.is_none());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn n44_empty_store() {
        let path = "/tmp/n44_empty.json";
        let _ = fs::remove_file(path);

        let store = BlockStore::new(path);
        let blocks = store.load_all().unwrap();
        assert!(blocks.is_empty());

        let _ = fs::remove_file(path);
    }
}
