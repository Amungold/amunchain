use amun_evidence_root::{EvidenceRoot, EvidenceChain};
use amun_block_builder::BlockBuilder;
use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
use amun_mempool::Mempool;
use ed25519_dalek::SigningKey;

#[test]
fn n40_evidence_backed_block_created() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&seed); sk.verifying_key().to_bytes() };
    builder.engine.state.create_account(alice, 1000);
    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version: 1, sender: alice, nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload { to: [2u8; 32], amount: 300 }),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx).unwrap();
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
    let evidence = EvidenceRoot::compute(
        block.state_root, block.block_hash(), [0u8; 32], [0u8; 32], [0u8; 32], block.height,
    );
    assert!(evidence.verify());
    assert_eq!(evidence.state_root, block.state_root);
    assert_eq!(evidence.height, 1);
}

#[test]
fn n40_evidence_chain_across_blocks() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&seed); sk.verifying_key().to_bytes() };
    builder.engine.state.create_account(alice, 1000);
    let mut chain = EvidenceChain::new();
    let mut mempool = Mempool::new();
    for i in 1..=3 {
        let mut tx = Transaction {
            version:1, sender:alice, nonce:i,
            payload: TransactionPayload::Transfer(TransferPayload{to:[i as u8;32], amount:100}),
            signature: vec![],
        };
        tx.sign(&SigningKey::from_bytes(&seed));
        mempool.add_transaction(tx).unwrap();
        let parent = if i == 1 { [0u8; 32] } else { [i as u8 - 1; 32] };
        let b = builder.build_block(i, parent, &mut mempool, 100, [9u8;32], i*1000);
        chain.append(b.state_root, b.block_hash(), [0u8;32], [0u8;32], i);
    }
    assert!(chain.verify());
    assert_eq!(chain.len(), 3);
}

#[test]
fn n40_evidence_root_changes_with_state() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&seed); sk.verifying_key().to_bytes() };
    builder.engine.state.create_account(alice, 1000);
    let before = builder.engine.state.state_root();
    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version:1, sender:alice, nonce:1,
        payload: TransactionPayload::Transfer(TransferPayload{to:[2u8;32], amount:100}),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx).unwrap();
    builder.build_block(1, [0u8;32], &mut mempool, 100, [9u8;32], 1000);
    let after = builder.engine.state.state_root();
    let e1 = EvidenceRoot::compute(before, [0u8;32], [0u8;32], [0u8;32], [0u8;32], 1);
    let e2 = EvidenceRoot::compute(after, [0u8;32], [0u8;32], [0u8;32], [0u8;32], 1);
    assert_ne!(e1.root, e2.root);
}
