// ============================================================================
// N112.4 — End-to-End Push Evidence Sync
// ============================================================================
// Node A creates evidence, selects it for push, sends it via transport,
// and Node B receives and processes it — all without a
// MissingEvidenceRequest/Response cycle.  The certificate is accepted
// immediately after the push.
// ============================================================================

use amun_consensus_network::{
    process_incoming_evidence_push, validate_certificate_evidence, EvidenceCount, EvidenceGossip,
    EvidencePush, EvidenceRecord, EvidenceStore, EvidenceType, EvidenceValidationResult,
    SlashingCertificate, ValidatorStatus,
};

fn make_evidence(validator_id: [u8; 32], height: u64, seed: u8) -> EvidenceRecord {
    EvidenceRecord::new(
        validator_id,
        height,
        EvidenceType::DoubleVote,
        1000 * height,
        vec![seed; 32],
    )
}

fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
    SlashingCertificate::from_slash_result(
        validator_id,
        30,
        evidence_ids,
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

// ============================================================================
// N112.4 GATEKEEPER — Push sync eliminates the need for MissingEvidenceRequest
// ============================================================================
#[test]
fn n112_4_push_sync_end_to_end() {
    let validator_id = [0x42; 32];

    // ========================================================================
    // Phase 1: Node A creates evidence and prepares push
    // ========================================================================
    let mut store_a = EvidenceStore::new();
    let gossip_a = EvidenceGossip::new();
    let mut push_a = EvidencePush::default();

    // Create 4 evidence records
    let ev1 = make_evidence(validator_id, 1, 1);
    let ev2 = make_evidence(validator_id, 2, 2);
    let ev3 = make_evidence(validator_id, 3, 3);
    let ev4 = make_evidence(validator_id, 4, 4);

    store_a.store_evidence(ev1.clone());
    store_a.store_evidence(ev2.clone());
    store_a.store_evidence(ev3.clone());
    store_a.store_evidence(ev4.clone());

    // Select evidence for push
    let selected = push_a.select_for_push(&store_a, &gossip_a);
    assert_eq!(
        selected.len(),
        4,
        "All 4 evidence records must be selected for push"
    );

    // Serialize the selected records for transport
    let serialized: Vec<Vec<u8>> = selected
        .iter()
        .map(|(record, _)| postcard::to_stdvec(record).unwrap())
        .collect();

    // Create certificate referencing these evidence IDs
    let cert = make_certificate(
        validator_id,
        vec![
            ev1.evidence_id,
            ev2.evidence_id,
            ev3.evidence_id,
            ev4.evidence_id,
        ],
    );

    // ========================================================================
    // Phase 2: Node B receives the push (no prior evidence)
    // ========================================================================
    let mut store_b = EvidenceStore::new();
    let mut gossip_b = EvidenceGossip::new();
    let mut push_b = EvidencePush::default();

    // Before push: certificate should be rejected
    let result_before = validate_certificate_evidence(&cert, &store_b);
    match result_before {
        EvidenceValidationResult::MissingEvidence { missing_ids } => {
            assert_eq!(
                missing_ids.len(),
                4,
                "All 4 evidence IDs should be missing before push"
            );
        }
        _ => panic!("Expected MissingEvidence before push"),
    }

    // ========================================================================
    // Phase 3: Node B processes the push
    // ========================================================================
    let result =
        process_incoming_evidence_push(&serialized, &mut store_b, &mut gossip_b, &mut push_b)
            .unwrap();

    assert_eq!(result.total_received, 4);
    assert_eq!(result.newly_imported, 4);
    assert_eq!(result.duplicates, 0);
    assert_eq!(store_b.len(), 4, "Node B must now have 4 evidence records");

    // ========================================================================
    // Phase 4: Node B validates the certificate — ACCEPTED without pull
    // ========================================================================
    let result_after = validate_certificate_evidence(&cert, &store_b);
    assert_eq!(
        result_after,
        EvidenceValidationResult::AllPresent,
        "N112.4 FAIL: After push, certificate must be accepted without MissingEvidenceRequest"
    );

    // Verify gossip was updated
    assert!(gossip_b.has_evidence(&ev1.evidence_id));
    assert!(gossip_b.has_evidence(&ev2.evidence_id));
    assert!(gossip_b.has_evidence(&ev3.evidence_id));
    assert!(gossip_b.has_evidence(&ev4.evidence_id));

    // Verify push tracking
    assert!(push_b.has_been_pushed(&ev1.evidence_id));
    assert_eq!(push_b.pushed_count(), 4);

    eprintln!("N112.4 GATEKEEPER PASSED: push sync eliminated MissingEvidenceRequest cycle");
}

// ============================================================================
// N112.4: Push of already-known evidence is harmless (duplicates counted)
// ============================================================================
#[test]
fn n112_4_push_duplicate_is_harmless() {
    let mut store = EvidenceStore::new();
    let mut gossip = EvidenceGossip::new();
    let mut push = EvidencePush::default();

    let ev = make_evidence([0x42; 32], 1, 1);
    let serialized = vec![postcard::to_stdvec(&ev).unwrap()];

    // First push: imported (evidence is new)
    let r1 =
        process_incoming_evidence_push(&serialized, &mut store, &mut gossip, &mut push).unwrap();
    assert_eq!(r1.newly_imported, 1);
    assert_eq!(store.len(), 1);

    // Second push of same evidence: duplicate, not re-imported
    let r2 =
        process_incoming_evidence_push(&serialized, &mut store, &mut gossip, &mut push).unwrap();
    assert_eq!(r2.duplicates, 1);
    assert_eq!(r2.newly_imported, 0);
    assert_eq!(store.len(), 1, "Duplicate push must not grow store");
}

// ============================================================================
// N112.4: Certificate validated immediately after push (no pull cycle)
// ============================================================================
#[test]
fn n112_4_certificate_immediately_accepted_after_push() {
    let validator_id = [0xAA; 32];

    // Node A: evidence + push
    let mut store_a = EvidenceStore::new();
    let gossip_a = EvidenceGossip::new();
    let mut push_a = EvidencePush::default();
    let ev = make_evidence(validator_id, 1, 1);
    store_a.store_evidence(ev.clone());
    let selected = push_a.select_for_push(&store_a, &gossip_a);
    let serialized: Vec<Vec<u8>> = selected
        .iter()
        .map(|(r, _)| postcard::to_stdvec(r).unwrap())
        .collect();
    let cert = make_certificate(validator_id, vec![ev.evidence_id]);

    // Node B: empty, receives push
    let mut store_b = EvidenceStore::new();
    let mut gossip_b = EvidenceGossip::new();
    let mut push_b = EvidencePush::default();

    // Push received
    process_incoming_evidence_push(&serialized, &mut store_b, &mut gossip_b, &mut push_b).unwrap();

    // Certificate accepted — NO MissingEvidenceRequest needed
    let result = validate_certificate_evidence(&cert, &store_b);
    assert_eq!(result, EvidenceValidationResult::AllPresent);

    eprintln!("N112.4: Push-first flow verified — no pull cycle needed");
}
