use amun_nft_bridge::{BridgeLock, BridgeUnlock, BridgeLedger};

#[test]
fn n139_lock_and_unlock_flow() {
    let mut ledger = BridgeLedger::new();
    let lock = BridgeLock {
        source_chain: 1,
        token_id: [10u8; 32],
        owner: [100u8; 32],
        destination_chain: 2,
        destination_owner: [200u8; 32],
        lock_height: 42,
    };
    let lock_id = ledger.lock(lock);
    assert!(ledger.is_locked(&lock_id));

    let unlock = BridgeUnlock {
        lock_id,
        destination_chain: 2,
        new_owner: [200u8; 32],
        unlock_height: 100,
    };
    let record = ledger.unlock(unlock).unwrap();
    assert!(!ledger.is_locked(&lock_id));
    assert_eq!(record.lock.source_chain, 1);
    assert_eq!(record.unlock.as_ref().unwrap().new_owner, [200u8; 32]);
}

#[test]
fn n139_unlock_without_lock_fails() {
    let mut ledger = BridgeLedger::new();
    let result = ledger.unlock(BridgeUnlock {
        lock_id: [99u8; 32],
        destination_chain: 2,
        new_owner: [200u8; 32],
        unlock_height: 100,
    });
    assert!(result.is_none());
}

#[test]
fn n139_double_lock_differs() {
    let mut ledger = BridgeLedger::new();
    let lock1 = BridgeLock {
        source_chain: 1, token_id: [1u8; 32], owner: [10u8; 32],
        destination_chain: 2, destination_owner: [20u8; 32], lock_height: 1,
    };
    let lock2 = BridgeLock {
        source_chain: 3, token_id: [1u8; 32], owner: [10u8; 32],
        destination_chain: 2, destination_owner: [20u8; 32], lock_height: 1,
    };
    let id1 = ledger.lock(lock1);
    let id2 = ledger.lock(lock2);
    assert_ne!(id1, id2);
    assert!(ledger.is_locked(&id1));
    assert!(ledger.is_locked(&id2));
}

#[test]
fn n139_deterministic_bridge_root() {
    let mut l1 = BridgeLedger::new();
    let mut l2 = BridgeLedger::new();
    let lock = BridgeLock {
        source_chain: 1, token_id: [5u8; 32], owner: [50u8; 32],
        destination_chain: 2, destination_owner: [60u8; 32], lock_height: 10,
    };
    l1.lock(lock.clone());
    l2.lock(lock);
    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
}

#[test]
fn n139_bridge_root_changes_after_unlock() {
    let mut l1 = BridgeLedger::new();
    let mut l2 = BridgeLedger::new();
    let lock = BridgeLock {
        source_chain: 1, token_id: [7u8; 32], owner: [70u8; 32],
        destination_chain: 2, destination_owner: [80u8; 32], lock_height: 20,
    };
    let id1 = l1.lock(lock.clone());
    let _id2 = l2.lock(lock);
    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
    l1.unlock(BridgeUnlock { lock_id: id1, destination_chain: 2, new_owner: [80u8; 32], unlock_height: 30 });
    assert_ne!(l1.compute_bridge_root(), l2.compute_bridge_root());
}
