// N120.4 — Consensus Enforcement of slashing_root
use amun_block_builder::BlockBuilder;

use amun_mempool::Mempool;

#[test]
fn n120_4a_validator_accepts_matching_slashing_root() {
    // Setup: build a block with a known slashing_root
    let root = [0x42; 32];
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
    block.slashing_root = root;

    // Validator verifies with the same root → accepted
    assert!(
        block.verify_slashing_root(&root).is_ok(),
        "N120.4a FAIL: Validator must accept matching slashing_root"
    );
}

#[test]
fn n120_4b_validator_rejects_mismatched_slashing_root() {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
    block.slashing_root = [0x42; 32];

    // Validator has a different root → rejected
    let result = block.verify_slashing_root(&[0xFF; 32]);
    assert!(
        result.is_err(),
        "N120.4b FAIL: Validator must reject mismatched slashing_root"
    );
}

#[test]
fn n120_4c_block_with_tampered_root_rejected() {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);

    // Proposer sets the root
    block.slashing_root = [0x42; 32];

    // Attacker tampers with the root after the block is built
    block.slashing_root = [0xBA; 32]; // tampered

    // Validator has the original root → must reject
    let result = block.verify_slashing_root(&[0x42; 32]);
    assert!(
        result.is_err(),
        "N120.4c FAIL: Tampered slashing_root must be rejected by validators"
    );
}

#[test]
fn n120_4d_mismatched_root_prevents_voting() {
    // Simulates: proposer builds block with root X, validator computes root Y
    // The mismatch is detected, and the validator must NOT accept the block.
    let proposed_root = [0x42; 32];
    let validator_root = [0xFF; 32]; // different!

    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
    block.slashing_root = proposed_root;

    // Validator checks the root → mismatch detected
    let mismatch = block.slashing_root != validator_root;
    assert!(
        mismatch,
        "N120.4d FAIL: Mismatched root must be detectable before voting"
    );

    // Block must be rejected
    assert!(
        block.verify_slashing_root(&validator_root).is_err(),
        "N120.4d FAIL: Block with mismatched root must be rejected"
    );
}
