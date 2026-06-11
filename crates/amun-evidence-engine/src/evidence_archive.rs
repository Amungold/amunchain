use std::collections::BTreeMap;

use crate::evidence_types::ConstitutionalEvidence;

/// A lightweight evidence archive for the VM kernel.
/// In production, this integrates with the full N47 EvidenceArchive.
#[derive(Debug, Clone, Default)]
pub struct VMEvidenceArchive {
    records: BTreeMap<[u8; 32], ConstitutionalEvidence>,
}

impl VMEvidenceArchive {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, evidence: ConstitutionalEvidence) -> [u8; 32] {
        let id = evidence.evidence_id();
        self.records.insert(id, evidence);
        id
    }

    pub fn get(&self, id: &[u8; 32]) -> Option<&ConstitutionalEvidence> {
        self.records.get(id)
    }

    pub fn total(&self) -> usize {
        self.records.len()
    }

    pub fn by_category(&self, category: &str) -> Vec<&ConstitutionalEvidence> {
        self.records
            .values()
            .filter(|e| e.category() == category)
            .collect()
    }

    pub fn violations(&self) -> Vec<&ConstitutionalEvidence> {
        self.records
            .values()
            .filter(|e| matches!(e, ConstitutionalEvidence::ConstitutionalViolation { .. }))
            .collect()
    }

    pub fn failures(&self) -> Vec<&ConstitutionalEvidence> {
        self.records
            .values()
            .filter(|e| matches!(e, ConstitutionalEvidence::ExecutionFailure { .. }))
            .collect()
    }
}
