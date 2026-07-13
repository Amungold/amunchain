#[cfg(test)]
mod audit_byzantine {
    use amun_snapshot_engine::{
        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
    };

    // CONST-BYZ-001: Quorum with identical manifests
    #[test]
    fn byz001_quorum_detection() {
        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
        // Create identical manifests so they have the same manifest_hash
        let manifest = SnapshotManifest::new(
            [0x42u8; 32],
            [0x43u8; 32],
            1,
            [0x44u8; 32],
            100,
            50000,
            0,
            0,
            0,
            [0x45u8; 32],
            identity.constitutional_hash,
        );

        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
        engine.add_peer_manifest(PeerManifest {
            peer_id: [0x01u8; 32],
            manifest: manifest.clone(),
            identity: identity.clone(),
            signature: None,
        });
        engine.add_peer_manifest(PeerManifest {
            peer_id: [0x02u8; 32],
            manifest: manifest.clone(),
            identity: identity.clone(),
            signature: None,
        });

        match engine.decide() {
            SyncDecision::Accepted { agreeing_peers, .. } => {
                assert!(
                    agreeing_peers >= 2,
                    "CONST-BYZ-001: Must have quorum, got {}",
                    agreeing_peers
                );
            }
            other => panic!("CONST-BYZ-001: Expected Accepted, got {:?}", other),
        }
    }

    // CONST-BYZ-002: Foreign identity is rejected
    #[test]
    fn byz002_identity_mismatch_rejection() {
        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
        let mut engine = ByzantineSyncEngine::new(local.clone(), 1);
        let manifest = SnapshotManifest::new(
            [0x42u8; 32],
            [0x43u8; 32],
            1,
            [0x44u8; 32],
            100,
            50000,
            0,
            0,
            0,
            [0x45u8; 32],
            foreign.constitutional_hash,
        );

        engine.add_peer_manifest(PeerManifest {
            peer_id: [0x03u8; 32],
            manifest,
            identity: foreign,
            signature: None,
        });

        match engine.decide() {
            SyncDecision::InsufficientPeers { .. } => {}
            other => panic!("CONST-BYZ-002: Expected InsufficientPeers, got {:?}", other),
        }
    }
}
