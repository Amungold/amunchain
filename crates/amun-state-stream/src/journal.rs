// Persistent Resumable Sync Journal
// Records sync progress so interrupted syncs can resume from the
// last constitutionally VERIFIED chunk.
// FIX: Uses has_verified_chunk bool + optional index to avoid i64->u64 corruption.

use amun_canonical_codec::{CanonicalReader, CanonicalWriter};
use std::fs;
use std::path::PathBuf;

pub const SYNC_JOURNAL_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncJournal {
    pub snapshot_root: [u8; 32],
    pub total_chunks: u64,
    pub verified_chunks: u64,
    pub has_verified_chunk: bool,
    pub last_verified_chunk_index: u64,
    pub last_verified_chunk_hash: [u8; 32],
    pub constitutional_hash: [u8; 32],
    pub manifest_hash: [u8; 32],
    pub is_complete: bool,
    pub journal_path: PathBuf,
}

impl SyncJournal {
    pub fn new(
        snapshot_root: [u8; 32],
        total_chunks: u64,
        constitutional_hash: [u8; 32],
        manifest_hash: [u8; 32],
        journal_path: PathBuf,
    ) -> Self {
        Self {
            snapshot_root,
            total_chunks,
            verified_chunks: 0,
            has_verified_chunk: false,
            last_verified_chunk_index: 0,
            last_verified_chunk_hash: [0u8; 32],
            constitutional_hash,
            manifest_hash,
            is_complete: false,
            journal_path,
        }
    }

    /// Record that a chunk has been constitutionally verified
    pub fn record_verified_chunk(&mut self, chunk_index: u64, chunk_hash: [u8; 32]) {
        self.verified_chunks += 1;
        self.has_verified_chunk = true;
        self.last_verified_chunk_index = chunk_index;
        self.last_verified_chunk_hash = chunk_hash;
        if self.verified_chunks >= self.total_chunks {
            self.is_complete = true;
        }
        self.persist();
    }

    /// Load journal from disk, returns None if no journal exists
    pub fn load(path: &PathBuf) -> Option<Self> {
        let data = fs::read(path).ok()?;
        Self::decode(&data)
    }

    /// Persist journal to disk with atomic write
    pub fn persist(&self) {
        if let Ok(data) = self.encode() {
            let tmp = self.journal_path.with_extension("tmp");
            let _ = fs::write(&tmp, &data);
            let _ = fs::rename(&tmp, &self.journal_path);
        }
    }

    /// Delete journal after successful sync
    pub fn complete(self) {
        let _ = fs::remove_file(&self.journal_path);
    }

    pub fn encode(&self) -> Result<Vec<u8>, String> {
        let mut w = CanonicalWriter::new();
        w.write_u32(SYNC_JOURNAL_VERSION);
        w.write_hash(&self.snapshot_root);
        w.write_u64(self.total_chunks);
        w.write_u64(self.verified_chunks);
        w.write_bool(self.has_verified_chunk);
        w.write_u64(self.last_verified_chunk_index);
        w.write_hash(&self.last_verified_chunk_hash);
        w.write_hash(&self.constitutional_hash);
        w.write_hash(&self.manifest_hash);
        w.write_bool(self.is_complete);
        Ok(w.into_bytes())
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let version = r.read_u32()?;
        if version != SYNC_JOURNAL_VERSION {
            return None;
        }
        let snapshot_root = r.read_hash()?;
        let total_chunks = r.read_u64()?;
        let verified_chunks = r.read_u64()?;
        let has_verified_chunk = r.read_bool()?;
        let last_verified_chunk_index = r.read_u64()?;
        let last_verified_chunk_hash = r.read_hash()?;
        let constitutional_hash = r.read_hash()?;
        let manifest_hash = r.read_hash()?;
        let is_complete = r.read_bool()?;
        Some(Self {
            snapshot_root,
            total_chunks,
            verified_chunks,
            has_verified_chunk,
            last_verified_chunk_index,
            last_verified_chunk_hash,
            constitutional_hash,
            manifest_hash,
            is_complete,
            journal_path: PathBuf::new(),
        })
    }

    /// Next chunk to request (for resumable sync)
    pub fn next_chunk_to_request(&self) -> u64 {
        if self.has_verified_chunk {
            self.last_verified_chunk_index + 1
        } else {
            0
        }
    }

    /// Progress as percentage
    pub fn progress_percent(&self) -> f64 {
        if self.total_chunks == 0 {
            100.0
        } else {
            (self.verified_chunks as f64 / self.total_chunks as f64) * 100.0
        }
    }
}
