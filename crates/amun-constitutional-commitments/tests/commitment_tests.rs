use amun_constitutional_commitments::SparseMerkleTree;

fn build_domain_roots() -> ([u8; 32], [u8; 32], [u8; 32]) {
    let mut state_tree = SparseMerkleTree::new(b"AMUN_STATE_DOMAIN");
    let mut gov_tree = SparseMerkleTree::new(b"AMUN_GOVERNANCE_DOMAIN");
    let mut exec_tree = SparseMerkleTree::new(b"AMUN_EXECUTION_DOMAIN");

    state_tree.insert(b"balance:alice", &[100u8; 32]);
    gov_tree.insert(b"proposal:42", &[1u8; 32]);
    exec_tree.insert(b"receipt:123", &[9u8; 32]);

    (state_tree.root(), gov_tree.root(), exec_tree.root())
}

#[test]
fn domain_roots_are_deterministic() {
    let (s1, g1, e1) = build_domain_roots();
    let (s2, g2, e2) = build_domain_roots();
    assert_eq!(s1, s2);
    assert_eq!(g1, g2);
    assert_eq!(e1, e2);
}

#[test]
fn different_domains_produce_different_roots() {
    let (s, g, e) = build_domain_roots();
    assert_ne!(s, g);
    assert_ne!(s, e);
    assert_ne!(g, e);
}

#[test]
fn domain_proofs_are_independent() {
    let mut state_tree = SparseMerkleTree::new(b"AMUN_STATE_DOMAIN");
    state_tree.insert(b"balance:alice", &[100u8; 32]);
    let state_root = state_tree.root();
    let proof = state_tree.prove(b"balance:alice");
    assert!(state_tree.verify(&state_root, &proof));

    let mut gov_tree = SparseMerkleTree::new(b"AMUN_GOVERNANCE_DOMAIN");
    gov_tree.insert(b"proposal:42", &[1u8; 32]);
    let gov_root = gov_tree.root();
    assert!(!gov_tree.verify(&gov_root, &proof));
}
