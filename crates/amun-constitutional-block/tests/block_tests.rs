use amun_constitutional_kernel::receipt::ExecutionReceipt;
use amun_constitutional_commitments::SparseMerkleTree;
use amun_constitutional_block::{ConstitutionalBlock, Blockchain};

fn dummy_receipt(id: &str) -> ExecutionReceipt {
    ExecutionReceipt::new(id.into(), "s".into(), 0, 1, "t".into())
}

fn make_genesis() -> ConstitutionalBlock {
    let r = dummy_receipt("r1");
    ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![r],
        "s".into(), "g".into(), "e".into(), "ev".into(), String::new())
}

#[test]
fn test_genesis_creation() {
    let g = make_genesis();
    assert_eq!(g.block_height, 0);
    assert_eq!(g.parent_hash, "0".repeat(64));
}

#[test]
fn test_block_hash_deterministic() {
    let b1 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), "ev".into(), String::new());
    let b2 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), "ev".into(), String::new());
    assert_eq!(b1.block_hash, b2.block_hash);
}

#[test]
fn test_evidence_root_affects_hash() {
    let b1 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), "ev1".into(), String::new());
    let b2 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), "ev2".into(), String::new());
    assert_ne!(b1.block_hash, b2.block_hash);
}

#[test]
fn test_chain_append() {
    let mut chain = Blockchain::new();
    chain.append(make_genesis()).unwrap();
    assert_eq!(chain.blocks.len(), 1);
}

#[test]
fn test_evidence_root_determinism() {
    let mut ev1 = SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
    let mut ev2 = SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
    ev1.insert(b"action_0", &[42u8; 32]);
    ev2.insert(b"action_0", &[42u8; 32]);

    let b1 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), hex::encode(ev1.root()), String::new());
    let b2 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), hex::encode(ev2.root()), String::new());
    assert_eq!(b1.evidence_root, b2.evidence_root);
    assert_eq!(b1.block_hash, b2.block_hash);
}

#[test]
fn test_evidence_root_sensitivity() {
    let mut ev1 = SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
    let mut ev2 = SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
    ev1.insert(b"action_0", &[42u8; 32]);
    ev2.insert(b"action_0", &[99u8; 32]);

    let b1 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), hex::encode(ev1.root()), String::new());
    let b2 = ConstitutionalBlock::new(0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "s".into(), "g".into(), "e".into(), hex::encode(ev2.root()), String::new());
    assert_ne!(b1.evidence_root, b2.evidence_root);
    assert_ne!(b1.block_hash, b2.block_hash);
}

#[test]
fn test_verify_block_provenance_valid() {
    use amun_constitutional_block::verify_block_provenance;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert))
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    assert!(verify_block_provenance(&block, &cert).is_ok());
}

#[test]
fn test_verify_block_provenance_tampered_state_fails() {
    use amun_constitutional_block::verify_block_provenance;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert))
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        "tampered_state".into(), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    assert!(verify_block_provenance(&block, &cert).is_err());
}

#[test]
fn test_verify_block_provenance_wrong_certificate_fails() {
    use amun_constitutional_block::verify_block_provenance;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);

    // Use a different certificate's merkle root
    let wrong_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(&[])
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        wrong_root,
    );

    assert!(verify_block_provenance(&block, &cert).is_err());
}

#[test]
fn n6b_full_replay_valid() {
    use amun_constitutional_block::verify_full_replay;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert))
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    assert!(verify_full_replay(&block, &cert, rt.journal()).is_ok());
}

#[test]
fn n6b_full_replay_tampered_journal_fails() {
    use amun_constitutional_block::verify_full_replay;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert))
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    let mut tampered = rt.journal().to_vec();
    tampered[0].transition_hash = [0xFF; 32];

    assert!(verify_full_replay(&block, &cert, &tampered).is_err());
}

#[test]
fn n6b_full_replay_wrong_count_fails() {
    use amun_constitutional_block::verify_full_replay;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert))
    );

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    // Only pass one record when cert expects two
    assert!(verify_full_replay(&block, &cert, &rt.journal()[..1]).is_err());
}

#[test]
fn n8_light_client_valid() {
    use amun_constitutional_block::verify_light_client_proof;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let certs = vec![cert.clone()];
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(&certs)
    );
    let hash = cert.certificate_hash();
    let inclusion_proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_ok());
}

#[test]
fn n8_light_client_tampered_proof_fails() {
    use amun_constitutional_block::verify_light_client_proof;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let certs = vec![cert.clone()];
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(&certs)
    );
    let hash = cert.certificate_hash();
    let mut inclusion_proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();

    // Tamper with the proof's root
    inclusion_proof.root = [0xFF; 32];

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
}

#[test]
fn n8_light_client_wrong_certificate_fails() {
    use amun_constitutional_block::verify_light_client_proof;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt1 = ConstitutionalStateRuntime::new();
    rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert1 = rt1.create_certificate(1, [0u8; 32]);

    let mut rt2 = ConstitutionalStateRuntime::new();
    rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
    let cert2 = rt2.create_certificate(1, [0u8; 32]);

    let certs = vec![cert1.clone()];
    let merkle_root = hex::encode(
        ConstitutionalStateRuntime::certificate_merkle_root(&certs)
    );
    let hash1 = cert1.certificate_hash();
    let inclusion_proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash1).unwrap();

    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt1.state_root()), "g".into(), "e".into(), "ev".into(),
        merkle_root,
    );

    // Use cert2 with inclusion_proof for cert1
    assert!(verify_light_client_proof(&block, &cert2, &inclusion_proof).is_err());
}

#[test]
fn n8_light_client_wrong_block_root_fails() {
    use amun_constitutional_block::verify_light_client_proof;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    let mut rt = ConstitutionalStateRuntime::new();
    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
    let cert = rt.create_certificate(1, [0u8; 32]);
    let certs = vec![cert.clone()];
    let hash = cert.certificate_hash();
    let inclusion_proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();

    // Block has a different replay_certificate_root
    let block = ConstitutionalBlock::new(
        0, "0".repeat(64), "t".into(), "p".into(), vec![],
        hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(),
        "wrong_root".into(),
    );

    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
}

