use amun_block_builder::BlockBuilder;
use amun_mempool::Mempool;
use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
use ed25519_dalek::SigningKey;

#[test]
fn n28_first_economic_block() {
    let mut builder = BlockBuilder::new();
    let alice_seed = [1u8; 32];
    let alice = {
        let sk = SigningKey::from_bytes(&alice_seed);
        sk.verifying_key().to_bytes()
    };
    let bob = [2u8; 32];
    builder.engine.state.create_account(alice, 1000);
    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version: 1,
        sender: alice,
        nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: bob,
            amount: 300,
        }),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&alice_seed));
    mempool.add_transaction(tx).unwrap();
    let genesis_root = builder.engine.state.state_root();
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
    assert_eq!(builder.engine.state.balance_of(&alice), 700);
    assert_eq!(builder.engine.state.balance_of(&bob), 300);
    assert_eq!(block.transactions.len(), 1);
    assert_eq!(block.receipts.len(), 1);
    assert!(block.receipts[0].success);
    assert_ne!(block.state_root, genesis_root);
    assert_ne!(block.block_hash(), [0u8; 32]);
}

#[test]
fn n28_multiple_transfers_in_block() {
    let mut builder = BlockBuilder::new();
    let s1 = [1u8; 32];
    let s2 = [3u8; 32];
    let a1 = {
        let sk = SigningKey::from_bytes(&s1);
        sk.verifying_key().to_bytes()
    };
    let a2 = {
        let sk = SigningKey::from_bytes(&s2);
        sk.verifying_key().to_bytes()
    };
    let a3 = [5u8; 32];
    builder.engine.state.create_account(a1, 1000);
    builder.engine.state.create_account(a2, 500);
    let mut mempool = Mempool::new();
    let mut tx1 = Transaction {
        version: 1,
        sender: a1,
        nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: a3,
            amount: 200,
        }),
        signature: vec![],
    };
    tx1.sign(&SigningKey::from_bytes(&s1));
    mempool.add_transaction(tx1).unwrap();
    let mut tx2 = Transaction {
        version: 1,
        sender: a2,
        nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: a3,
            amount: 150,
        }),
        signature: vec![],
    };
    tx2.sign(&SigningKey::from_bytes(&s2));
    mempool.add_transaction(tx2).unwrap();
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
    assert_eq!(block.transactions.len(), 2);
    assert!(block.receipts[0].success);
    assert_eq!(builder.engine.state.balance_of(&a1), 800);
    assert_eq!(builder.engine.state.balance_of(&a3), 350);
}

#[test]
fn n28_failed_transaction_in_block() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = {
        let sk = SigningKey::from_bytes(&seed);
        sk.verifying_key().to_bytes()
    };
    builder.engine.state.create_account(alice, 100);
    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version: 1,
        sender: alice,
        nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: [2u8; 32],
            amount: 500,
        }),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx).unwrap();
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
    assert!(!block.receipts[0].success);
    assert_eq!(block.receipts[0].error_code, Some(3));
    assert_eq!(builder.engine.state.balance_of(&alice), 100);
}
