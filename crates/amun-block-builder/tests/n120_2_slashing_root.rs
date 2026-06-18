// N120.2 — Slashing Root in Block Header Tests
use amun_block_builder::{Block, BlockBuilder};
use amun_mempool::Mempool;

fn build_block_with_root(root: [u8; 32]) -> Block {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
    block.slashing_root = root;
    block
}

#[test]
fn n120_2_block_hash_changes_with_slashing_root() {
    let block1 = build_block_with_root([0u8; 32]);
    let mut block2 = build_block_with_root([0u8; 32]);
    block2.slashing_root = [0xAA; 32];

    let h1 = block1.block_hash();
    let h2 = block2.block_hash();
    assert_ne!(
        h1, h2,
        "N120.2 FAIL: different slashing roots must produce different block hashes"
    );
}

#[test]
fn n120_2_same_root_same_hash() {
    let block1 = build_block_with_root([0x42; 32]);
    let block2 = build_block_with_root([0x42; 32]);

    let h1 = block1.block_hash();
    let h2 = block2.block_hash();
    assert_eq!(
        h1, h2,
        "N120.2 FAIL: same slashing root must produce same block hash"
    );
}

#[test]
fn n120_2_empty_root_allowed() {
    let block = build_block_with_root([0u8; 32]);
    let hash = block.block_hash();
    assert_ne!(
        hash, [0u8; 32],
        "N120.2 FAIL: block hash must not be zero even with empty root"
    );
}

#[test]
fn n120_2_different_roots_different_hashes() {
    let root_a = [0x11u8; 32];
    let root_b = [0x22u8; 32];
    let block_a = build_block_with_root(root_a);
    let block_b = build_block_with_root(root_b);
    assert_ne!(
        block_a.block_hash(),
        block_b.block_hash(),
        "N120.2 FAIL: different slashing roots must produce different block hashes"
    );
}

#[test]
fn n120_2_zero_root_vs_nonzero_root_different_hash() {
    let block_zero = build_block_with_root([0u8; 32]);
    let block_nonzero = build_block_with_root([0xAB; 32]);
    assert_ne!(
        block_zero.block_hash(),
        block_nonzero.block_hash(),
        "N120.2 FAIL: zero root vs nonzero root must produce different hashes"
    );
}

// N120.3 — Consensus Verification of slashing_root
#[test]
fn n120_3_matching_root_accepted() {
    let root = [0x42; 32];
    let block = build_block_with_root(root);
    assert!(
        block.verify_slashing_root(&root).is_ok(),
        "N120.3 FAIL: matching root must be accepted"
    );
}

#[test]
fn n120_3_mismatched_root_rejected() {
    let block = build_block_with_root([0x42; 32]);
    let result = block.verify_slashing_root(&[0xFF; 32]);
    assert!(
        result.is_err(),
        "N120.3 FAIL: mismatched root must be rejected"
    );
    assert!(result.unwrap_err().contains("slashing_root mismatch"));
}

#[test]
fn n120_3_zero_root_verified_correctly() {
    let block = build_block_with_root([0u8; 32]);
    assert!(
        block.verify_slashing_root(&[0u8; 32]).is_ok(),
        "N120.3 FAIL: zero root must match zero root"
    );
    assert!(
        block.verify_slashing_root(&[0x01; 32]).is_err(),
        "N120.3 FAIL: zero root must not match nonzero root"
    );
}
