use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceRegistry,
};
use std::collections::BTreeMap;

pub struct IncrementalSnapshot {
    pub base_state_root: [u8; 32],
    pub delta: BTreeMap<ResourceId, ResourceMetadata>,
    pub version: u64,
}

impl IncrementalSnapshot {
    pub fn new(base_state_root: [u8; 32], version: u64) -> Self {
        Self { base_state_root, delta: BTreeMap::new(), version }
    }

    pub fn add_delta(&mut self, meta: ResourceMetadata) {
        self.delta.insert(meta.resource_id, meta);
    }

    pub fn apply_to(&self, registry: &mut ResourceRegistry) {
        for meta in self.delta.values() {
            registry.register_genesis(meta.clone()).ok();
        }
    }

    pub fn delta_count(&self) -> usize {
        self.delta.len()
    }
}

pub fn compress_snapshot(registry: &ResourceRegistry) -> (Vec<ResourceMetadata>, [u8; 32]) {
    let active: Vec<ResourceMetadata> = registry.active_resources().iter().map(|&m| m.clone()).collect();
    let state_root = registry.compute_state_root();
    (active, state_root)
}

pub fn restore_from_compressed(compressed: &[ResourceMetadata]) -> ResourceRegistry {
    let mut reg = ResourceRegistry::new(compressed.len() * 2);
    
    // Pass 1: register all genesis resources
    for meta in compressed {
        if meta.lineage.parent_resource_ids.is_empty() {
            reg.register_genesis(meta.clone()).ok();
        }
    }
    
    // Pass 2: derive children from their parents
    for meta in compressed {
        if !meta.lineage.parent_resource_ids.is_empty() {
            let parent_id = meta.lineage.parent_resource_ids[0];
            reg.consume_and_derive(&parent_id, meta.clone()).ok();
        }
    }
    
    reg
}
