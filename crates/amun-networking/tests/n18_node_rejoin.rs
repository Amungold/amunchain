use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_chain_checkpoint::bootstrap::BootstrapSession;

// ============================================================
// N18.2 — Lifecycle Invariants
// ============================================================

#[test]
fn n18_bootstrapping_node_cannot_propose() {
    let mut node = NetworkNode::new_bootstrapping([1u8; 32]);
    node.propose();
    assert!(
        node.consensus.pending_actions.is_empty(),
        "Bootstrapping node must not emit consensus actions"
    );
}

#[test]
fn n18_active_node_can_propose() {
    let mut node = NetworkNode::new([2u8; 32]);
    assert_eq!(node.lifecycle, NodeLifecycle::Active);
    node.propose();
    assert!(
        !node.consensus.pending_actions.is_empty(),
        "Active node must emit BroadcastProposal"
    );
}

#[test]
fn n18_lifecycle_transitions() {
    let mut node = NetworkNode::new_bootstrapping([3u8; 32]);
    assert_eq!(node.lifecycle, NodeLifecycle::Bootstrapping);

    // Simulate catch-up phase
    node.lifecycle = NodeLifecycle::CatchingUp;
    assert_eq!(node.lifecycle, NodeLifecycle::CatchingUp);

    // Simulate verification phase
    node.lifecycle = NodeLifecycle::Verifying;
    assert_eq!(node.lifecycle, NodeLifecycle::Verifying);

    // Activate
    node.activate();
    assert_eq!(node.lifecycle, NodeLifecycle::Active);
}

#[test]
fn n18_bootstrapping_node_stores_trusted_root() {
    let root = [0xAB; 32];
    let session = BootstrapSession::new(root);
    assert_eq!(session.trusted_root(), root);
}

// ============================================================
// N18.3 — Rejoin Test (skeleton, requires sync protocol)
// ============================================================

#[test]
#[ignore = "Requires full sync protocol implementation"]
fn n18_full_rejoin_after_crash() {
    // Scenario:
    // 1. 4-node network commits block 1
    // 2. Node 2 crashes
    // 3. Network commits 20 more blocks
    // 4. Node 2 rejoins as Bootstrapping
    // 5. Node 2 catches up via checkpoints
    // 6. Node 2 verifies state root
    // 7. Node 2 becomes Active
    // 8. assert_eq!(node2.height, network.height)
    // 9. assert_eq!(node2.state_root, canonical_root)
}

// ============================================================
// N18.5 — Constitutional Invariant REJOIN-001
// ============================================================

#[test]
fn n18_rejoin001_bootstrapping_node_must_verify_before_active() {
    let mut node = NetworkNode::new_bootstrapping([5u8; 32]);

    // Bootstrapping: cannot propose
    node.propose();
    assert!(node.consensus.pending_actions.is_empty());

    // Even after setting height manually, still cannot propose
    node.current_height = 10;
    node.propose();
    assert!(node.consensus.pending_actions.is_empty());

    // Only after explicit activation
    node.activate();
    assert_eq!(node.lifecycle, NodeLifecycle::Active);
    node.propose();
    assert!(!node.consensus.pending_actions.is_empty());
}
