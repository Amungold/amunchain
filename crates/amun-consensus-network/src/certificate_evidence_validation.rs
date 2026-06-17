// ============================================================================
// N111.6 — Certificate Evidence Validation
// ============================================================================
// Enforces that a SlashingCertificate is only accepted when ALL evidence
// IDs it references are present in the local EvidenceStore.
//
// This closes the critical limitation documented in v0.4.0-N110:
// "Evidence propagation is not yet network-wide."
//
// After N111.6:
//   - Certificate with missing evidence → rejected
//   - Missing evidence → MissingEvidenceRequest sent to peers
//   - Evidence received → certificate retried → accepted
// ============================================================================

use crate::evidence_store::EvidenceStore;
use crate::messages::{MissingEvidenceRequest, MissingEvidenceResponse};
use crate::slashing_certificate::SlashingCertificate;

/// N111.6: Result of validating a certificate's evidence availability.
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceValidationResult {
    /// All evidence IDs are present locally — certificate can be accepted.
    AllPresent,
    /// Some evidence IDs are missing — request them from peers.
    MissingEvidence { missing_ids: Vec<[u8; 32]> },
}

/// N111.6: Check that every evidence_id referenced by a certificate
/// exists in the local EvidenceStore.
pub fn validate_certificate_evidence(
    cert: &SlashingCertificate,
    evidence_store: &EvidenceStore,
) -> EvidenceValidationResult {
    let mut missing = Vec::new();

    for evidence_id in &cert.evidence_ids {
        if evidence_store.get_by_id(evidence_id).is_none() {
            missing.push(*evidence_id);
        }
    }

    if missing.is_empty() {
        EvidenceValidationResult::AllPresent
    } else {
        EvidenceValidationResult::MissingEvidence {
            missing_ids: missing,
        }
    }
}

/// N111.6: Build a MissingEvidenceRequest for the given evidence IDs.
pub fn build_missing_evidence_request(
    requester_id: [u8; 32],
    missing_ids: Vec<[u8; 32]>,
) -> MissingEvidenceRequest {
    MissingEvidenceRequest {
        requester_id,
        evidence_ids: missing_ids,
    }
}

/// N111.6: Process a MissingEvidenceResponse by extracting the evidence
/// records from the serialized data.
/// Returns the number of evidence records successfully imported.
pub fn process_evidence_response(
    response: &MissingEvidenceResponse,
    evidence_store: &mut EvidenceStore,
) -> Result<usize, String> {
    let mut imported = 0;
    for data in &response.evidence_data {
        let record: crate::evidence_store::EvidenceRecord = postcard::from_bytes(data)
            .map_err(|e| format!("Failed to decode evidence record: {}", e))?;
        if evidence_store.store_evidence(record) {
            imported += 1;
        }
    }
    Ok(imported)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::{EvidenceRecord, EvidenceStore, EvidenceType};
    use crate::misbehavior_registry::ValidatorStatus;
    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};

    fn make_cert_with_evidence_ids(ids: Vec<[u8; 32]>) -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            [0x42; 32],
            30,
            ids,
            vec![EvidenceCount {
                evidence_type: EvidenceType::DoubleVote,
                count: 3,
                weight: 30,
            }],
            1500,
            15000,
            85000,
            3,
            ValidatorStatus::SlashEligible,
            100,
        )
    }

    /// N111.6 GATEKEEPER: Certificate rejected when evidence is missing
    #[test]
    fn n111_6_certificate_rejected_when_evidence_missing() {
        let store = EvidenceStore::new();
        let cert = make_cert_with_evidence_ids(vec![[0xA1; 32], [0xA2; 32]]);

        let result = validate_certificate_evidence(&cert, &store);
        match result {
            EvidenceValidationResult::MissingEvidence { missing_ids } => {
                assert_eq!(missing_ids.len(), 2);
                assert!(missing_ids.contains(&[0xA1; 32]));
                assert!(missing_ids.contains(&[0xA2; 32]));
            }
            _ => panic!("Expected MissingEvidence, got {:?}", result),
        }
    }

    /// N111.6: Certificate accepted when all evidence is present
    #[test]
    fn n111_6_certificate_accepted_after_evidence_sync() {
        let mut store = EvidenceStore::new();

        // Store the required evidence
        let e1 = EvidenceRecord::new([0x42; 32], 1, EvidenceType::DoubleVote, 1000, vec![1]);
        let e2 = EvidenceRecord::new([0x42; 32], 2, EvidenceType::DoubleVote, 2000, vec![2]);
        let id1 = e1.evidence_id;
        let id2 = e2.evidence_id;
        store.store_evidence(e1);
        store.store_evidence(e2);

        let cert = make_cert_with_evidence_ids(vec![id1, id2]);

        let result = validate_certificate_evidence(&cert, &store);
        assert_eq!(result, EvidenceValidationResult::AllPresent);
    }

    /// N111.6: Partial evidence → still rejected
    #[test]
    fn n111_6_partial_evidence_still_rejected() {
        let mut store = EvidenceStore::new();
        let e1 = EvidenceRecord::new([0x42; 32], 1, EvidenceType::DoubleVote, 1000, vec![1]);
        let id1 = e1.evidence_id;
        store.store_evidence(e1);

        let cert = make_cert_with_evidence_ids(vec![id1, [0xFF; 32]]);

        let result = validate_certificate_evidence(&cert, &store);
        match result {
            EvidenceValidationResult::MissingEvidence { missing_ids } => {
                assert_eq!(missing_ids.len(), 1);
                assert!(missing_ids.contains(&[0xFF; 32]));
            }
            _ => panic!("Expected MissingEvidence"),
        }
    }

    /// N111.6: Build missing evidence request
    #[test]
    fn n111_6_build_missing_evidence_request() {
        let req = build_missing_evidence_request([0xAA; 32], vec![[0xB1; 32], [0xB2; 32]]);
        assert_eq!(req.requester_id, [0xAA; 32]);
        assert_eq!(req.evidence_ids.len(), 2);
    }

    /// N111.6: Process evidence response imports records
    #[test]
    fn n111_6_process_evidence_response() {
        let mut store = EvidenceStore::new();
        let evidence = EvidenceRecord::new(
            [0x42; 32],
            5,
            EvidenceType::StateRootMismatch,
            5000,
            vec![7, 8, 9],
        );
        let encoded = postcard::to_stdvec(&evidence).unwrap();

        let response = MissingEvidenceResponse {
            responder_id: [0xCC; 32],
            evidence_data: vec![encoded],
        };

        let imported = process_evidence_response(&response, &mut store).unwrap();
        assert_eq!(imported, 1);
        assert_eq!(store.len(), 1);
    }
}
