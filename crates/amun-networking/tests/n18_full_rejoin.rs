use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_networking::sync_protocol::{SyncRequest, SyncResponse};
use amun_chain_checkpoint::{
    CheckpointCertificate,
    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
    bootstrap::BootstrapSession,
};
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_constitutional_block::ConstitutionalBlock;
use amun_certificate_network::distribution::LightClientProofBundle;

/// Build a checkpoint covering blocks [start, end].
fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in start..=end {
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(
            ConstitutionalStateRuntime::certificate_merkle_root(&certs)
        );
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(
            &certs, &hash
        ).unwrap();

        let parent_hash = if height == start {
            &parent
        } else {
            &bundles.last().unwrap().block_header.block_hash
        };

        let block = ConstitutionalBlock::new(
            height, parent_hash.into(), "t".into(), "p".into(), vec![],
            hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(), merkle_root,
        );

        bundles.push(LightClientProofBundle::new(block, cert, proof));
    }

    CheckpointCertificate::create(start, end, &bundles).unwrap()
}

// ============================================================
// N18.7 — Full Rejoin After Network Progress
// ============================================================

#[test]
fn n18_rejoin_after_network_progress() {
    // Phase 1: Network commits 10 blocks, then node 3 is removed
    let _cp_early = build_checkpoint(0, 9);
    let _network_height_early = 10;

    // Phase 2: Network commits 40 more blocks (50 total)
    let cp_late = build_checkpoint(10, 49);
    let network_height_late = 50;
    let checkpoints = vec![cp_late.clone()];
    let trusted_root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp_late.clone(), proof);

    // Phase 3: Node 3 rejoins as bootstrapping
    let mut node3 = NetworkNode::new_bootstrapping([3u8; 32]);
    assert_eq!(node3.lifecycle, NodeLifecycle::Bootstrapping);
    assert_eq!(node3.current_height, 0);

    // Cannot activate directly — must go through lifecycle
    node3.propose();
    assert!(node3.consensus.pending_actions.is_empty());

    // Phase 4: Node requests and receives latest checkpoint
    let _request = SyncRequest { from_height: 0 };
    let response = SyncResponse {
        latest_height: network_height_late,
        checkpoints: vec![cp_late.clone()],
    };

    // Phase 5: Verify checkpoint against trusted root
    let mut session = BootstrapSession::new(trusted_root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    // Phase 6: Import checkpoint height
    node3.import_checkpoint_height(response.latest_height);
    assert_eq!(node3.current_height, network_height_late);

    // Phase 7: Transition through lifecycle
    node3.begin_catchup();
    assert_eq!(node3.lifecycle, NodeLifecycle::CatchingUp);

    node3.begin_verification();
    assert_eq!(node3.lifecycle, NodeLifecycle::Verifying);

    // Cannot propose in Verifying state
    node3.propose();
    assert!(node3.consensus.pending_actions.is_empty());

    node3.activate();
    assert_eq!(node3.lifecycle, NodeLifecycle::Active);
    assert!(node3.is_active());

    // Phase 8: Node is now at network height and can participate
    assert_eq!(node3.current_height, network_height_late);
    node3.propose();
    assert!(!node3.consensus.pending_actions.is_empty());
}

#[test]
fn n18_rejoin_preserves_network_height_across_multiple_checkpoints() {
    let cp1 = build_checkpoint(0, 19);
    let cp2 = build_checkpoint(20, 39);
    let cp3 = build_checkpoint(40, 59);

    let checkpoints = vec![cp1.clone(), cp2.clone(), cp3.clone()];
    let trusted_root = checkpoint_merkle_root(&checkpoints);

    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
    let proof3 = prove_checkpoint_inclusion(&checkpoints, &cp3.checkpoint_hash_bytes()).unwrap();

    let bundles = vec![
        CheckpointBundle::new(cp1, proof1),
        CheckpointBundle::new(cp2, proof2),
        CheckpointBundle::new(cp3, proof3),
    ];

    let mut session = BootstrapSession::new(trusted_root);
    assert!(session.ingest_bundles(&bundles).is_ok());

    let mut node = NetworkNode::new_bootstrapping([4u8; 32]);
    node.import_checkpoint_height(60);
    node.begin_verification();
    node.activate();

    assert_eq!(node.current_height, 60);
    assert!(node.is_active());
}

#[test]
fn n18_bootstrapping_node_cannot_activate_directly() {
    let mut node = NetworkNode::new_bootstrapping([5u8; 32]);

    // Import height but skip verification
    node.import_checkpoint_height(100);

    // Try to propose before activation — must fail
    node.propose();
    assert!(node.consensus.pending_actions.is_empty());

    // Only after proper lifecycle transition
    node.begin_catchup();
    node.begin_verification();
    node.activate();

    assert!(node.is_active());
    node.propose();
    assert!(!node.consensus.pending_actions.is_empty());
}
