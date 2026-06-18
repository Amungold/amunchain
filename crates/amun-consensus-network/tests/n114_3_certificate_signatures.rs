// ============================================================================
// N114.3 — SlashingCertificate Signature Tests
// ============================================================================
use amun_consensus_network::{
    SlashingCertificate, EvidenceCount, EvidenceType, ValidatorStatus,
};
use ed25519_dalek::SigningKey;

fn make_unsigned_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
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

// ============================================================================
// Gatekeeper 1: Signed certificate verifies successfully
// ============================================================================
#[test]
fn n114_3_signed_certificate_verifies() {
    let mut cert = make_unsigned_certificate([0x42; 32]);
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);

    cert.sign(&signing_key);

    assert_eq!(cert.signer_public_key, signing_key.verifying_key().to_bytes());
    assert_ne!(cert.signature, [0u8; 64], "Signature must not be zero");
    assert!(cert.verify_signature().is_ok(), "N114.3 FAIL: Signed certificate must verify");
}

// ============================================================================
// Gatekeeper 2: Unsigned certificate is rejected
// ============================================================================
#[test]
fn n114_3_unsigned_certificate_rejected() {
    let cert = make_unsigned_certificate([0x42; 32]);

    assert_eq!(cert.signature, [0u8; 64], "Unsigned cert must have zero signature");
    assert!(cert.verify_signature().is_err(),
        "N114.3 FAIL: Unsigned certificate must be rejected");
}

// ============================================================================
// Gatekeeper 3: Tampered certificate is rejected
// ============================================================================
#[test]
fn n114_3_tampered_certificate_rejected() {
    let mut cert = make_unsigned_certificate([0x42; 32]);
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    cert.sign(&signing_key);

    // Tamper with amount_slashed after signing
    cert.amount_slashed = 1; // Changed from 15000 to 1

    assert!(cert.verify_signature().is_err(),
        "N114.3 FAIL: Tampered certificate must be rejected");
}

// ============================================================================
// Gatekeeper 4: Wrong public key is rejected
// ============================================================================
#[test]
fn n114_3_wrong_public_key_rejected() {
    let mut cert = make_unsigned_certificate([0x42; 32]);
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    cert.sign(&signing_key);

    // Replace signer_public_key with a different key
    cert.signer_public_key = [0xFF; 32];

    assert!(cert.verify_signature().is_err(),
        "N114.3 FAIL: Wrong public key must be rejected");
}

// ============================================================================
// Additional: Signature changes when certificate content changes
// ============================================================================
#[test]
fn n114_3_signature_changes_with_certificate_content() {
    let mut cert1 = make_unsigned_certificate([0x42; 32]);
    let mut cert2 = make_unsigned_certificate([0x42; 32]);
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);

    // Modify cert2 before signing
    cert2.amount_slashed = 9999;

    cert1.sign(&signing_key);
    cert2.sign(&signing_key);

    assert_ne!(cert1.signature, cert2.signature,
        "N114.3 FAIL: Different content must produce different signatures");
    assert!(cert1.verify_signature().is_ok());
    assert!(cert2.verify_signature().is_ok());
}

// ============================================================================
// Roundtrip: Serialization preserves signature fields
// ============================================================================
#[test]
fn n114_3_signed_certificate_roundtrip() {
    let mut cert = make_unsigned_certificate([0x42; 32]);
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    cert.sign(&signing_key);

    let encoded = postcard::to_stdvec(&cert).unwrap();
    let decoded: SlashingCertificate = postcard::from_bytes(&encoded).unwrap();

    assert_eq!(decoded.signer_public_key, cert.signer_public_key);
    assert_eq!(decoded.signature, cert.signature);
    assert!(decoded.verify_signature().is_ok(),
        "N114.3 FAIL: Roundtripped certificate must verify");
}
