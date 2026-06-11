use amun_certificate_network::distribution::LightClientProofBundle;
use amun_chain_checkpoint::{
    bootstrap::BootstrapSession,
    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
    CheckpointCertificate,
};
use amun_constitutional_block::ConstitutionalBlock;
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_networking::sync_protocol::{SyncRequest, SyncResponse};

/// Helper: build a checkpoint covering blocks [start, end].
fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in start..=end {
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();

        let parent_hash = if height == start {
            &parent
        } else {
            &bundles.last().unwrap().block_header.block_hash
        };

        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );

        bundles.push(LightClientProofBundle::new(block, cert, proof));
    }

    CheckpointCertificate::create(start, end, &bundles).unwrap()
}

#[test]
fn n18_checkpoint_sync_between_nodes() {
    // Node A is at height 50 with a checkpoint
    let cp_a = build_checkpoint(0, 49);
    let checkpoints = vec![cp_a.clone()];
    let trusted_root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_a.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp_a.clone(), proof);

    // Node B starts bootstrapping
    let mut node_b = NetworkNode::new_bootstrapping([20u8; 32]);
    assert_eq!(node_b.lifecycle, NodeLifecycle::Bootstrapping);
    assert_eq!(node_b.current_height, 0);

    // Node B requests sync
    let _request = SyncRequest { from_height: 0 };

    // Node A responds with checkpoint
    let response = SyncResponse {
        latest_height: 50,
        checkpoints: vec![cp_a.clone()],
    };

    // Node B processes response
    assert_eq!(response.latest_height, 50);
    assert_eq!(response.checkpoints.len(), 1);

    // Node B verifies the checkpoint against trusted root
    let mut session = BootstrapSession::new(trusted_root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    // Node B imports the checkpoint height
    node_b.import_checkpoint_height(response.latest_height);
    assert_eq!(node_b.current_height, 50);

    // Node B transitions through lifecycle
    node_b.begin_verification();
    assert_eq!(node_b.lifecycle, NodeLifecycle::Verifying);

    node_b.activate();
    assert_eq!(node_b.lifecycle, NodeLifecycle::Active);
    assert!(node_b.is_active());

    // Node B can now participate
    node_b.propose();
    assert!(!node_b.consensus.pending_actions.is_empty());
}

#[test]
fn n18_bootstrapping_node_rejects_proposal_before_activation() {
    let mut node = NetworkNode::new_bootstrapping([21u8; 32]);

    // Even with correct height, cannot propose before activation
    node.import_checkpoint_height(100);
    node.propose();
    assert!(node.consensus.pending_actions.is_empty());

    // Only after activation
    node.activate();
    node.propose();
    assert!(!node.consensus.pending_actions.is_empty());
}

#[test]
fn n18_sync_response_with_multiple_checkpoints() {
    let cp1 = build_checkpoint(0, 9);
    let cp2 = build_checkpoint(10, 19);

    let response = SyncResponse {
        latest_height: 20,
        checkpoints: vec![cp1, cp2],
    };

    let json = serde_json::to_string(&response).unwrap();
    let decoded: SyncResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.latest_height, 20);
    assert_eq!(decoded.checkpoints.len(), 2);
}
