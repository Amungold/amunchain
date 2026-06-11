use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// A single replay record capturing a state transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayRecord {
    pub height: u64,
    pub tx_hash: String,
    pub state_root_before: String,
    pub state_root_after: String,
    pub commit_hash: String,
}

impl ReplayRecord {
    /// Compute a verification hash for this record.
    pub fn record_hash(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_REPLAY_RECORD_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(self.tx_hash.as_bytes());
        hasher.update(self.state_root_before.as_bytes());
        hasher.update(self.state_root_after.as_bytes());
        hasher.update(self.commit_hash.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
}

/// Persistent store for replay journal.
pub struct ReplayStore {
    pub path: String,
}

impl ReplayStore {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Append a replay record to the journal.
    pub fn append(&self, record: &ReplayRecord) -> Result<(), String> {
        let mut records = self.load_all().unwrap_or_default();
        records.push(record.clone());
        let json = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    /// Load all replay records.
    pub fn load_all(&self) -> Result<Vec<ReplayRecord>, String> {
        if !Path::new(&self.path).exists() {
            return Ok(Vec::new());
        }
        let json = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    /// Load a specific replay record by height.
    pub fn load_height(&self, height: u64) -> Result<Option<ReplayRecord>, String> {
        let records = self.load_all()?;
        Ok(records.into_iter().find(|r| r.height == height))
    }

    /// Verify chain continuity: each record's state_root_before
    /// must match the previous record's state_root_after.
    pub fn verify_chain(&self) -> Result<bool, String> {
        let records = self.load_all()?;
        if records.is_empty() {
            return Ok(true);
        }
        for i in 1..records.len() {
            if records[i].state_root_before != records[i - 1].state_root_after {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_record(h: u64, before: &str, after: &str) -> ReplayRecord {
        ReplayRecord {
            height: h,
            tx_hash: format!("tx{}", h),
            state_root_before: before.to_string(),
            state_root_after: after.to_string(),
            commit_hash: format!("commit{}", h),
        }
    }

    #[test]
    fn n45_append_and_load() {
        let path = "/tmp/n45_replay_store.json";
        let _ = fs::remove_file(path);
        let store = ReplayStore::new(path);
        let record = make_record(1, "root0", "root1");
        store.append(&record).unwrap();
        let records = store.load_all().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].height, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n45_replay_sequence() {
        let path = "/tmp/n45_sequence.json";
        let _ = fs::remove_file(path);
        let store = ReplayStore::new(path);
        store.append(&make_record(1, "root0", "root1")).unwrap();
        store.append(&make_record(2, "root1", "root2")).unwrap();
        store.append(&make_record(3, "root2", "root3")).unwrap();
        assert!(store.verify_chain().unwrap());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n45_corruption_detection() {
        let path = "/tmp/n45_corrupt.json";
        let _ = fs::remove_file(path);
        let store = ReplayStore::new(path);
        store.append(&make_record(1, "root0", "root1")).unwrap();
        store.append(&make_record(2, "WRONG", "root2")).unwrap();
        store.append(&make_record(3, "root2", "root3")).unwrap();
        assert!(!store.verify_chain().unwrap());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n45_height_lookup() {
        let path = "/tmp/n45_lookup.json";
        let _ = fs::remove_file(path);
        let store = ReplayStore::new(path);
        store.append(&make_record(1, "r0", "r1")).unwrap();
        store.append(&make_record(2, "r1", "r2")).unwrap();
        let r = store.load_height(2).unwrap();
        assert!(r.is_some());
        assert_eq!(r.unwrap().state_root_after, "r2");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n45_empty_store() {
        let path = "/tmp/n45_empty.json";
        let _ = fs::remove_file(path);
        let store = ReplayStore::new(path);
        assert!(store.load_all().unwrap().is_empty());
        assert!(store.verify_chain().unwrap());
        let _ = fs::remove_file(path);
    }
}
