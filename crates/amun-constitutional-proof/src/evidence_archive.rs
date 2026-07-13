use std::collections::BTreeMap;

use crate::{EvidenceRecord, EvidenceStatus, EvidenceType, ObligationId};

/// The constitutional evidence archive.
///
/// Stores and manages all evidence records, enforcing the admissibility
/// rules defined in Article III:
/// - Only `Verified` or `Archived` evidence is admissible for verdicts.
/// - `Rejected` evidence is kept forever but never used.
/// - Evidence lineage hashes are verified on insertion.
/// - Records are never deleted, only superseded or rejected.
#[derive(Debug, Clone, Default)]
pub struct EvidenceArchive {
    records: BTreeMap<String, EvidenceRecord>,
}

impl EvidenceArchive {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new evidence record into the archive.
    ///
    /// Returns an error if the evidence ID already exists.
    /// If the record carries a lineage, the parent's hash is verified before insertion.
    pub fn insert(&mut self, record: EvidenceRecord) -> Result<(), String> {
        if self.records.contains_key(&record.evidence_id) {
            return Err(format!("duplicate evidence id: {}", record.evidence_id));
        }
        if let Some(ref lineage) = record.lineage {
            if let Some(parent) = self.records.get(&lineage.parent_id) {
                if parent.data_hash != lineage.parent_hash {
                    return Err(format!(
                        "lineage parent hash mismatch for {}: expected {} but parent has {}",
                        record.evidence_id, lineage.parent_hash, parent.data_hash
                    ));
                }
            } else {
                return Err(format!(
                    "lineage parent {} not found for evidence {}",
                    lineage.parent_id, record.evidence_id
                ));
            }
        }
        self.records.insert(record.evidence_id.clone(), record);
        Ok(())
    }

    /// Retrieve an evidence record by ID.
    pub fn get(&self, evidence_id: &str) -> Option<&EvidenceRecord> {
        self.records.get(evidence_id)
    }

    /// Total number of records in the archive.
    pub fn total_count(&self) -> usize {
        self.records.len()
    }

    /// Return all records belonging to a given phase.
    pub fn by_phase(&self, phase: &str) -> Vec<&EvidenceRecord> {
        self.records.values().filter(|r| r.phase == phase).collect()
    }

    /// Return all records of a specific evidence type.
    pub fn by_type(&self, ev_type: EvidenceType) -> Vec<&EvidenceRecord> {
        self.records
            .values()
            .filter(|r| r.evidence_type == ev_type)
            .collect()
    }

    /// Return all records that support a particular obligation.
    pub fn by_obligation(&self, obligation_id: &ObligationId) -> Vec<&EvidenceRecord> {
        self.records
            .values()
            .filter(|r| r.obligation_ids.contains(obligation_id))
            .collect()
    }

    /// Mark a record as `Verified`.
    pub fn verify(&mut self, evidence_id: &str) -> Result<(), String> {
        let record = self
            .records
            .get_mut(evidence_id)
            .ok_or_else(|| format!("evidence {} not found", evidence_id))?;
        if record.status == EvidenceStatus::Rejected {
            return Err("cannot verify rejected evidence".into());
        }
        record.status = EvidenceStatus::Verified;
        Ok(())
    }

    /// Mark a record as `Archived`.
    pub fn archive(&mut self, evidence_id: &str) -> Result<(), String> {
        let record = self
            .records
            .get_mut(evidence_id)
            .ok_or_else(|| format!("evidence {} not found", evidence_id))?;
        if record.status == EvidenceStatus::Collected {
            return Err("evidence must be verified before archiving".into());
        }
        if record.status == EvidenceStatus::Rejected {
            return Err("cannot archive rejected evidence".into());
        }
        record.status = EvidenceStatus::Archived;
        Ok(())
    }

    /// Mark a record as `Rejected`. Once rejected, the record is preserved
    /// indefinitely but is inadmissible for any constitutional verdict.
    pub fn reject(&mut self, evidence_id: &str) -> Result<(), String> {
        let record = self
            .records
            .get_mut(evidence_id)
            .ok_or_else(|| format!("evidence {} not found", evidence_id))?;
        record.status = EvidenceStatus::Rejected;
        Ok(())
    }

    /// Return true if the evidence is constitutionally admissible
    /// (i.e. `Verified` or `Archived`).
    pub fn is_admissible(record: &EvidenceRecord) -> bool {
        matches!(
            record.status,
            EvidenceStatus::Verified | EvidenceStatus::Archived
        )
    }
}
