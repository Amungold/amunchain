#[cfg(test)]
mod constitutional_tests {
    use amun_snapshot_engine::{
        CompatibilityEngine, CompatibilityLevel, ConstitutionalHash, ConstitutionalIdentity,
        ConstitutionalRelationship, TransitionClassifier,
    };

    #[test]
    fn identical_identities_match() {
        let hash = [0x42u8; 32];
        let id1 = ConstitutionalIdentity::new(hash);
        let id2 = ConstitutionalIdentity::new(hash);
        assert!(id1.matches(&id2));
        assert_eq!(id1.identity_hash, id2.identity_hash);
    }

    #[test]
    fn different_constitutions_produce_different_identities() {
        let id1 = ConstitutionalIdentity::new([0x01u8; 32]);
        let id2 = ConstitutionalIdentity::new([0x02u8; 32]);
        assert!(!id1.matches(&id2));
        assert_ne!(id1.identity_hash, id2.identity_hash);
    }

    #[test]
    fn identity_self_verification() {
        let id = ConstitutionalIdentity::new([0xAAu8; 32]);
        assert!(id.verify());
    }

    #[test]
    fn fully_compatible_when_identical() {
        let id = ConstitutionalIdentity::new([0x11u8; 32]);
        let matrix = CompatibilityEngine::compute(&id, &id);
        assert_eq!(matrix.level, CompatibilityLevel::FullyCompatible);
    }

    #[test]
    fn different_constitutional_hash_allows_readonly() {
        let id1 = ConstitutionalIdentity::new([0xAAu8; 32]);
        let id2 = ConstitutionalIdentity::new([0xBBu8; 32]);
        let matrix = CompatibilityEngine::compute(&id1, &id2);
        assert_eq!(matrix.level, CompatibilityLevel::ReadOnlyCompatible);
    }

    #[test]
    fn truly_incompatible_when_structural_universe_differs() {
        let mut id1 = ConstitutionalIdentity::new([0xCCu8; 32]);
        let mut id2 = ConstitutionalIdentity::new([0xDDu8; 32]);
        id1.canonical_empty_root = [0x01u8; 32];
        id2.canonical_empty_root = [0xFFu8; 32];
        let matrix = CompatibilityEngine::compute(&id1, &id2);
        assert_eq!(matrix.level, CompatibilityLevel::Incompatible);
    }

    #[test]
    fn can_sync_only_when_compatible() {
        let id = ConstitutionalIdentity::new([0xCCu8; 32]);
        assert!(CompatibilityEngine::can_sync(&id, &id));
    }

    #[test]
    fn classify_identical_as_identical() {
        let id = ConstitutionalIdentity::new([0xDDu8; 32]);
        let rel = TransitionClassifier::classify(&id, &id);
        assert!(matches!(rel, ConstitutionalRelationship::Identical));
    }

    #[test]
    fn sync_possible_between_identical() {
        let id = ConstitutionalIdentity::new([0xEEu8; 32]);
        assert!(TransitionClassifier::can_sync(&id, &id));
    }

    #[test]
    fn constitutional_hash_is_deterministic() {
        let h1 = ConstitutionalHash::compute(
            "const",
            "spec",
            "replay",
            "snapshot",
            "validity",
            "traversal",
        );
        let h2 = ConstitutionalHash::compute(
            "const",
            "spec",
            "replay",
            "snapshot",
            "validity",
            "traversal",
        );
        assert_eq!(h1, h2);
    }

    #[test]
    fn constitutional_hash_changes_with_different_input() {
        let h1 = ConstitutionalHash::compute(
            "const_a",
            "spec",
            "replay",
            "snapshot",
            "validity",
            "traversal",
        );
        let h2 = ConstitutionalHash::compute(
            "const_b",
            "spec",
            "replay",
            "snapshot",
            "validity",
            "traversal",
        );
        assert_ne!(h1, h2);
    }

    #[test]
    fn identity_encode_decode_roundtrip() {
        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
        let encoded = id.encode();
        let decoded = ConstitutionalIdentity::decode(&encoded).unwrap();
        assert_eq!(id.identity_hash, decoded.identity_hash);
        assert!(decoded.verify());
    }

    #[test]
    fn tampered_identity_fails_verification() {
        let id = ConstitutionalIdentity::new([0x13u8; 32]);
        let mut encoded = id.encode();
        if encoded.len() > 64 {
            encoded[64] ^= 0xFF;
        }
        let decoded = ConstitutionalIdentity::decode(&encoded).unwrap();
        assert!(!decoded.verify());
    }
}
