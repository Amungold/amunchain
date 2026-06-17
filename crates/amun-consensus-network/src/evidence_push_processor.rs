// N112.3 — Automatic Peer Evidence Propagation
// ==============================================
// When a node receives an EvidencePushMessage, it automatically
// imports the evidence records into its EvidenceStore without
// requiring a MissingEvidenceRequest.  This eliminates the pull
// dependency for evidence that peers proactively push.

use crate::evidence_gossip::EvidenceGossip;
use crate::evidence_push::EvidencePush;
use crate::evidence_store::{EvidenceRecord, EvidenceStore};

/// Result of processing an incoming evidence push.
#[derive(Debug, Clone, PartialEq)]
pub struct PushProcessResult {
    pub total_received: usize,
    pub newly_imported: usize,
    pub duplicates: usize,
}

/// N112.3: Process an incoming EvidencePushMessage.
/// Decodes each serialized EvidenceRecord and stores it locally.
/// Records that already exist (deduplication) are counted but not
/// re-stored.
pub fn process_incoming_evidence_push(
    serialized_records: &[Vec<u8>],
    evidence_store: &mut EvidenceStore,
    gossip: &mut EvidenceGossip,
    push: &mut EvidencePush,
) -> Result<PushProcessResult, String> {
    let mut result = PushProcessResult {
        total_received: serialized_records.len(),
        newly_imported: 0,
        duplicates: 0,
    };

    for data in serialized_records {
        let record: EvidenceRecord = postcard::from_bytes(data)
            .map_err(|e| format!("Failed to decode evidence record: {}", e))?;

        // Store in EvidenceStore
        if evidence_store.store_evidence(record.clone()) {
            result.newly_imported += 1;

            // Update gossip tracking
            gossip.receive_announcement(crate::evidence_gossip::EvidenceAnnouncement {
                evidence_id: record.evidence_id,
                validator_id: record.validator_id,
                evidence_type: record.evidence_type.clone(),
                height: record.height,
                timestamp: record.timestamp,
            });

            // Mark as pushed so we don't re-push it ourselves
            push.mark_pushed(&record.evidence_id);
        } else {
            result.duplicates += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_gossip::EvidenceGossip;
    use crate::evidence_push::EvidencePush;
    use crate::evidence_store::{EvidenceRecord, EvidenceStore, EvidenceType};

    fn make_evidence_record(id: u8) -> EvidenceRecord {
        EvidenceRecord::new(
            [0x42; 32],
            id as u64,
            EvidenceType::DoubleVote,
            1000 * (id as u64),
            vec![id; 8],
        )
    }

    #[test]
    fn n112_3_process_push_imports_all_records() {
        let mut store = EvidenceStore::new();
        let mut gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        let records: Vec<_> = (1..=3).map(make_evidence_record).collect();
        let serialized: Vec<_> = records
            .iter()
            .map(|r| postcard::to_stdvec(r).unwrap())
            .collect();

        let result =
            process_incoming_evidence_push(&serialized, &mut store, &mut gossip, &mut push)
                .unwrap();

        assert_eq!(result.total_received, 3);
        assert_eq!(result.newly_imported, 3);
        assert_eq!(result.duplicates, 0);
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn n112_3_duplicate_records_counted_but_not_reimported() {
        let mut store = EvidenceStore::new();
        let mut gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        let record = make_evidence_record(1);
        let serialized = postcard::to_stdvec(&record).unwrap();

        // First push
        let result1 = process_incoming_evidence_push(
            std::slice::from_ref(&serialized),
            &mut store,
            &mut gossip,
            &mut push,
        )
        .unwrap();
        assert_eq!(result1.newly_imported, 1);
        assert_eq!(store.len(), 1);

        // Second push of same record
        let result2 =
            process_incoming_evidence_push(&[serialized], &mut store, &mut gossip, &mut push)
                .unwrap();
        assert_eq!(result2.duplicates, 1);
        assert_eq!(result2.newly_imported, 0);
        assert_eq!(store.len(), 1, "Store must not grow on duplicates");
    }

    #[test]
    fn n112_3_push_updates_gossip_tracking() {
        let mut store = EvidenceStore::new();
        let mut gossip = EvidenceGossip::new();
        let mut push = EvidencePush::default();

        let record = make_evidence_record(1);
        let serialized = postcard::to_stdvec(&record).unwrap();

        process_incoming_evidence_push(&[serialized], &mut store, &mut gossip, &mut push).unwrap();

        assert!(gossip.has_evidence(&record.evidence_id));
        assert!(push.has_been_pushed(&record.evidence_id));
    }
}
