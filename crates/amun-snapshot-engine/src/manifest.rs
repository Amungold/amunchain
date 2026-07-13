// Snapshot Manifest v1.1 - Constitutional Document for State Transfer
// Now includes snapshot_cutoff_sequence and snapshot_cutoff_root
// to bind the snapshot to an exact WAL boundary.

use amun_canonical_codec::PROTOCOL_DOMAIN_MANIFEST;
use amun_canonical_codec::{CanonicalHasher, CanonicalReader, CanonicalWriter};

pub const MANIFEST_VERSION_V1: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotManifest {
    pub manifest_version: u32,
    pub snapshot_version: u32,
    pub protocol_version: u32,
    pub state_root: [u8; 32],
    pub canonical_empty_root: [u8; 32],
    pub chunk_count: u64,
    pub chunk_root: [u8; 32],
    pub total_nodes: u64,
    pub total_size: u64,
    pub created_at_epoch: u64,
    pub created_at_generation: u64,
    // Constitutional cutoff: binds snapshot to exact WAL position
    pub snapshot_cutoff_sequence: u64,
    pub snapshot_cutoff_root: [u8; 32],
    pub constitutional_hash: [u8; 32],
    pub manifest_hash: [u8; 32],
}

impl SnapshotManifest {
    pub fn new(
        state_root: [u8; 32],
        canonical_empty_root: [u8; 32],
        chunk_count: u64,
        chunk_root: [u8; 32],
        total_nodes: u64,
        total_size: u64,
        epoch: u64,
        generation: u64,
        cutoff_sequence: u64,
        cutoff_root: [u8; 32],
        constitutional_hash: [u8; 32],
    ) -> Self {
        let mut m = Self {
            manifest_version: MANIFEST_VERSION_V1,
            snapshot_version: 1,
            protocol_version: 1,
            state_root,
            canonical_empty_root,
            chunk_count,
            chunk_root,
            total_nodes,
            total_size,
            created_at_epoch: epoch,
            created_at_generation: generation,
            snapshot_cutoff_sequence: cutoff_sequence,
            snapshot_cutoff_root: cutoff_root,
            constitutional_hash,
            manifest_hash: [0u8; 32],
        };
        m.manifest_hash = m.compute_self_hash();
        m
    }

    fn compute_self_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_MANIFEST);
        h.update_u64(self.manifest_version as u64);
        h.update_u64(self.snapshot_version as u64);
        h.update_u64(self.protocol_version as u64);
        h.update(&self.state_root);
        h.update(&self.canonical_empty_root);
        h.update_u64(self.chunk_count);
        h.update(&self.chunk_root);
        h.update_u64(self.total_nodes);
        h.update_u64(self.total_size);
        h.update_u64(self.created_at_epoch);
        h.update_u64(self.created_at_generation);
        h.update_u64(self.snapshot_cutoff_sequence);
        h.update(&self.snapshot_cutoff_root);
        h.update(&self.constitutional_hash);
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_self_hash() == self.manifest_hash
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u32(self.manifest_version);
        w.write_u32(self.snapshot_version);
        w.write_u32(self.protocol_version);
        w.write_hash(&self.state_root);
        w.write_hash(&self.canonical_empty_root);
        w.write_u64(self.chunk_count);
        w.write_hash(&self.chunk_root);
        w.write_u64(self.total_nodes);
        w.write_u64(self.total_size);
        w.write_u64(self.created_at_epoch);
        w.write_u64(self.created_at_generation);
        w.write_u64(self.snapshot_cutoff_sequence);
        w.write_hash(&self.snapshot_cutoff_root);
        w.write_hash(&self.constitutional_hash);
        w.write_hash(&self.manifest_hash);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let manifest_version = r.read_u32()?;
        if manifest_version != MANIFEST_VERSION_V1 {
            return None;
        }
        let snapshot_version = r.read_u32()?;
        let protocol_version = r.read_u32()?;
        let state_root = r.read_hash()?;
        let canonical_empty_root = r.read_hash()?;
        let chunk_count = r.read_u64()?;
        let chunk_root = r.read_hash()?;
        let total_nodes = r.read_u64()?;
        let total_size = r.read_u64()?;
        let created_at_epoch = r.read_u64()?;
        let created_at_generation = r.read_u64()?;
        let snapshot_cutoff_sequence = r.read_u64()?;
        let snapshot_cutoff_root = r.read_hash()?;
        let constitutional_hash = r.read_hash()?;
        let manifest_hash = r.read_hash()?;
        if !r.is_finished() {
            return None;
        }
        Some(Self {
            manifest_version,
            snapshot_version,
            protocol_version,
            state_root,
            canonical_empty_root,
            chunk_count,
            chunk_root,
            total_nodes,
            total_size,
            created_at_epoch,
            created_at_generation,
            snapshot_cutoff_sequence,
            snapshot_cutoff_root,
            constitutional_hash,
            manifest_hash,
        })
    }
}

// ============================================================
// Manifest Builder
// ============================================================
pub struct ManifestBuilder {
    state_root: Option<[u8; 32]>,
    canonical_empty_root: Option<[u8; 32]>,
    chunk_root: Option<[u8; 32]>,
    constitutional_hash: Option<[u8; 32]>,
    epoch: u64,
    generation: u64,
    cutoff_sequence: u64,
    cutoff_root: Option<[u8; 32]>,
}

impl ManifestBuilder {
    pub fn new() -> Self {
        Self {
            state_root: None,
            canonical_empty_root: None,
            chunk_root: None,
            constitutional_hash: None,
            epoch: 0,
            generation: 0,
            cutoff_sequence: 0,
            cutoff_root: None,
        }
    }

    pub fn with_state_root(mut self, root: [u8; 32]) -> Self {
        self.state_root = Some(root);
        self
    }
    pub fn with_canonical_empty_root(mut self, root: [u8; 32]) -> Self {
        self.canonical_empty_root = Some(root);
        self
    }
    pub fn with_chunk_root(mut self, root: [u8; 32]) -> Self {
        self.chunk_root = Some(root);
        self
    }
    pub fn with_constitutional_hash(mut self, hash: [u8; 32]) -> Self {
        self.constitutional_hash = Some(hash);
        self
    }
    pub fn with_epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }
    pub fn with_generation(mut self, gen: u64) -> Self {
        self.generation = gen;
        self
    }
    pub fn with_cutoff(mut self, seq: u64, root: [u8; 32]) -> Self {
        self.cutoff_sequence = seq;
        self.cutoff_root = Some(root);
        self
    }

    pub fn build(self) -> Option<SnapshotManifest> {
        Some(SnapshotManifest::new(
            self.state_root?,
            self.canonical_empty_root?,
            0,
            self.chunk_root?,
            0,
            0,
            self.epoch,
            self.generation,
            self.cutoff_sequence,
            self.cutoff_root?,
            self.constitutional_hash?,
        ))
    }
}
