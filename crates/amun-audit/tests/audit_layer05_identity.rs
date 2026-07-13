#[cfg(test)]
mod audit_identity {
    use amun_snapshot_engine::ConstitutionalIdentity;

    // CONST-ID-001: Different constitutional hash = different identity
    #[test]
    fn id001_different_hash_different_identity() {
        let id1 = ConstitutionalIdentity::new([0x01u8; 32]);
        let id2 = ConstitutionalIdentity::new([0x02u8; 32]);
        assert_ne!(
            id1.identity_hash, id2.identity_hash,
            "CONST-ID-001 VIOLATION: Different constitutions must differ"
        );
    }

    // CONST-ID-002: Identity must self-verify
    #[test]
    fn id002_identity_self_verification() {
        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
        assert!(
            id.verify(),
            "CONST-ID-002 VIOLATION: Identity must self-verify"
        );
    }

    // CONST-ID-003: Encode/decode roundtrip preserves identity
    #[test]
    fn id003_encode_decode_roundtrip() {
        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
        let encoded = id.encode();
        let decoded = ConstitutionalIdentity::decode(&encoded)
            .expect("CONST-ID-003: Must decode successfully");
        assert_eq!(
            id.identity_hash, decoded.identity_hash,
            "CONST-ID-003 VIOLATION: Roundtrip changed identity hash"
        );
        assert!(
            decoded.verify(),
            "CONST-ID-003 VIOLATION: Decoded must verify"
        );
    }

    // CONST-ID-004: Tampered identity fails verification
    #[test]
    fn id004_tampered_identity_detection() {
        let id = ConstitutionalIdentity::new([0x13u8; 32]);
        let mut encoded = id.encode();
        if encoded.len() > 32 {
            encoded[32] ^= 0xFF;
        }
        if let Some(decoded) = ConstitutionalIdentity::decode(&encoded) {
            assert!(
                !decoded.verify(),
                "CONST-ID-004 VIOLATION: Tampered identity must fail verification"
            );
        }
    }
}
