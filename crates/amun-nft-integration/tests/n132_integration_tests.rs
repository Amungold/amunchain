use amun_evidence_root::EvidenceRoot;
use amun_nft_integration::extend_evidence_root_with_nft;

#[test]
fn n132_extended_root_deterministic() {
    let original = EvidenceRoot::genesis();
    let nft_root = [1u8; 32];

    let r1 = extend_evidence_root_with_nft(&original, &nft_root);
    let r2 = extend_evidence_root_with_nft(&original, &nft_root);
    assert_eq!(r1.root, r2.root);
}

#[test]
fn n132_different_nft_changes_root() {
    let original = EvidenceRoot::genesis();
    let nft_root1 = [1u8; 32];
    let nft_root2 = [2u8; 32];

    let r1 = extend_evidence_root_with_nft(&original, &nft_root1);
    let r2 = extend_evidence_root_with_nft(&original, &nft_root2);
    assert_ne!(r1.root, r2.root);
}

#[test]
fn n132_height_and_metadata_preserved() {
    let original = EvidenceRoot::compute(
        [9u8; 32], // state_root
        [8u8; 32], // commit_hash
        [7u8; 32], // replay_certificate
        [6u8; 32], // audit_record
        [0u8; 32], // previous_root
        42,        // height
    );
    let nft_root = [3u8; 32];
    let extended = extend_evidence_root_with_nft(&original, &nft_root);

    assert_eq!(extended.height, 42);
    assert_eq!(extended.state_root, [9u8; 32]);
    assert_eq!(extended.commit_hash, [8u8; 32]);
    assert_eq!(extended.replay_certificate, [7u8; 32]);
    assert_eq!(extended.audit_record, [6u8; 32]);
}
