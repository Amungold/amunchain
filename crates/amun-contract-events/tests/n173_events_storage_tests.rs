use amun_resource_core::ResourceId;
use amun_contract_events::ContractStorage;

#[test]
fn n173_store_and_retrieve() {
    let mut storage = ContractStorage::new();
    let key = [1u8; 32];
    let value = vec![1, 2, 3];
    storage.store(key, value.clone());
    assert_eq!(storage.get(&key), Some(&value));
}

#[test]
fn n173_emit_and_query_events() {
    let mut storage = ContractStorage::new();
    let contract_id = ResourceId([5u8; 32]);
    storage.emit_event(contract_id, "Transfer".into(), vec![10, 20], 42);
    storage.emit_event(contract_id, "Approve".into(), vec![30, 40], 43);
    let events = storage.get_events_by_contract(&contract_id);
    assert_eq!(events.len(), 2);
}

#[test]
fn n173_events_root_deterministic() {
    let mut storage1 = ContractStorage::new();
    let mut storage2 = ContractStorage::new();
    let cid = ResourceId([1u8; 32]);
    storage1.emit_event(cid, "Mint".into(), vec![1], 1);
    storage2.emit_event(cid, "Mint".into(), vec![1], 1);
    assert_eq!(storage1.compute_events_root(), storage2.compute_events_root());
}
