use amun_block_builder::BlockBuilder;
use amun_constitutional_commitment::EconomicTree;
use amun_mempool::Mempool;
use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
use ed25519_dalek::SigningKey;

fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
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

#[test]
fn n132_3_6_economic_root_consistent_with_snapshot() {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();

    let a1 = {
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        sk.verifying_key().to_bytes()
    };

    let a2 = [2u8; 32];

    builder.engine.state.create_account(a1, 10000);

    mempool
        .add_transaction(create_signed_transfer(1, 1, 500, a2))
        .unwrap();

    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);

    let recomputed_economic =
        EconomicTree::root(&block.economic_snapshot).expect("economic root must be computable");

    assert_eq!(
        block.economic_root, recomputed_economic,
        "economic_root must match recomputed root from snapshot"
    );
}

#[test]
fn n132_3_6_consistent_with_snapshot() {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();

    let a1 = {
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        sk.verifying_key().to_bytes()
    };

    let a2 = [2u8; 32];

    builder.engine.state.create_account(a1, 10000);

    mempool
        .add_transaction(create_signed_transfer(1, 1, 500, a2))
        .unwrap();

    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);

    // Recompute the complete constitutional roots using the same canonical
    // pipeline as BlockBuilder.
    let expected = builder
        .engine
        .state
        .constitutional_roots_with_ledger(&builder.engine.economic);

    assert_eq!(
        block.constitutional_root, expected.constitutional_root,
        " mismatch",
    );

    assert_eq!(
        block.constitutional_root, expected.constitutional_root,
        "constitutional_root mismatch",
    );

    assert_eq!(
        block.identity_root, expected.identity_root,
        "identity_root mismatch",
    );

    assert_eq!(
        block.economic_root, expected.economic_root,
        "economic_root mismatch",
    );
}

#[test]
fn n132_3_6_economic_change_changes_constitutional_roots() {
    let mut b1 = BlockBuilder::new();
    let mut b2 = BlockBuilder::new();

    let a = {
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        sk.verifying_key().to_bytes()
    };

    b1.engine.state.create_account(a, 10000);
    b2.engine.state.create_account(a, 10000);

    let mut mp1 = Mempool::new();

    mp1.add_transaction(create_signed_transfer(1, 1, 500, [2u8; 32]))
        .unwrap();

    let block1 = b1.build_block(1, [0u8; 32], &mut mp1, 10, [0u8; 32], 1000);

    let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);

    assert_ne!(block1.economic_root, block2.economic_root);
    assert_ne!(block1.constitutional_root, block2.constitutional_root);
}
