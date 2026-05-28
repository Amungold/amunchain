#[cfg(test)]
mod audit_snapshot {
    use amun_snapshot_engine::{
        CompatibilityEngine, CompatibilityLevel, ConstitutionalIdentity, SnapshotHeader,
        SnapshotManifest,
    };

    // CONST-SNAP-001: Snapshot magic bytes are correct
    #[test]
    fn snap001_magic_bytes() {
        let header = SnapshotHeader {
            magic: *b"AMSN",
            snapshot_version: 1,
            protocol_version: 1,
            state_root: [0u8; 32],
            canonical_empty_root: [0u8; 32],
            chunk_count: 0,
            total_nodes: 0,
            total_size: 0,
            created_at_epoch: 0,
            created_at_generation: 0,
            constitutional_hash: [0u8; 32],
        };
        let encoded = header.encode();
        assert_eq!(
            &encoded[0..4],
            b"AMSN",
            "CONST-SNAP-001 VIOLATION: Magic bytes must be AMSN"
        );
    }

    // CONST-SNAP-002: Manifest must self-verify (constructed directly)
    #[test]
    fn snap002_manifest_self_verification() {
        let manifest = SnapshotManifest::new(
            [0x02u8; 32], // state_root
            [0x03u8; 32], // canonical_empty_root
            1,            // chunk_count
            [0x04u8; 32], // chunk_root
            100,          // total_nodes
            50000,        // total_size
            1,            // epoch
            0,            // generation
            0,            // cutoff_sequence
            [0x05u8; 32], // cutoff_root
            [0x06u8; 32], // constitutional_hash
        );
        assert!(
            manifest.verify(),
            "CONST-SNAP-002 VIOLATION: Manifest self-verify failed"
        );
    }

    // CONST-SNAP-003: Constitutional identity must be deterministic
    #[test]
    fn snap003_identity_determinism() {
        let id1 = ConstitutionalIdentity::new([0x42u8; 32]);
        let id2 = ConstitutionalIdentity::new([0x42u8; 32]);
        assert_eq!(
            id1.identity_hash, id2.identity_hash,
            "CONST-SNAP-003 VIOLATION: Same constitution must produce identical identity"
        );
        assert!(
            id1.matches(&id2),
            "CONST-SNAP-003 VIOLATION: Identical identities must match"
        );
    }

    // CONST-SNAP-004: Self-compatibility is FullyCompatible
    #[test]
    fn snap004_self_compatibility() {
        let id = ConstitutionalIdentity::new([0xAAu8; 32]);
        let matrix = CompatibilityEngine::compute(&id, &id);
        assert_eq!(
            matrix.level,
            CompatibilityLevel::FullyCompatible,
            "CONST-SNAP-004 VIOLATION: Self must be FullyCompatible"
        );
        assert!(
            CompatibilityEngine::can_sync(&id, &id),
            "CONST-SNAP-004 VIOLATION: Self must be syncable"
        );
    }
}
