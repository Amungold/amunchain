use amun_canonical_codec::{CanonicalHasher, CanonicalReader, CanonicalWriter};
use amun_canonical_codec::{PROTOCOL_CHAIN_ID, PROTOCOL_DOMAIN_CONSTITUTION};
use amun_storage_kernel::SparseMerkleTree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalIdentity {
    pub constitutional_hash: [u8; 32],
    pub protocol_version: u32,
    pub snapshot_version: u32,
    pub proof_version: u8,
    pub codec_version: u16,
    pub canonical_empty_root: [u8; 32],
    pub max_depth: usize,
    pub max_chunk_size: u64,
    pub chain_id: [u8; 32],
    pub identity_hash: [u8; 32],
}

impl ConstitutionalIdentity {
    pub fn new(constitutional_hash: [u8; 32]) -> Self {
        let canonical_empty_root = SparseMerkleTree::canonical_empty_root();
        let mut id = Self {
            constitutional_hash,
            protocol_version: 1,
            snapshot_version: 1,
            proof_version: 0x01,
            codec_version: 1,
            canonical_empty_root,
            max_depth: 256,
            max_chunk_size: 16 * 1024 * 1024,
            chain_id: PROTOCOL_CHAIN_ID,
            identity_hash: [0u8; 32],
        };
        id.identity_hash = id.compute_identity_hash();
        id
    }

    fn compute_identity_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_CONSTITUTION);
        h.update(&self.constitutional_hash);
        h.update_u64(self.protocol_version as u64);
        h.update_u64(self.snapshot_version as u64);
        h.update(&[self.proof_version]);
        h.update(&self.codec_version.to_le_bytes());
        h.update(&self.canonical_empty_root);
        h.update_u64(self.max_depth as u64);
        h.update_u64(self.max_chunk_size);
        h.update(&self.chain_id);
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_identity_hash() == self.identity_hash
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_hash(&self.constitutional_hash);
        w.write_u32(self.protocol_version);
        w.write_u32(self.snapshot_version);
        w.write_u8(self.proof_version);
        w.write_u16(self.codec_version);
        w.write_hash(&self.canonical_empty_root);
        w.write_u64(self.max_depth as u64);
        w.write_u64(self.max_chunk_size);
        w.write_hash(&self.chain_id);
        w.write_hash(&self.identity_hash);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let constitutional_hash = r.read_hash()?;
        let protocol_version = r.read_u32()?;
        let snapshot_version = r.read_u32()?;
        let proof_version = r.read_u8()?;
        let codec_version = r.read_u16()?;
        let canonical_empty_root = r.read_hash()?;
        let max_depth = r.read_u64()? as usize;
        let max_chunk_size = r.read_u64()?;
        let chain_id = r.read_hash()?;
        let identity_hash = r.read_hash()?;
        if !r.is_finished() {
            return None;
        }
        Some(Self {
            constitutional_hash,
            protocol_version,
            snapshot_version,
            proof_version,
            codec_version,
            canonical_empty_root,
            max_depth,
            max_chunk_size,
            chain_id,
            identity_hash,
        })
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.identity_hash == other.identity_hash
    }
}
