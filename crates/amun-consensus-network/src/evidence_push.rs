// N112.1 — Push-Based Evidence Propagation
// =========================================
// Extends EvidenceGossip with proactive push of evidence announcements
// and full EvidenceRecords to known peers.  This reduces latency in
// evidence distribution and removes the single-point-of-failure pull
// model.

use crate::evidence_gossip::{EvidenceAnnouncement, EvidenceGossip};
use crate::evidence_store::{EvidenceRecord, EvidenceStore};
use std::collections::HashSet;

/// Configuration for push-based evidence gossip.
pub struct EvidencePushConfig {
    /// Maximum number of peers to push to in one round.
    pub max_push_peers: usize,
    /// Maximum evidence records to push in one batch.
    pub max_batch_size: usize,
    /// Whether to push full EvidenceRecords or just announcements.
    pub push_full_records: bool,
}

impl Default for EvidencePushConfig {
    fn default() -> Self {
        Self {
            max_push_peers: 8,
            max_batch_size: 50,
            push_full_records: true,
        }
    }
}

/// N112.1: Push engine that works alongside EvidenceGossip.
pub struct EvidencePush {
    config: EvidencePushConfig,
    /// Evidence IDs that have already been pushed.
    pushed_ids: HashSet<[u8; 32]>,
}

impl EvidencePush {
    pub fn new(config: EvidencePushConfig) -> Self {
        Self {
            config,
            pushed_ids: HashSet::new(),
        }
    }

    /// Select evidence that should be pushed to peers.
    /// Returns a list of (EvidenceRecord, EvidenceAnnouncement) pairs
    /// for evidence that hasn't been pushed yet.
    pub fn select_for_push(
        &mut self,
        store: &EvidenceStore,
        _gossip: &EvidenceGossip,
    ) -> Vec<(EvidenceRecord, EvidenceAnnouncement)> {
        let mut selected = Vec::new();

        for record in store.records.values() {
            if self.pushed_ids.contains(&record.evidence_id) {
                continue;
            }
            if selected.len() >= self.config.max_batch_size {
                break;
            }

            // Build announcement from the record
            let announcement = EvidenceAnnouncement {
                evidence_id: record.evidence_id,
                validator_id: record.validator_id,
                evidence_type: record.evidence_type.clone(),
                height: record.height,
                timestamp: record.timestamp,
            };

            selected.push((record.clone(), announcement));
            self.pushed_ids.insert(record.evidence_id);
        }

        selected
    }

    /// Mark evidence as pushed (without actually sending it).
    pub fn mark_pushed(&mut self, evidence_id: &[u8; 32]) {
        self.pushed_ids.insert(*evidence_id);
    }

    /// Check if an evidence ID has already been pushed.
    pub fn has_been_pushed(&self, evidence_id: &[u8; 32]) -> bool {
        self.pushed_ids.contains(evidence_id)
    }

    /// Return the number of evidence IDs already pushed.
    pub fn pushed_count(&self) -> usize {
        self.pushed_ids.len()
    }

    /// Reset the pushed tracking (e.g., on restart or reconnection).
    pub fn reset(&mut self) {
        self.pushed_ids.clear();
    }
}

impl Default for EvidencePush {
    fn default() -> Self {
        Self::new(EvidencePushConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;

    fn make_store_with_evidence(count: u8) -> (EvidenceStore, Vec<[u8; 32]>) {
        let mut store = EvidenceStore::new();
        let mut ids = Vec::new();
        for i in 0..count {
            let record = EvidenceRecord::new(
                [i; 32],
                i as u64 + 1,
                EvidenceType::DoubleVote,
                1000 * (i as u64 + 1),
                vec![i; 8],
            );
            let id = record.evidence_id;
            store.store_evidence(record);
            ids.push(id);
        }
        (store, ids)
    }

    #[test]
    fn n112_1_select_unpushed_evidence() {
        let (store, ids) = make_store_with_evidence(5);
        let gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        let selected = push.select_for_push(&store, &gossip);
        assert_eq!(selected.len(), 5);
        // All 5 should now be marked as pushed
        for id in &ids {
            assert!(push.has_been_pushed(id));
        }
    }

    #[test]
    fn n112_1_no_duplicate_push() {
        let (store, _ids) = make_store_with_evidence(5);
        let gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        let first = push.select_for_push(&store, &gossip);
        assert_eq!(first.len(), 5);

        let second = push.select_for_push(&store, &gossip);
        assert_eq!(second.len(), 0, "No evidence should be pushed twice");
    }

    #[test]
    fn n112_1_respects_batch_size() {
        let (store, _ids) = make_store_with_evidence(20);
        let gossip = EvidenceGossip::new();
        let config = EvidencePushConfig {
            max_batch_size: 5,
            ..Default::default()
        };
        let mut push = EvidencePush::new(config);

        let selected = push.select_for_push(&store, &gossip);
        assert_eq!(selected.len(), 5, "Must respect max_batch_size");
    }

    #[test]
    fn n112_1_mark_pushed_individual() {
        let (store, ids) = make_store_with_evidence(3);
        let gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        // Mark one as pushed without selection
        push.mark_pushed(&ids[0]);
        assert!(push.has_been_pushed(&ids[0]));
        assert!(!push.has_been_pushed(&ids[1]));

        let selected = push.select_for_push(&store, &gossip);
        assert_eq!(
            selected.len(),
            2,
            "Only unpushed evidence should be selected"
        );
    }

    #[test]
    fn n112_1_reset_clears_tracking() {
        let (store, _ids) = make_store_with_evidence(3);
        let gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        push.select_for_push(&store, &gossip);
        assert_eq!(push.pushed_count(), 3);

        push.reset();
        assert_eq!(push.pushed_count(), 0);

        let selected = push.select_for_push(&store, &gossip);
        assert_eq!(
            selected.len(),
            3,
            "After reset, all evidence should be selectable again"
        );
    }
}
