//! QCStore - DETERMINISTIC (BTreeMap only)

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use blake3;
use crate::crypto::types::SignatureBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum QCStatus {
    None = 0,
    Justified = 1,
    Locked = 2,
    Finalized = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct QCHash(pub [u8; 32]);

impl QCHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self { Self(bytes) }
    pub fn as_bytes(&self) -> [u8; 32] { self.0 }
    pub fn compute(height: u64, bh: &[u8; 32], parent: Option<&QCHash>, auth_root: &[u8; 32], lineage_root: &[u8; 32], round: u64, epoch: u64, thresh: usize, val_set_hash: &[u8; 32]) -> Self {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"AMUN_QC_V3");
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&round.to_be_bytes());
        bytes.extend_from_slice(&epoch.to_be_bytes());
        bytes.extend_from_slice(bh);
        if let Some(p) = parent { bytes.extend_from_slice(&p.as_bytes()); }
        bytes.extend_from_slice(auth_root);
        bytes.extend_from_slice(lineage_root);
        bytes.extend_from_slice(&(thresh as u64).to_be_bytes());
        bytes.extend_from_slice(val_set_hash);
        Self(blake3::hash(&bytes).into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteCertificate { pub validator_id: u64, pub signature: SignatureBytes }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QC {
    pub hash: QCHash,
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub authority_root: [u8; 32],
    pub lineage_root: [u8; 32],
    pub parent_hash: Option<QCHash>,
    pub vote_count: usize,
    pub threshold: usize,
    pub round: u64,
    pub epoch: u64,
    pub validator_set_hash: [u8; 32],
    pub status: QCStatus,
    pub votes: Vec<VoteCertificate>,
}

impl QC {
    pub fn new(height: u64, bh: [u8; 32], thresh: usize, round: u64, epoch: u64, val_set_hash: [u8; 32]) -> Self {
        Self { hash: QCHash::from_bytes([0u8; 32]), block_height: height, block_hash: bh, authority_root: [0u8; 32], lineage_root: [0u8; 32], parent_hash: None, vote_count: 0, threshold: thresh, round, epoch, validator_set_hash: val_set_hash, status: QCStatus::None, votes: Vec::new() }
    }
    pub fn with_parent(mut self, p: QCHash) -> Self { self.parent_hash = Some(p); self }
    pub fn with_authority_root(mut self, r: [u8; 32]) -> Self { self.authority_root = r; self }
    pub fn with_lineage_root(mut self, r: [u8; 32]) -> Self { self.lineage_root = r; self }
    pub fn finalize(mut self) -> Self {
        self.hash = QCHash::compute(self.block_height, &self.block_hash, self.parent_hash.as_ref(), &self.authority_root, &self.lineage_root, self.round, self.epoch, self.threshold, &self.validator_set_hash);
        self
    }
    pub fn add_vote(&mut self, vid: u64, sig: SignatureBytes) -> bool {
        if self.votes.iter().any(|v| v.validator_id == vid) { return false; }
        self.votes.push(VoteCertificate { validator_id: vid, signature: sig });
        self.vote_count = self.votes.len();
        if self.vote_count >= self.threshold && self.status == QCStatus::None { self.status = QCStatus::Justified; true } else { false }
    }
    pub fn is_justified(&self) -> bool { self.status >= QCStatus::Justified }
    pub fn is_locked(&self) -> bool { self.status >= QCStatus::Locked }
    pub fn is_finalized(&self) -> bool { self.status >= QCStatus::Finalized }
    pub fn mark_locked(&mut self) { if self.status == QCStatus::Justified { self.status = QCStatus::Locked; } }
    pub fn mark_finalized(&mut self) { if self.status == QCStatus::Locked { self.status = QCStatus::Finalized; } }
}

#[derive(Debug, Clone, Default)]
pub struct QCStore {
    qcs: BTreeMap<QCHash, QC>,
    by_height: BTreeMap<u64, Vec<QCHash>>,
}
impl QCStore {
    pub fn new() -> Self { Self { qcs: BTreeMap::new(), by_height: BTreeMap::new() } }
    pub fn insert(&mut self, qc: QC) -> Option<QC> { let h = qc.hash; self.by_height.entry(qc.block_height).or_default().push(h); self.qcs.insert(h, qc) }
    pub fn get(&self, h: &QCHash) -> Option<&QC> { self.qcs.get(h) }
    pub fn values(&self) -> impl Iterator<Item=&QC> { self.qcs.values() }
    pub fn len(&self) -> usize { self.qcs.len() }

    pub fn values_sorted(&self) -> Vec<&QC> {
        let mut values: Vec<&QC> = self.qcs.values().collect();
        values.sort_by(|a, b| {
            a.block_height.cmp(&b.block_height)
                .then_with(|| a.hash.as_bytes().cmp(&b.hash.as_bytes()))
        });
        values
    }
}
