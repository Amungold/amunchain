#[cfg(test)]
mod audit_byzantine_mesh {
    use amun_snapshot_engine::{
        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
    };

    // CONST-MESH-001: Conflicting manifests (different state_root) are detected
    #[test]
    fn mesh001_conflicting_manifests_detected() {
        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
        let manifest1 = SnapshotManifest::new(
            [0xAAu8; 32],
            [0xBBu8; 32],
            1,
            [0xCCu8; 32],
            100,
            50000,
            0,
            0,
            0,
            [0xDDu8; 32],
            identity.constitutional_hash,
        );
        let manifest2 = SnapshotManifest::new(
            [0x11u8; 32],
            [0xBBu8; 32],
            1,
            [0x22u8; 32],
            100,
            50000,
            0,
            0,
            0,
            [0x33u8; 32],
            identity.constitutional_hash,
        );

        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
        engine.add_peer_manifest(PeerManifest {
            peer_id: [0x01u8; 32],
            manifest: manifest1,
            identity: identity.clone(),
            signature: None,
        });
        engine.add_peer_manifest(PeerManifest {
            peer_id: [0x02u8; 32],
            manifest: manifest2,
            identity: identity.clone(),
            signature: None,
        });

        match engine.decide() {
            SyncDecision::ConflictingCivilizations { groups } => {
                assert!(
                    !groups.is_empty(),
                    "CONST-MESH-001 VIOLATION: Must detect conflicting groups"
                );
            }
            other => panic!(
                "CONST-MESH-001: Expected ConflictingCivilizations, got {:?}",
                other
            ),
        }
    }

    // CONST-MESH-002: Foreign civilizations are rejected
    #[test]
    fn mesh002_foreign_civilization_rejection() {
        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
        assert!(
            !local.matches(&foreign),
            "CONST-MESH-002 VIOLATION: Different constitutions must not match"
        );
    }
}
