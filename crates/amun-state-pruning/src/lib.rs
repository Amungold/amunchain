use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use std::collections::BTreeMap;

pub struct PrunableRegistry {
    pub registry: ResourceRegistry,
    pub archived_resources: BTreeMap<ResourceId, ResourceMetadata>,
    pub pruned_count: u64,
}

impl PrunableRegistry {
    pub fn new(max_lineage_depth: usize) -> Self {
        Self {
            registry: ResourceRegistry::new(max_lineage_depth),
            archived_resources: BTreeMap::new(),
            pruned_count: 0,
        }
    }

    pub fn archive_by_height(&mut self, max_height: u64) -> u64 {
        let to_archive: Vec<ResourceId> = self.registry.active_resources()
            .iter()
            .filter(|m| m.lineage.version <= max_height)
            .map(|m| m.resource_id)
            .collect();
        let count = to_archive.len() as u64;
        for id in &to_archive {
            if let Some(meta) = self.registry.get(id).cloned() {
                self.archived_resources.insert(*id, meta);
                self.pruned_count += 1;
            }
        }
        count
    }

    pub fn restore_archived(&mut self) -> u64 {
        let mut restored = 0u64;
        let ids: Vec<ResourceId> = self.archived_resources.keys().cloned().collect();
        for id in &ids {
            if let Some(meta) = self.archived_resources.remove(id) {
                self.registry.register_genesis(meta).ok();
                restored += 1;
            }
        }
        self.pruned_count -= restored;
        restored
    }

    pub fn compute_pruned_root(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_PRUNED_STATE_V1");
        hasher.update(&self.registry.compute_state_root());
        hasher.update(&self.pruned_count.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn total_active(&self) -> usize {
        self.registry.total_active()
    }
}
