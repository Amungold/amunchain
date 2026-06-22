use amun_resource_core::ResourceId;
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

pub struct ContractEvent {
    pub contract_id: ResourceId,
    pub event_name: String,
    pub event_data: Vec<u8>,
    pub block_height: u64,
    pub event_hash: [u8; 32],
}

pub struct ContractStorage {
    pub storage: BTreeMap<[u8; 32], Vec<u8>>,
    pub events: Vec<ContractEvent>,
}

impl ContractStorage {
    pub fn new() -> Self {
        Self { storage: BTreeMap::new(), events: Vec::new() }
    }

    pub fn store(&mut self, key: [u8; 32], value: Vec<u8>) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }

    pub fn emit_event(
        &mut self,
        contract_id: ResourceId,
        event_name: String,
        event_data: Vec<u8>,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_CONTRACT_EVENT_V1");
        hasher.update(&contract_id.0);
        hasher.update(event_name.as_bytes());
        hasher.update(&event_data);
        hasher.update(&block_height.to_le_bytes());
        let event_hash: [u8; 32] = hasher.finalize().into();

        self.events.push(ContractEvent {
            contract_id,
            event_name,
            event_data,
            block_height,
            event_hash,
        });

        event_hash
    }

    pub fn get_events_by_contract(&self, contract_id: &ResourceId) -> Vec<&ContractEvent> {
        self.events.iter().filter(|e| e.contract_id == *contract_id).collect()
    }

    pub fn compute_events_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_CONTRACT_EVENTS_ROOT_V1");
        for event in &self.events {
            hasher.update(&event.event_hash);
        }
        hasher.finalize().into()
    }
}
