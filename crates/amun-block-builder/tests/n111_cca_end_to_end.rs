use amun_accounts::AccountStore;
use amun_block_builder::BlockBuilder;
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
fn n111_cca_state_root_preserved_through_block() {
    let mut builder = BlockBuilder::new();
    let mut mempool = Mempool::new();

    let a1 = {
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        sk.verifying_key().to_bytes()
    };
    let a2 = [2u8; 32];

    builder.engine.state.create_account(a1, 1000);
    mempool
        .add_transaction(create_signed_transfer(1, 1, 300, a2))
        .unwrap();

    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
    let expected_state_root = builder.engine.state.state_root();

    assert_eq!(
        block.state_root, expected_state_root,
        "Block.state_root must equal AccountStore::state_root() after CCA injection"
    );

    eprintln!(
        "n111 OK: state_root={:02x?}.. block_hash={:02x?}.. height={}",
        &expected_state_root[..4],
        &block.block_hash()[..4],
        block.height
    );
}

#[test]
fn n111_cca_state_root_changes_reflected_in_block() {
    let mut builder1 = BlockBuilder::new();
    let mut builder2 = BlockBuilder::new();

    let a = {
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        sk.verifying_key().to_bytes()
    };

    builder1.engine.state.create_account(a, 1000);
    builder2.engine.state.create_account(a, 999);

    let block1 = builder1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
    let block2 = builder2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);

    assert_ne!(
        block1.state_root, block2.state_root,
        "Different economic state must produce different CCA state roots"
    );
    assert_ne!(
        block1.block_hash(), block2.block_hash(),
        "Different CCA state roots must produce different block hashes"
    );
}

#[test]
fn n111_cca_raw_state_root_differs_from_cca_state_root() {
    let mut store = AccountStore::new();
    let a = [1u8; 32];
    store.create_account(a, 1000);

    let raw = store.raw_state_root();
    let cca = store.state_root();

    assert_ne!(
        raw, cca,
        "CCA state_root must differ from raw account root"
    );
}
