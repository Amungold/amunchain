use amun_accounts::AccountStore;
use amun_block_builder::BlockBuilder;
use amun_constitutional_commitment::{
    commitment_root, compute_constitutional_root, ConstitutionalCommitment, ConstitutionalRoots,
    EconomicTree, EndBlockPipeline,
};
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

    // Recompute economic_root from the snapshot
    let recomputed_economic =
        EconomicTree::root(&block.economic_snapshot).expect("economic root must be computable");
    assert_eq!(
        block.economic_root, recomputed_economic,
        "economic_root must match recomputed root from snapshot"
    );
}

#[test]
fn n132_3_6_commitment_root_consistent_with_snapshot() {
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

    let identity_root: [u8; 32] = [0u8; 32];
    let evidence_root: [u8; 32] = [0u8; 32];
    let governance_root: [u8; 32] = [0u8; 32];

    let commitment = EndBlockPipeline::execute(
        identity_root,
        evidence_root,
        governance_root,
        &block.economic_snapshot,
    )
    .expect("commitment must be buildable");

    let recomputed_commitment = commitment_root(&commitment);
    let recomputed_constitutional = compute_constitutional_root(
        identity_root,
        evidence_root,
        governance_root,
        block.economic_root,
    );

    assert_eq!(
        block.commitment_root, recomputed_commitment,
        "commitment_root must match recomputed commitment"
    );
    assert_eq!(
        block.constitutional_root, recomputed_constitutional,
        "constitutional_root must match recomputed constitutional"
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

    // b1 has a transfer, b2 has none — different economic state
    let mut mp1 = Mempool::new();
    mp1.add_transaction(create_signed_transfer(1, 1, 500, [2u8; 32]))
        .unwrap();
    let block1 = b1.build_block(1, [0u8; 32], &mut mp1, 10, [0u8; 32], 1000);
    let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);

    // Different economic state must produce different roots
    assert_ne!(block1.economic_root, block2.economic_root);
    assert_ne!(block1.commitment_root, block2.commitment_root);
    assert_ne!(block1.constitutional_root, block2.constitutional_root);
}
