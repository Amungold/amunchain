// ============================================================================
// N111.7 — End-to-End Evidence Sync Across Two Simulated Nodes
// ============================================================================
// Node A creates evidence, produces a SlashingCertificate, and sends it
// to Node B. Node B discovers missing evidence, requests it, receives it,
// verifies it, and finally accepts the certificate.
//
// This is the integration gate for N111.
// ============================================================================

use amun_consensus_network::{
    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
    CertificateGossip, EvidenceAnnouncement, EvidenceCount, EvidenceGossip, EvidenceRecord,
    EvidenceStore, EvidenceType, EvidenceValidationResult, SlashingCertificate, ValidatorStatus,
};

/// Helper: create a valid SlashingCertificate that references specific evidence IDs.
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

/// Helper: create a realistic EvidenceRecord.
fn make_evidence_record(
    validator_id: [u8; 32],
    height: u64,
    evidence_type: EvidenceType,
    seed: u8,
) -> EvidenceRecord {
    EvidenceRecord::new(
        validator_id,
        height,
        evidence_type,
        1000 * height,
        vec![seed; 32],
    )
}

// ============================================================================
// N111.7 GATEKEEPER — Full evidence sync pipeline
// ============================================================================
#[test]
fn n111_7_evidence_sync_end_to_end() {
    let validator_id = [0x42; 32];

    // ========================================================================
    // Phase 1: Node A creates evidence and a certificate
    // ========================================================================
    let mut store_a = EvidenceStore::new();
    let mut gossip_a = EvidenceGossip::new();

    // Create 3 evidence records
    let ev1 = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
    let ev2 = make_evidence_record(validator_id, 2, EvidenceType::DoubleVote, 2);
    let ev3 = make_evidence_record(validator_id, 3, EvidenceType::DoubleVote, 3);
    let id1 = ev1.evidence_id;
    let id2 = ev2.evidence_id;
    let id3 = ev3.evidence_id;

    store_a.store_evidence(ev1.clone());
    store_a.store_evidence(ev2.clone());
    store_a.store_evidence(ev3.clone());

    // Gossip announcements for all 3
    for ev in &[ev1, ev2, ev3] {
        gossip_a.receive_announcement(EvidenceAnnouncement {
            evidence_id: ev.evidence_id,
            validator_id: ev.validator_id,
            evidence_type: ev.evidence_type.clone(),
            height: ev.height,
            timestamp: ev.timestamp,
        });
    }

    // Create certificate referencing all 3 evidence IDs
    let cert = make_certificate(validator_id, vec![id1, id2, id3]);

    // Node A validates locally — all evidence present
    let result_a = validate_certificate_evidence(&cert, &store_a);
    assert_eq!(
        result_a,
        EvidenceValidationResult::AllPresent,
        "N111.7 FAIL: Node A must have all evidence present"
    );

    // ========================================================================
    // Phase 2: Node B receives the certificate but has NO evidence
    // ========================================================================
    let mut store_b = EvidenceStore::new();
    let _gossip_b = EvidenceGossip::new();

    // Node B tries to validate — evidence is missing
    let result_b = validate_certificate_evidence(&cert, &store_b);
    let missing_ids = match result_b {
        EvidenceValidationResult::MissingEvidence { missing_ids } => missing_ids,
        _ => panic!("N111.7 FAIL: Node B must detect missing evidence"),
    };
    assert_eq!(
        missing_ids.len(),
        3,
        "N111.7 FAIL: Node B must report 3 missing evidence IDs"
    );

    // ========================================================================
    // Phase 3: Node B requests missing evidence from Node A
    // ========================================================================
    let request = build_missing_evidence_request([0xBB; 32], missing_ids);
    assert_eq!(request.evidence_ids.len(), 3);

    // Node A responds by sending the serialized evidence records
    let mut response_data = Vec::new();
    for id in &request.evidence_ids {
        if let Some(record) = store_a.get_by_id(id) {
            response_data.push(postcard::to_stdvec(record).unwrap());
        }
    }
    let response = amun_consensus_network::MissingEvidenceResponse {
        responder_id: [0xAA; 32],
        evidence_data: response_data,
    };

    // ========================================================================
    // Phase 4: Node B processes the response and imports evidence
    // ========================================================================
    let imported = process_evidence_response(&response, &mut store_b).unwrap();
    assert_eq!(
        imported, 3,
        "N111.7 FAIL: Must import exactly 3 evidence records"
    );

    // Verify evidence is now in store_b
    assert!(store_b.get_by_id(&id1).is_some());
    assert!(store_b.get_by_id(&id2).is_some());
    assert!(store_b.get_by_id(&id3).is_some());

    // ========================================================================
    // Phase 5: Node B re-validates — all evidence now present
    // ========================================================================
    let result_b_after = validate_certificate_evidence(&cert, &store_b);
    assert_eq!(
        result_b_after,
        EvidenceValidationResult::AllPresent,
        "N111.7 FAIL: After sync, Node B must have all evidence present"
    );

    eprintln!("N111.7 GATEKEEPER PASSED: full evidence sync pipeline works end-to-end");
}

// ============================================================================
// N111.7: Certificate gossiped between nodes
// ============================================================================
#[test]
fn n111_7_certificate_gossip_between_nodes() {
    let validator_id = [0x42; 32];

    // Node A: create certificate with evidence
    let mut store_a = EvidenceStore::new();
    let ev = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
    let ev_id = ev.evidence_id;
    store_a.store_evidence(ev);

    let cert = make_certificate(validator_id, vec![ev_id]);
    let cert_hash = cert.certificate_hash;

    // Node A: store in gossip
    let mut cert_gossip_a = CertificateGossip::new();
    cert_gossip_a.receive_certificate(cert.clone()).unwrap();

    // Node B: receive certificate (but no evidence yet)
    let mut store_b = EvidenceStore::new();
    let mut cert_gossip_b = CertificateGossip::new();

    // Certificate itself is valid and can be stored
    cert_gossip_b.receive_certificate(cert.clone()).unwrap();
    assert!(cert_gossip_b.certificates.contains_key(&cert_hash));

    // But evidence validation still fails
    let result = validate_certificate_evidence(&cert, &store_b);
    assert_ne!(result, EvidenceValidationResult::AllPresent);

    // After syncing evidence, validation passes
    let ev_encoded = postcard::to_stdvec(&store_a.get_by_id(&ev_id).unwrap()).unwrap();
    let response = amun_consensus_network::MissingEvidenceResponse {
        responder_id: [0xAA; 32],
        evidence_data: vec![ev_encoded],
    };
    process_evidence_response(&response, &mut store_b).unwrap();

    let result_after = validate_certificate_evidence(&cert, &store_b);
    assert_eq!(result_after, EvidenceValidationResult::AllPresent);

    eprintln!("N111.7: Certificate gossip + evidence sync complete");
}
