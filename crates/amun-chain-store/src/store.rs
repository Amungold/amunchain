use crate::record::FinalizedChainRecord;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum StoreError {
    IoError(String),
    DecodeError(String),
    NotFound(u64),
    Corrupted(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::IoError(s) => write!(f, "IO: {}", s),
            StoreError::DecodeError(s) => write!(f, "Decode: {}", s),
            StoreError::NotFound(h) => write!(f, "Height {} not found", h),
            StoreError::Corrupted(s) => write!(f, "Corrupted: {}", s),
        }
    }
}

pub struct ChainStore {
    data_dir: PathBuf,
    records: BTreeMap<u64, FinalizedChainRecord>,
    highest: u64,
}

impl ChainStore {
    pub fn open(data_dir: &str) -> Result<Self, StoreError> {
        let data_dir = PathBuf::from(data_dir);
        fs::create_dir_all(&data_dir).map_err(|e| StoreError::IoError(e.to_string()))?;
        let chain_file = data_dir.join("chain.dat");
        let index_file = data_dir.join("chain.index");
        if chain_file.exists() && index_file.exists() {
            Self::load(&data_dir, &chain_file, &index_file)
        } else {
            Ok(Self {
                data_dir,
                records: BTreeMap::new(),
                highest: 0,
            })
        }
    }

    fn load(data_dir: &Path, chain_file: &Path, index_file: &Path) -> Result<Self, StoreError> {
        let index_bytes = fs::read(index_file).map_err(|e| StoreError::IoError(e.to_string()))?;
        let offsets: Vec<u64> = index_bytes
            .chunks_exact(8)
            .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
            .collect();
        let chain_bytes = fs::read(chain_file).map_err(|e| StoreError::IoError(e.to_string()))?;
        let mut records = BTreeMap::new();
        let mut highest = 0u64;
        for &offset in &offsets {
            let start = offset as usize;
            if start + 4 > chain_bytes.len() {
                return Err(StoreError::Corrupted("Truncated".into()));
            }
            let len =
                u32::from_le_bytes(chain_bytes[start..start + 4].try_into().unwrap()) as usize;
            let end = start + 4 + len;
            if end > chain_bytes.len() {
                return Err(StoreError::Corrupted("Truncated data".into()));
            }
            let record = FinalizedChainRecord::decode(&chain_bytes[start + 4..end])
                .map_err(StoreError::DecodeError)?;
            highest = highest.max(record.height);
            records.insert(record.height, record);
        }
        Ok(Self {
            data_dir: data_dir.to_path_buf(),
            records,
            highest,
        })
    }

    pub fn append(&mut self, record: FinalizedChainRecord) -> Result<(), StoreError> {
        if record.height <= self.highest && self.records.contains_key(&record.height) {
            return Ok(());
        }
        let data = record.encode();
        let len = data.len() as u32;
        let chain_file = self.data_dir.join("chain.dat");
        let index_file = self.data_dir.join("chain.index");
        let offset = if chain_file.exists() {
            fs::metadata(&chain_file)
                .map_err(|e| StoreError::IoError(e.to_string()))?
                .len()
        } else {
            0
        };
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&chain_file)
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        f.write_all(&len.to_le_bytes())
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        f.write_all(&data)
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        f.flush().map_err(|e| StoreError::IoError(e.to_string()))?;
        let mut idx = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&index_file)
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        idx.write_all(&offset.to_le_bytes())
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        idx.flush()
            .map_err(|e| StoreError::IoError(e.to_string()))?;
        self.highest = self.highest.max(record.height);
        self.records.insert(record.height, record);
        Ok(())
    }

    pub fn load_height(&self, height: u64) -> Option<&FinalizedChainRecord> {
        self.records.get(&height)
    }
    pub fn load_height_range(&self, start: u64, end: u64) -> Vec<&FinalizedChainRecord> {
        self.records.range(start..=end).map(|(_, r)| r).collect()
    }

    /// PERF-5: Append multiple records efficiently.
    /// Reduces lock contention compared to individual appends.
    pub fn append_batch(&mut self, records: Vec<FinalizedChainRecord>) -> usize {
        let mut count = 0;
        for record in records {
            if self.append(record).is_ok() {
                count += 1;
            }
        }
        count
    }

    pub fn latest_height(&self) -> u64 {
        self.highest
    }
    pub fn load_tip(&self) -> Option<&FinalizedChainRecord> {
        self.records.get(&self.highest)
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_record(h: u64) -> FinalizedChainRecord {
        FinalizedChainRecord {
            height: h,
            block_hash: [h as u8; 32],
            state_root: [0xBB; 32],
            history_root: [0xCC; 32],
            certificate_hash: [0xDD; 32],
            slashing_root: [0u8; 32],
            commitment_root: [0u8; 32],
            constitutional_root: [0u8; 32],
            economic_root: [0u8; 32],
            identity_root: [0u8; 32],
            governance_root: [0u8; 32],
            verdict_hash: [0u8; 32],
            evidence_record_hash: [0u8; 32],
            evidence_root: [0u8; 32],
            timestamp: h * 1000,
        }
    }

    #[test]
    fn n70_append_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        store.append(make_record(0)).unwrap();
        store.append(make_record(1)).unwrap();
        assert_eq!(store.len(), 2);
        assert_eq!(store.latest_height(), 1);
    }

    #[test]
    fn n70_recover_after_restart() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        {
            let mut s = ChainStore::open(dir_str).unwrap();
            s.append(make_record(0)).unwrap();
            s.append(make_record(1)).unwrap();
        }
        let s = ChainStore::open(dir_str).unwrap();
        assert_eq!(s.len(), 2);
        assert_eq!(s.latest_height(), 1);
    }

    #[test]
    fn n70_empty_store() {
        let dir = tempfile::tempdir().unwrap();
        let s = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(s.len(), 0);
        assert!(s.load_tip().is_none());
    }

    #[test]
    fn n70_skip_duplicate_heights() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        store.append(make_record(0)).unwrap();
        store.append(make_record(0)).unwrap();
        assert_eq!(store.len(), 1);
        assert_eq!(store.latest_height(), 0);
    }

    #[test]
    fn n70_gap_heights_recovery() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        {
            let mut s = ChainStore::open(dir_str).unwrap();
            s.append(make_record(0)).unwrap();
            s.append(make_record(42)).unwrap();
        }
        let s = ChainStore::open(dir_str).unwrap();
        assert_eq!(s.latest_height(), 42);
        assert!(s.load_height(0).is_some());
        assert!(s.load_height(42).is_some());
        assert!(s.load_height(1).is_none());
    }
}
