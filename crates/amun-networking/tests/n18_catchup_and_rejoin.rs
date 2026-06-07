use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_networking::sync_protocol::{SyncRequest, SyncResponse};

// ============================================================
// N18.5 — Constitutional Catch-up (no network)
// ============================================================

#[test]
fn n18_catchup_import_checkpoint_height() {
    let mut node = NetworkNode::new_bootstrapping([10u8; 32]);

    // Node is behind
    assert_eq!(node.current_height, 0);

    // Import checkpoint from trusted source
    node.import_checkpoint_height(100);
    assert_eq!(node.current_height, 100);
    assert_eq!(node.consensus.state.height, 100);
}

#[test]
fn n18_full_lifecycle_with_catchup() {
    let mut node = NetworkNode::new_bootstrapping([11u8; 32]);

    // Phase 1: Bootstrap
    assert_eq!(node.lifecycle, NodeLifecycle::Bootstrapping);
    node.propose();
    assert!(node.consensus.pending_actions.is_empty());

    // Phase 2: Catch-up
    node.begin_catchup();
    node.import_checkpoint_height(50);
    assert_eq!(node.current_height, 50);

    // Phase 3: Verify
    node.begin_verification();
    assert_eq!(node.lifecycle, NodeLifecycle::Verifying);

    // Phase 4: Activate
    node.activate();
    assert_eq!(node.lifecycle, NodeLifecycle::Active);
    assert!(node.is_active());

    // Now can propose
    node.propose();
    assert!(!node.consensus.pending_actions.is_empty());
}

#[test]
fn n18_catchup_preserves_height_after_activation() {
    let mut node = NetworkNode::new_bootstrapping([12u8; 32]);
    node.import_checkpoint_height(42);
    node.activate();

    // Height must survive activation
    assert_eq!(node.current_height, 42);
    assert_eq!(node.consensus.state.height, 42);
}

// ============================================================
// N18.6 — Sync protocol integration test
// ============================================================

#[test]
fn n18_sync_request_roundtrip() {
    let req = SyncRequest { from_height: 7 };
    let json = serde_json::to_string(&req).unwrap();
    let decoded: SyncRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.from_height, 7);
}

#[test]
fn n18_sync_response_serialization_roundtrip() {
    let resp = SyncResponse {
        latest_height: 42,
        checkpoints: Vec::new(),
    };
    let json = serde_json::to_string(&resp).unwrap();
    let decoded: SyncResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.latest_height, 42);
    assert!(decoded.checkpoints.is_empty());
}

// ============================================================
// N18.7 — Full Rejoin (skeleton)
// ============================================================

#[test]
#[ignore = "Requires network-integrated bootstrap"]
fn n18_full_rejoin_matches_network_state() {
    // 4 nodes commit 10 blocks
    // Node 2 crashes
    // Network commits 20 more
    // Node 2 rejoins, catches up, verifies, activates
    // assert_eq!(node2.height, network.height)
    // assert_eq!(node2.state_root, network.state_root)
}
