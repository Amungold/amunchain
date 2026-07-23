// ============================================================================
// N110.4b — Consensus Verification of SlashingCertificates
// ============================================================================
use amun_block_builder::{Block, BlockBuilder};
use amun_consensus_network::{EvidenceCount, EvidenceType, SlashingCertificate, ValidatorStatus};
use amun_mempool::Mempool;

/// Create a valid slashing certificate for testing
fn make_valid_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
    SlashingCertificate::from_slash_result(
        validator_id,
        30,
        vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
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

/// Build a test block with given certificates
fn build_test_block(certs: Vec<SlashingCertificate>) -> Block {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    builder.build_block_with_certificates(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000, certs, [0u8; 32], [0u8; 32])
}

// ============================================================================
// N110.4b Tests
// ============================================================================

#[test]
fn n110_4b_valid_certificates_pass_verification() {
    let cert = make_valid_certificate([0x42; 32]);
    let block = build_test_block(vec![cert]);
    assert!(
        block.verify_slashing_certificates().is_ok(),
        "N110.4b FAIL: Valid certificates should pass verification"
    );
}

#[test]
fn n110_4b_tampered_certificate_rejected() {
    let mut cert = make_valid_certificate([0x42; 32]);
    cert.amount_slashed = 0; // Tamper with amount
                             // Recompute hash to keep it consistent? No, we want hash mismatch.
                             // verify() should catch either amount_slashed=0 or hash mismatch.
    let block = build_test_block(vec![cert]);
    let result = block.verify_slashing_certificates();
    assert!(
        result.is_err(),
        "N110.4b FAIL: Tampered certificate must be rejected"
    );
    assert!(
        result.unwrap_err().contains("Certificate 0 invalid"),
        "Error should reference certificate index"
    );
}

#[test]
fn n110_4b_too_many_certificates_rejected() {
    let mut certs = Vec::new();
    for i in 0..11 {
        certs.push(make_valid_certificate([i as u8; 32]));
    }
    let block = build_test_block(certs);
    let result = block.verify_slashing_certificates();
    assert!(
        result.is_err(),
        "N110.4b FAIL: Too many certificates must be rejected"
    );
    assert!(
        result
            .unwrap_err()
            .contains("Too many slashing certificates"),
        "Error should mention max certificates"
    );
}

#[test]
fn n110_4b_empty_evidence_ids_rejected() {
    let cert = SlashingCertificate::from_slash_result(
        [0x42; 32],
        30,
        vec![], // Empty evidence IDs
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
    );
    let block = build_test_block(vec![cert]);
    let result = block.verify_slashing_certificates();
    assert!(
        result.is_err(),
        "N110.4b FAIL: Certificate with no evidence IDs must be rejected"
    );
}

#[test]
fn n110_4b_hash_mismatch_rejected() {
    let mut cert = make_valid_certificate([0x42; 32]);
    // Tamper certificate_hash without changing content
    cert.certificate_hash = [0xFF; 32];
    let block = build_test_block(vec![cert]);
    let result = block.verify_slashing_certificates();
    assert!(
        result.is_err(),
        "N110.4b FAIL: Certificate with mismatched hash must be rejected"
    );
    assert!(
        result.unwrap_err().contains("hash mismatch"),
        "Error should mention hash mismatch"
    );
}

#[test]
fn n110_4b_empty_certificates_list_passes() {
    let block = build_test_block(vec![]);
    assert!(
        block.verify_slashing_certificates().is_ok(),
        "N110.4b FAIL: Block with no certificates should pass verification"
    );
}
