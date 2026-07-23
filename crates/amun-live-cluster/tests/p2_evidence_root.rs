//! P2: Evidence Root Tests
//!
//! Verify that evidence_root is properly chained across blocks
//! and affects block_hash as expected.

use amun_block_builder::{Block, BlockBuilder};
use amun_mempool::Mempool;
use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
use ed25519_dalek::SigningKey;

fn make_tx(seed: u8, nonce: u64) -> Transaction {
    let s = [seed; 32];
    let sk = SigningKey::from_bytes(&s);
    let sender = sk.verifying_key().to_bytes();
    let mut tx = Transaction {
        version: 1,
        sender,
        nonce,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: [2u8; 32],
            amount: 100,
        }),
        signature: vec![],
    };
    tx.sign(&sk);
    tx
}

fn build_block_with_evidence(
    height: u64,
    parent_hash: [u8; 32],
    evidence_root: [u8; 32],
) -> Block {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let tx = make_tx(1, height);
    mempool.add_transaction(tx.clone()).ok();
    builder.build_block_with_certificates(
        height, parent_hash, &mut mempool, 1, [0u8; 32], 1000, vec![], evidence_root,
        [0u8; 32],
    )
}

#[test]
fn p2_evidence_root_changes_block_hash() {
    let parent = [0u8; 32];
    let block1 = build_block_with_evidence(1, parent, [0u8; 32]);
    let block2 = build_block_with_evidence(1, parent, [1u8; 32]);

    assert_ne!(block1.evidence_root, block2.evidence_root,
        "Different evidence_root must be stored in the block");
    assert_ne!(block1.block_hash(), block2.block_hash(),
        "Different evidence_root must produce different block_hash");
}

#[test]
fn p2_evidence_root_deterministic() {
    let parent = [0u8; 32];
    let ev_root = [42u8; 32];
    let block1 = build_block_with_evidence(1, parent, ev_root);
    let block2 = build_block_with_evidence(1, parent, ev_root);

    assert_eq!(block1.block_hash(), block2.block_hash(),
        "Same evidence_root must produce same block_hash");
    assert_eq!(block1.evidence_root, block2.evidence_root,
        "Same evidence_root must be stored identically");
}

#[test]
fn p2_evidence_root_chaining() {
    let genesis_evidence = [0u8; 32];
    let block1 = build_block_with_evidence(1, [0u8; 32], genesis_evidence);

    let ev1 = [1u8; 32];
    let block2 = build_block_with_evidence(2, block1.block_hash(), ev1);

    let ev2 = [2u8; 32];
    let block3 = build_block_with_evidence(3, block2.block_hash(), ev2);

    // Each block stores the evidence from the PREVIOUS block's era
    assert_eq!(block1.evidence_root, genesis_evidence);
    assert_eq!(block2.evidence_root, ev1);
    assert_eq!(block3.evidence_root, ev2);

    // Changing any evidence_root changes all subsequent block hashes
    let block2_alt = build_block_with_evidence(2, block1.block_hash(), [99u8; 32]);
    let block3_alt = build_block_with_evidence(3, block2_alt.block_hash(), ev2);

    assert_ne!(block3.block_hash(), block3_alt.block_hash(),
        "Changing evidence_root in block 2 must change block 3's hash");
}

#[test]
fn p2_evidence_root_zero_default() {
    let block = build_block_with_evidence(1, [0u8; 32], [0u8; 32]);
    assert_eq!(block.evidence_root, [0u8; 32],
        "Genesis block should have zero evidence_root");
    assert_ne!(block.block_hash(), [0u8; 32],
        "Block hash must still be non-zero");
}
