// ============================================================================
// N109.10/N109.11 — Evidence Store (HashMap-based)
// ============================================================================
// N109.11: Upgraded from Vec to HashMap<evidence_id, EvidenceRecord> for O(1) lookup.
// This is required for N110 Slashing which does frequent evidence lookups.
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// N109.10: Types of validator misbehavior that produce evidence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EvidenceType {
    StateRootMismatch,
    InvalidExecutionCommitment,
    InvalidSignature,
    VoteBindingViolation,
    DoubleVote,
    FutureVote,
    ExecutionFailure,
}

/// N109.10: Lifecycle status of an evidence record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceStatus {
    Pending,
    Confirmed,
    Slashed,
    Rejected,
}

/// N109.10: A permanent cryptographic record of validator misbehavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceRecord {
    #[serde(with = "serde_bytes")]
    pub evidence_id: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub validator_id: [u8; 32],
    pub height: u64,
    pub evidence_type: EvidenceType,
    pub timestamp: u64,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
    pub status: EvidenceStatus,
}

impl EvidenceRecord {
    pub fn compute_evidence_id(
        validator_id: &[u8; 32],
        height: u64,
        evidence_type: &EvidenceType,
        payload: &[u8],
    ) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_V1");
        hasher.update(validator_id);
        hasher.update(&height.to_le_bytes());
        let type_byte = match evidence_type {
            EvidenceType::StateRootMismatch => 0x01u8,
            EvidenceType::InvalidExecutionCommitment => 0x02,
            EvidenceType::InvalidSignature => 0x03,
            EvidenceType::VoteBindingViolation => 0x04,
            EvidenceType::DoubleVote => 0x05,
            EvidenceType::FutureVote => 0x06,
            EvidenceType::ExecutionFailure => 0x07,
        };
        hasher.update(&[type_byte]);
        hasher.update(payload);
        hasher.finalize().into()
    }

    pub fn new(
        validator_id: [u8; 32],
        height: u64,
        evidence_type: EvidenceType,
        timestamp: u64,
        payload: Vec<u8>,
    ) -> Self {
        let evidence_id =
            Self::compute_evidence_id(&validator_id, height, &evidence_type, &payload);
        Self {
            evidence_id,
            validator_id,
            height,
            evidence_type,
            timestamp,
            payload,
            status: EvidenceStatus::Pending,
        }
    }

    pub fn state_root_mismatch(
        v: [u8; 32],
        h: u64,
        t: u64,
        proposed: &[u8; 32],
        computed: &[u8; 32],
    ) -> Self {
        let mut payload = Vec::new();
        payload.extend_from_slice(proposed);
        payload.extend_from_slice(computed);
        Self::new(v, h, EvidenceType::StateRootMismatch, t, payload)
    }

    pub fn invalid_execution_commitment(v: [u8; 32], h: u64, t: u64, reason: &str) -> Self {
        Self::new(
            v,
            h,
            EvidenceType::InvalidExecutionCommitment,
            t,
            reason.as_bytes().to_vec(),
        )
    }

    pub fn invalid_signature(v: [u8; 32], h: u64, t: u64, key: &[u8; 32]) -> Self {
        Self::new(v, h, EvidenceType::InvalidSignature, t, key.to_vec())
    }

    pub fn vote_binding_violation(v: [u8; 32], h: u64, t: u64, detail: &str) -> Self {
        Self::new(
            v,
            h,
            EvidenceType::VoteBindingViolation,
            t,
            detail.as_bytes().to_vec(),
        )
    }
}

/// N109.11: HashMap-based evidence store with O(1) lookup by evidence_id
#[derive(Debug, Clone)]
pub struct EvidenceStore {
    pub records: HashMap<[u8; 32], EvidenceRecord>,
    persist_path: Option<PathBuf>,
}

impl EvidenceStore {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            persist_path: None,
        }
    }

    pub fn with_persistence(path: PathBuf) -> Self {
        Self {
            records: HashMap::new(),
            persist_path: Some(path),
        }
    }

    /// N109.11: O(1) store with deduplication
    pub fn store_evidence(&mut self, record: EvidenceRecord) -> bool {
        if self.records.contains_key(&record.evidence_id) {
            return false;
        }
        self.records.insert(record.evidence_id, record);
        true
    }

    /// N109.11: O(1) lookup by evidence_id
    pub fn get_by_id(&self, evidence_id: &[u8; 32]) -> Option<&EvidenceRecord> {
        self.records.get(evidence_id)
    }

    /// N109.10: Query evidence for a specific validator at a specific height
    pub fn get_evidence_for(&self, validator_id: &[u8; 32], height: u64) -> Vec<&EvidenceRecord> {
        self.records
            .values()
            .filter(|r| r.validator_id == *validator_id && r.height == height)
            .collect()
    }

    /// N109.11: Get ALL evidence for a validator (for MisbehaviorRegistry)
    pub fn get_all_for_validator(&self, validator_id: &[u8; 32]) -> Vec<&EvidenceRecord> {
        self.records
            .values()
            .filter(|r| r.validator_id == *validator_id)
            .collect()
    }

    pub fn update_status(&mut self, evidence_id: &[u8; 32], status: EvidenceStatus) -> bool {
        if let Some(record) = self.records.get_mut(evidence_id) {
            record.status = status;
            true
        } else {
            false
        }
    }

    pub fn get_pending(&self) -> Vec<&EvidenceRecord> {
        self.records
            .values()
            .filter(|r| r.status == EvidenceStatus::Pending)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    // Persistence (unchanged logic, adapted for HashMap)
    pub fn persist(&self) -> Result<(), String> {
        let path = match &self.persist_path {
            Some(p) => p.clone(),
            None => return Err("No persist_path configured".into()),
        };
        let mut content = String::new();
        for record in self.records.values() {
            let json = serde_json::to_string(record).map_err(|e| format!("Serialize: {}", e))?;
            content.push_str(&json);
            content.push('\n');
        }
        std::fs::write(&path, content).map_err(|e| format!("Write: {}", e))?;
        Ok(())
    }

    pub fn load(path: PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self {
                records: HashMap::new(),
                persist_path: Some(path),
            });
        }
        let content = std::fs::read_to_string(&path).map_err(|e| format!("Read: {}", e))?;
        let mut records = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let record: EvidenceRecord =
                serde_json::from_str(line).map_err(|e| format!("Deserialize: {}", e))?;
            records.insert(record.evidence_id, record);
        }
        Ok(Self {
            records,
            persist_path: Some(path),
        })
    }
}

impl Default for EvidenceStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn n109_10_store_and_query() {
        let mut store = EvidenceStore::new();
        let e = EvidenceRecord::new(
            [0x42; 32],
            5,
            EvidenceType::StateRootMismatch,
            1000,
            vec![1, 2, 3],
        );
        assert!(store.store_evidence(e.clone()));
        assert_eq!(store.get_evidence_for(&[0x42; 32], 5).len(), 1);
    }

    #[test]
    fn n109_10_duplicate_evidence_is_deduplicated() {
        let mut store = EvidenceStore::new();
        let e = EvidenceRecord::new(
            [0x42; 32],
            3,
            EvidenceType::InvalidSignature,
            2000,
            vec![9, 9, 9],
        );
        assert!(store.store_evidence(e.clone()));
        assert!(!store.store_evidence(e.clone()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn n109_10_evidence_survives_restart() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("evidence.jsonl");
        let id = {
            let mut s = EvidenceStore::with_persistence(path.clone());
            let e = EvidenceRecord::new(
                [0xDE; 32],
                7,
                EvidenceType::VoteBindingViolation,
                3000,
                b"test".to_vec(),
            );
            let id = e.evidence_id;
            s.store_evidence(e);
            s.persist().unwrap();
            id
        };
        let s = EvidenceStore::load(path).unwrap();
        assert_eq!(s.len(), 1);
        assert!(s.get_by_id(&id).is_some());
    }

    #[test]
    fn n109_10_status_lifecycle() {
        let mut s = EvidenceStore::new();
        let e = EvidenceRecord::new([0xAA; 32], 10, EvidenceType::DoubleVote, 5000, vec![5, 6]);
        let id = e.evidence_id;
        s.store_evidence(e);
        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Pending);
        s.update_status(&id, EvidenceStatus::Confirmed);
        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Confirmed);
        s.update_status(&id, EvidenceStatus::Slashed);
        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Slashed);
    }

    #[test]
    fn n109_11_get_all_for_validator() {
        let mut s = EvidenceStore::new();
        s.store_evidence(EvidenceRecord::new(
            [1u8; 32],
            1,
            EvidenceType::InvalidSignature,
            1000,
            vec![1],
        ));
        s.store_evidence(EvidenceRecord::new(
            [1u8; 32],
            2,
            EvidenceType::StateRootMismatch,
            2000,
            vec![2],
        ));
        s.store_evidence(EvidenceRecord::new(
            [2u8; 32],
            1,
            EvidenceType::DoubleVote,
            3000,
            vec![3],
        ));
        assert_eq!(s.get_all_for_validator(&[1u8; 32]).len(), 2);
        assert_eq!(s.get_all_for_validator(&[2u8; 32]).len(), 1);
    }
}
