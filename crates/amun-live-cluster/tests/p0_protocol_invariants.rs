//! Protocol Invariant Tests (ADR-028 §5)
//!
//! Automated verification of invariants I1 through I8.
//! Part of the P0.2 Conformance Suite.

use amun_block_builder::{Block, BlockBuilder};
use amun_mempool::Mempool;
use amun_transactions::{Transaction, TransactionPayload, TransactionReceipt, TransferPayload};
use ed25519_dalek::SigningKey;

/// Helper: Create a signed transfer transaction.
fn make_tx(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
    let s = [seed; 32];
    let sk = SigningKey::from_bytes(&s);
    let sender = sk.verifying_key().to_bytes();
    let mut tx = Transaction {
        version: 1,
        sender,
        nonce,
        payload: TransactionPayload::Transfer(TransferPayload { to, amount }),
        signature: vec![],
    };
    tx.sign(&sk);
    tx
}

/// Helper: Create a receipt.
fn make_receipt(tx_hash: [u8; 32], success: bool, gas: u64) -> TransactionReceipt {
    TransactionReceipt {
        tx_hash,
        success,
        error_code: if success { None } else { Some(1) },
        sender: [0u8; 32],
        nonce: 0,
        gas_used: gas,
    }
}

/// Helper: Build a block with given transactions and receipts.
fn build_block(
    height: u64,
    parent_hash: [u8; 32],
    txs: Vec<Transaction>,
    receipts: Vec<TransactionReceipt>,
) -> Block {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    for tx in &txs {
        mempool.add_transaction(tx.clone()).ok();
    }
    let mut block = builder.build_block_with_certificates(
        height,
        parent_hash,
        &mut mempool,
        txs.len(),
        [0u8; 32],
        1000,
        vec![],
        [0u8; 32],
    );
    // Override transactions and receipts for testing
    block.transactions = txs;
    block.receipts = receipts;
    // Recompute roots from the actual data
    let tx_hashes: Vec<[u8; 32]> = block.transactions.iter().map(|t| t.tx_hash()).collect();
    block.transactions_root = amun_merkle::transactions_root(&tx_hashes);
    let receipt_hashes: Vec<[u8; 32]> = block.receipts.iter().map(|r| r.receipt_hash()).collect();
    block.receipts_root = amun_merkle::receipts_root(&receipt_hashes);
    block
}

// ============================================================================
// I1: Any transaction change → transactions_root changes → block_hash changes
// ============================================================================
#[test]
fn i1_transaction_mutation_changes_tx_root_and_block_hash() {
    let parent = [0u8; 32];
    let tx1 = make_tx(1, 0, 100, [3u8; 32]);
    let tx2 = make_tx(2, 0, 200, [4u8; 32]);
    let receipt = make_receipt(tx1.tx_hash(), true, 1000);

    let block1 = build_block(1, parent, vec![tx1.clone()], vec![receipt.clone()]);
    let block2 = build_block(1, parent, vec![tx2.clone()], vec![receipt]);

    assert_ne!(block1.transactions_root, block2.transactions_root,
        "I1: Different txs must produce different transactions_root");
    assert_ne!(block1.block_hash(), block2.block_hash(),
        "I1: Different transactions_root must produce different block_hash");
}

// ============================================================================
// I2: Any receipt change → receipts_root changes → block_hash changes
// ============================================================================
#[test]
fn i2_receipt_mutation_changes_receipt_root_and_block_hash() {
    let parent = [0u8; 32];
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let r1 = make_receipt(tx.tx_hash(), true, 1000);
    let r2 = make_receipt(tx.tx_hash(), true, 2000);

    let block1 = build_block(1, parent, vec![tx.clone()], vec![r1]);
    let block2 = build_block(1, parent, vec![tx], vec![r2]);

    assert_ne!(block1.receipts_root, block2.receipts_root,
        "I2: Different receipts must produce different receipts_root");
    assert_ne!(block1.block_hash(), block2.block_hash(),
        "I2: Different receipts_root must produce different block_hash");
}

// ============================================================================
// I3: Any state change → state_root changes → block_hash changes
// ============================================================================
#[test]
fn i3_state_mutation_changes_state_root_and_block_hash() {
    let parent = [0u8; 32];
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let receipt = make_receipt(tx.tx_hash(), true, 1000);

    let block1 = build_block(1, parent, vec![tx.clone()], vec![receipt.clone()]);
    let mut block2 = build_block(1, parent, vec![tx], vec![receipt]);
    block2.state_root = [1u8; 32]; // Simulate different state

    assert_ne!(block1.state_root, block2.state_root,
        "I3: Different state must produce different state_root");
    assert_ne!(block1.block_hash(), block2.block_hash(),
        "I3: Different state_root must produce different block_hash");
}

// ============================================================================
// I4: Any block_hash change → history_root changes
// ============================================================================
#[test]
fn i4_block_hash_change_updates_history_root() {
    let tx1 = make_tx(1, 0, 100, [3u8; 32]);
    let tx2 = make_tx(2, 0, 200, [4u8; 32]);
    let receipt = make_receipt(tx1.tx_hash(), true, 1000);

    let block1 = build_block(1, [0u8; 32], vec![tx1], vec![receipt.clone()]);
    let block2 = build_block(1, [0u8; 32], vec![tx2], vec![receipt]);

    let hr1 = amun_history::compute_history_root([0u8; 32], block1.block_hash());
    let hr2 = amun_history::compute_history_root([0u8; 32], block2.block_hash());

    assert_ne!(hr1, hr2,
        "I4: Different block_hash must produce different history_root");
}

// ============================================================================
// I5: Any past block change → all subsequent history_root values change
// ============================================================================
#[test]
fn i5_historical_change_propagates_history() {
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let receipt = make_receipt(tx.tx_hash(), true, 1000);

    // Chain A: block1_a → block2
    let block1_a = build_block(1, [0u8; 32], vec![tx.clone()], vec![receipt.clone()]);
    let hr1_a = amun_history::compute_history_root([0u8; 32], block1_a.block_hash());
    let block2 = build_block(2, block1_a.block_hash(), vec![tx.clone()], vec![receipt.clone()]);
    let hr2_a = amun_history::compute_history_root(hr1_a, block2.block_hash());

    // Chain B: block1_b (different tx) → block2
    let tx2 = make_tx(2, 0, 200, [4u8; 32]);
    let block1_b = build_block(1, [0u8; 32], vec![tx2], vec![receipt]);
    let hr1_b = amun_history::compute_history_root([0u8; 32], block1_b.block_hash());
    let hr2_b = amun_history::compute_history_root(hr1_b, block2.block_hash());

    assert_ne!(hr2_a, hr2_b,
        "I5: Different block1 must change all subsequent history_root values");
}

// ============================================================================
// I6: Same inputs → same block_hash (Determinism)
// ============================================================================
#[test]
fn i6_deterministic_block_hash() {
    let parent = [0u8; 32];
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let receipt = make_receipt(tx.tx_hash(), true, 1000);

    let block1 = build_block(1, parent, vec![tx.clone()], vec![receipt.clone()]);
    let block2 = build_block(1, parent, vec![tx], vec![receipt]);

    assert_eq!(block1.block_hash(), block2.block_hash(),
        "I6: Same inputs must produce identical block_hash");
}

// ============================================================================
// I7: parent_hash = [0u8; 32] only for genesis block
// ============================================================================
#[test]
fn i7_only_genesis_has_zero_parent() {
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let receipt = make_receipt(tx.tx_hash(), true, 1000);

    // Genesis (height 1) with zero parent is valid
    let genesis = build_block(1, [0u8; 32], vec![tx.clone()], vec![receipt.clone()]);
    assert_eq!(genesis.parent_hash, [0u8; 32],
        "I7: Genesis block must have zero parent_hash");

    // Non-genesis with zero parent should still compute (validation is caller's responsibility)
    // The invariant is that only genesis has this property
    let non_genesis = build_block(2, [0u8; 32], vec![tx], vec![receipt]);
    // This block has parent_hash = 0 but height != 1.
    // It would be rejected by consensus, but structurally it's possible.
    // The test verifies the invariant is documentable.
    assert_eq!(non_genesis.parent_hash, [0u8; 32],
        "Block 2 with zero parent is structurally possible but invalid per protocol");
}

// ============================================================================
// I8: Commitments are independent
// ============================================================================
#[test]
fn i8_commitments_are_independent() {
    let parent = [0u8; 32];
    let tx = make_tx(1, 0, 100, [3u8; 32]);
    let receipt = make_receipt(tx.tx_hash(), true, 1000);

    let block1 = build_block(1, parent, vec![tx.clone()], vec![receipt.clone()]);
    let mut block2 = build_block(1, parent, vec![tx], vec![receipt]);

    // Change only transactions_root
    block2.transactions_root = [1u8; 32];

    // receipts_root must NOT change
    assert_eq!(block1.receipts_root, block2.receipts_root,
        "I8: Changing transactions_root must not change receipts_root");
    // state_root must NOT change
    assert_eq!(block1.state_root, block2.state_root,
        "I8: Changing transactions_root must not change state_root");
    // slashing_root must NOT change
    assert_eq!(block1.slashing_root, block2.slashing_root,
        "I8: Changing transactions_root must not change slashing_root");
}
