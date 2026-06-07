use serde::{Deserialize, Serialize};

use crate::{
    EvidenceLineage, EvidenceStatus, EvidenceType, ObligationId, Reproducibility,
};

/// A single piece of constitutional evidence.
///
/// Each record is immutable once archived, carries a data hash for tamper detection,
/// and can optionally reference a parent through an evidence lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub evidence_id: String,
    pub evidence_type: EvidenceType,
    pub source: String,
    pub timestamp: u64,
    pub data_hash: String,
    pub phase: String,
    pub obligation_ids: Vec<ObligationId>,
    pub reproducibility: Option<Reproducibility>,
    pub status: EvidenceStatus,
    pub lineage: Option<EvidenceLineage>,
}

impl EvidenceRecord {
    pub fn new(
        evidence_id: String,
        evidence_type: EvidenceType,
        source: String,
        timestamp: u64,
        data_hash: String,
        phase: String,
        obligation_ids: Vec<ObligationId>,
    ) -> Self {
        Self {
            evidence_id,
            evidence_type,
            source,
            timestamp,
            data_hash,
            phase,
            obligation_ids,
            reproducibility: None,
            status: EvidenceStatus::Collected,
            lineage: None,
        }
    }

    pub fn with_reproducibility(mut self, reproducibility: Reproducibility) -> Self {
        self.reproducibility = Some(reproducibility);
        self
    }

    pub fn with_lineage(mut self, lineage: EvidenceLineage) -> Self {
        self.lineage = Some(lineage);
        self
    }

    pub fn with_status(mut self, status: EvidenceStatus) -> Self {
        self.status = status;
        self
    }
}
