//! P3: Validator Set Root Tests
//!
//! Verify that validator_set_root is properly computed and affects block_hash.

use amun_block_builder::{Block, BlockBuilder};
use amun_mempool::Mempool;
use amun_merkle;
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

fn build_block_with_validator_set(vs_root: [u8; 32]) -> Block {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();
    let tx = make_tx(1, 1);
    mempool.add_transaction(tx.clone()).ok();
    builder.build_block_with_certificates(
        1,
        [0u8; 32],
        &mut mempool,
        1,
        [0u8; 32],
        1000,
        vec![],
        [0u8; 32],
        vs_root,
    )
}

#[test]
fn p3_validator_set_root_changes_block_hash() {
    let b1 = build_block_with_validator_set([0u8; 32]);
    let b2 = build_block_with_validator_set([1u8; 32]);
    assert_ne!(b1.block_hash(), b2.block_hash());
}

#[test]
fn p3_validator_set_root_deterministic() {
    let ids: Vec<[u8; 32]> = vec![[1u8; 32], [3u8; 32], [2u8; 32]];
    let r1 = amun_merkle::validator_set_root(&ids);
    let r2 = amun_merkle::validator_set_root(&ids);
    assert_eq!(r1, r2);
}

#[test]
fn p3_validator_set_root_order_independent() {
    let ids1 = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
    let ids2 = vec![[3u8; 32], [1u8; 32], [2u8; 32]];
    assert_eq!(
        amun_merkle::validator_set_root(&ids1),
        amun_merkle::validator_set_root(&ids2),
        "Validator set root must be order-independent (sorted)"
    );
}

#[test]
fn p3_validator_set_root_changes_with_set() {
    let r1 = amun_merkle::validator_set_root(&[[1u8; 32], [2u8; 32]]);
    let r2 = amun_merkle::validator_set_root(&[[1u8; 32], [3u8; 32]]);
    assert_ne!(r1, r2);
}

#[test]
fn p3_empty_validator_set_is_zero() {
    let empty: Vec<[u8; 32]> = vec![];
    assert_eq!(amun_merkle::validator_set_root(&empty), [0u8; 32]);
}
