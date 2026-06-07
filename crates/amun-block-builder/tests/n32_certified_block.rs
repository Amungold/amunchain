use amun_block_builder::{BlockBuilder, Block};
use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
use amun_mempool::Mempool;
use amun_consensus::types::QuorumCertificate;
use ed25519_dalek::SigningKey;

/// A block that has achieved quorum consensus.
#[derive(Debug, Clone)]
pub struct CertifiedBlock {
    pub block: Block,
    pub quorum_certificate: QuorumCertificate,
}

impl CertifiedBlock {
    pub fn new(block: Block, qc: QuorumCertificate) -> Self {
        Self { block, quorum_certificate: qc }
    }
}

#[test]
fn n32_certified_block_created() {
    let mut builder = BlockBuilder::new();

    // Setup accounts
    let alice_seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&alice_seed); sk.verifying_key().to_bytes() };
    let bob = [2u8; 32];
    builder.engine.state.create_account(alice, 1000);

    // Create and sign transaction
    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version: 1, sender: alice, nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload { to: bob, amount: 300 }),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&alice_seed));
    mempool.add_transaction(tx).unwrap();

    // Build the block
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);

    // Validators form a Quorum Certificate
    let qc = QuorumCertificate {
        height: block.height,
        block_hash: block.block_hash(),
        round: 0,
        aggregated_signature: vec![1u8; 64],
        signers_bitmap: vec![0xFF, 0x00, 0x00, 0x00],
    };

    // Create a certified block
    let certified = CertifiedBlock::new(block.clone(), qc);

    // Verify the certified block
    assert_eq!(certified.block.height, 1);
    assert_eq!(certified.quorum_certificate.height, 1);
    assert_eq!(certified.quorum_certificate.block_hash, block.block_hash());
    assert_eq!(certified.block.transactions.len(), 1);
    assert!(certified.block.receipts[0].success);
    assert_eq!(builder.engine.state.balance_of(&alice), 700);
    assert_eq!(builder.engine.state.balance_of(&bob), 300);
}

#[test]
fn n32_multiple_certified_blocks() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&seed); sk.verifying_key().to_bytes() };
    builder.engine.state.create_account(alice, 1000);

    // Block 1
    let mut mempool = Mempool::new();
    let mut tx1 = Transaction {
        version: 1, sender: alice, nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload { to: [2u8; 32], amount: 100 }),
        signature: vec![],
    };
    tx1.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx1).unwrap();
    let block1 = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
    let qc1 = QuorumCertificate {
        height: 1, block_hash: block1.block_hash(), round: 0,
        aggregated_signature: vec![1u8; 64], signers_bitmap: vec![0xFF, 0x00, 0x00, 0x00],
    };
    let certified1 = CertifiedBlock::new(block1, qc1);

    // Block 2
    let mut tx2 = Transaction {
        version: 1, sender: alice, nonce: 2,
        payload: TransactionPayload::Transfer(TransferPayload { to: [3u8; 32], amount: 200 }),
        signature: vec![],
    };
    tx2.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx2).unwrap();
    let block2 = builder.build_block(2, certified1.block.block_hash(), &mut mempool, 100, [9u8; 32], 2000);
    let qc2 = QuorumCertificate {
        height: 2, block_hash: block2.block_hash(), round: 0,
        aggregated_signature: vec![2u8; 64], signers_bitmap: vec![0xFF, 0x00, 0x00, 0x00],
    };
    let certified2 = CertifiedBlock::new(block2, qc2);

    // Verify chain continuity
    assert_eq!(certified2.block.parent_hash, certified1.block.block_hash());
    assert_ne!(certified1.quorum_certificate.block_hash, certified2.quorum_certificate.block_hash);
    assert_eq!(builder.engine.state.balance_of(&alice), 700);
}

#[test]
fn n32_certified_block_preserves_state_root() {
    let mut builder = BlockBuilder::new();
    let seed = [1u8; 32];
    let alice = { let sk = SigningKey::from_bytes(&seed); sk.verifying_key().to_bytes() };
    builder.engine.state.create_account(alice, 1000);
    let state_root_before = builder.engine.state.state_root();

    let mut mempool = Mempool::new();
    let mut tx = Transaction {
        version: 1, sender: alice, nonce: 1,
        payload: TransactionPayload::Transfer(TransferPayload { to: [2u8; 32], amount: 300 }),
        signature: vec![],
    };
    tx.sign(&SigningKey::from_bytes(&seed));
    mempool.add_transaction(tx).unwrap();
    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
    let qc = QuorumCertificate {
        height: 1, block_hash: block.block_hash(), round: 0,
        aggregated_signature: vec![1u8; 64], signers_bitmap: vec![0xFF, 0x00, 0x00, 0x00],
    };
    let certified = CertifiedBlock::new(block, qc);

    // State root must change after economic activity
    assert_ne!(certified.block.state_root, state_root_before);
    // Block's state root must match the engine's current state
    assert_eq!(certified.block.state_root, builder.engine.state.state_root());
}
