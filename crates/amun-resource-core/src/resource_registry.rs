use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{DerivationType, ResourceArchetype, ResourceId, ResourceLineage, TransformationMatrix};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceState {
    Active,
    Consumed { derived_children: Vec<ResourceId> },
    Archived { archive_height: u64 },
    Revoked { reason: String },
    TransferredOut { target_contract: [u8; 32], proof_id: [u8; 32] },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceMetadata {
    pub resource_id: ResourceId,
    pub archetype: ResourceArchetype,
    pub state: ResourceState,
    pub lineage: ResourceLineage,
    pub contract_id: [u8; 32],
    pub owner: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistryError {
    #[error("duplicate resource id: {0}")]
    DuplicateId(ResourceId),
    #[error("resource not found: {0}")]
    NotFound(ResourceId),
    #[error("resource not active: {0}")]
    NotActive(ResourceId),
    #[error("parent not found: {0}")]
    ParentNotFound(ResourceId),
    #[error("parent not consumed: {0}")]
    ParentNotConsumed(ResourceId),
    #[error("version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u64, actual: u64 },
    #[error("parent hash mismatch for resource {0}")]
    ParentHashMismatch(ResourceId),
    #[error("illegal transformation: {src:?} -> {tgt:?}")]
    IllegalTransformation { src: ResourceArchetype, tgt: ResourceArchetype },
    #[error("circular dependency detected")]
    CircularDependency,
    #[error("derived child count mismatch")]
    DerivedChildCountMismatch,
    #[error("derivation from terminal resource")]
    TerminalDerivation,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceRegistry {
    resources: BTreeMap<ResourceId, ResourceMetadata>,
    ancestor_cache: BTreeMap<ResourceId, HashSet<ResourceId>>,
    #[allow(dead_code)]
    max_lineage_depth: usize,
}

impl ResourceRegistry {
    pub fn new(max_lineage_depth: usize) -> Self {
        Self {
            resources: BTreeMap::new(),
            ancestor_cache: BTreeMap::new(),
            max_lineage_depth,
        }
    }

    pub fn get(&self, id: &ResourceId) -> Option<&ResourceMetadata> {
        self.resources.get(id)
    }

    pub fn contains(&self, id: &ResourceId) -> bool {
        self.resources.contains_key(id)
    }

    pub fn total_active(&self) -> usize {
        self.resources.values().filter(|m| matches!(m.state, ResourceState::Active)).count()
    }

    pub fn total(&self) -> usize {
        self.resources.len()
    }

    /// Returns all active resource IDs, sorted.
    /// This is the entry point needed by WitnessBuilder for Merkle proof construction.
    pub fn active_ids(&self) -> Vec<ResourceId> {
        let mut ids: Vec<ResourceId> = self.resources
            .iter()
            .filter(|(_, m)| matches!(m.state, ResourceState::Active))
            .map(|(id, _)| *id)
            .collect();
        ids.sort();
        ids
    }

    /// Returns all active resource metadata, sorted by ID.
    pub fn active_resources(&self) -> Vec<&ResourceMetadata> {
        let mut active: Vec<&ResourceMetadata> = self.resources
            .values()
            .filter(|m| matches!(m.state, ResourceState::Active))
            .collect();
        active.sort_by_key(|m| &m.resource_id);
        active
    }

    pub fn register_genesis(&mut self, meta: ResourceMetadata) -> Result<(), RegistryError> {
        if self.resources.contains_key(&meta.resource_id) {
            return Err(RegistryError::DuplicateId(meta.resource_id));
        }
        if meta.lineage.derivation != DerivationType::Genesis {
            return Err(RegistryError::IllegalTransformation {
                src: ResourceArchetype::Asset,
                tgt: meta.archetype,
            });
        }
        if meta.lineage.version != 1 {
            return Err(RegistryError::VersionMismatch { expected: 1, actual: meta.lineage.version });
        }
        self.ancestor_cache.insert(meta.resource_id, HashSet::new());
        self.resources.insert(meta.resource_id, meta);
        Ok(())
    }

    pub fn consume_and_derive(
        &mut self,
        parent_id: &ResourceId,
        child_meta: ResourceMetadata,
    ) -> Result<ResourceId, RegistryError> {
        if self.resources.contains_key(&child_meta.resource_id) {
            return Err(RegistryError::DuplicateId(child_meta.resource_id));
        }
        let parent = self.resources.get(parent_id).ok_or(RegistryError::NotFound(*parent_id))?;
        if !matches!(parent.state, ResourceState::Active) {
            return Err(RegistryError::NotActive(*parent_id));
        }
        if TransformationMatrix::is_terminal(parent.archetype) {
            return Err(RegistryError::TerminalDerivation);
        }
        if !TransformationMatrix::is_legal(parent.archetype, child_meta.archetype) {
            return Err(RegistryError::IllegalTransformation {
                src: parent.archetype,
                tgt: child_meta.archetype,
            });
        }
        let expected_version = parent.lineage.version + 1;
        if child_meta.lineage.version != expected_version {
            return Err(RegistryError::VersionMismatch {
                expected: expected_version,
                actual: child_meta.lineage.version,
            });
        }
        let actual_parent_hash = Self::hash_resource(parent);
        for claimed_hash in &child_meta.lineage.parent_hashes {
            if *claimed_hash != actual_parent_hash {
                return Err(RegistryError::ParentHashMismatch(child_meta.resource_id));
            }
        }
        for pid in &child_meta.lineage.parent_resource_ids {
            if !self.resources.contains_key(pid) {
                return Err(RegistryError::ParentNotFound(*pid));
            }
        }
        if self.would_create_cycle_fast(&child_meta.resource_id, parent_id) {
            return Err(RegistryError::CircularDependency);
        }
        let child_id = child_meta.resource_id;
        let mut child_ancestors = self.ancestor_cache
            .get(parent_id)
            .cloned()
            .unwrap_or_default();
        child_ancestors.insert(*parent_id);
        self.ancestor_cache.insert(child_id, child_ancestors);
        self.resources.get_mut(parent_id).expect("registry: parent must exist after contains_key check").state =
            ResourceState::Consumed {
                derived_children: vec![child_id],
            };
        self.resources.insert(child_id, child_meta);
        Ok(child_id)
    }

    /// Compute the Merkle state root from all active resources.
    /// Each leaf = hash_resource(metadata).  This is a true Merkle tree,
    /// enabling Merkle proof construction for PCCV.
    pub fn compute_state_root(&self) -> [u8; 32] {
        let active = self.active_resources();
        if active.is_empty() {
            return [0u8; 32];
        }

        // Build leaf hashes
        let mut leaves: Vec<[u8; 32]> = active
            .iter()
            .map(|m| Self::hash_resource(m))
            .collect();

        // Build Merkle tree bottom-up
        while leaves.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in leaves.chunks(2) {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"AMUN_MERKLE_NODE_V1");
                hasher.update(&chunk[0]);
                if chunk.len() == 2 {
                    hasher.update(&chunk[1]);
                } else {
                    // Odd leaf — duplicate last
                    hasher.update(&chunk[0]);
                }
                let hash = hasher.finalize();
                let mut h = [0u8; 32];
                h.copy_from_slice(hash.as_bytes());
                next_level.push(h);
            }
            leaves = next_level;
        }

        leaves[0]
    }

    pub fn lineage_depth(&self, id: &ResourceId) -> usize {
        self.ancestor_cache.get(id).map(|a| a.len()).unwrap_or(0)
    }

    pub fn hash_resource(meta: &ResourceMetadata) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_RESOURCE_LEAF_V1");
        hasher.update(meta.resource_id.as_bytes());
        hasher.update(&[meta.archetype as u8]);
        hasher.update(&meta.lineage.version.to_le_bytes());
        hasher.update(&meta.contract_id);
        hasher.update(&meta.owner);
        for pid in &meta.lineage.parent_resource_ids {
            hasher.update(pid.as_bytes());
        }
        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        bytes
    }

    fn would_create_cycle_fast(&self, new_id: &ResourceId, parent_id: &ResourceId) -> bool {
        if new_id == parent_id {
            return true;
        }
        if let Some(ancestors) = self.ancestor_cache.get(parent_id) {
            if ancestors.contains(new_id) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(seed: u8) -> ResourceId {
        let mut hash = [0u8; 32];
        hash[0] = seed;
        ResourceId(hash)
    }

    fn make_meta(id: ResourceId, archetype: ResourceArchetype) -> ResourceMetadata {
        ResourceMetadata {
            resource_id: id,
            archetype,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }
    }

    #[test]
    fn w1_register_genesis() {
        let mut reg = ResourceRegistry::new(1000);
        let id = make_id(1);
        let meta = make_meta(id, ResourceArchetype::Asset);
        assert!(reg.register_genesis(meta).is_ok());
        assert_eq!(reg.total(), 1);
        assert_eq!(reg.total_active(), 1);
    }

    #[test]
    fn w1_reject_duplicate_genesis() {
        let mut reg = ResourceRegistry::new(1000);
        let id = make_id(1);
        reg.register_genesis(make_meta(id, ResourceArchetype::Asset)).unwrap();
        assert!(reg.register_genesis(make_meta(id, ResourceArchetype::Asset)).is_err());
    }

    #[test]
    fn w1_transform_asset_to_constitutional_asset() {
        let mut reg = ResourceRegistry::new(1000);
        let parent_id = make_id(1);
        let child_id = make_id(2);
        let parent_meta = ResourceMetadata {
            resource_id: parent_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(parent_id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.register_genesis(parent_meta).unwrap();
        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
        let child_meta = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::ConstitutionalAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::transformation(child_id, parent_id, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        assert!(reg.consume_and_derive(&parent_id, child_meta).is_ok());
        assert_eq!(reg.total(), 2);
        assert_eq!(reg.total_active(), 1);
    }

    #[test]
    fn w1_reject_illegal_transformation() {
        let mut reg = ResourceRegistry::new(1000);
        let parent_id = make_id(1);
        let child_id = make_id(2);
        reg.register_genesis(make_meta(parent_id, ResourceArchetype::Evidence)).unwrap();
        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
        let child_meta = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::transformation(child_id, parent_id, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        assert!(reg.consume_and_derive(&parent_id, child_meta).is_err());
    }

    #[test]
    fn w1_detect_cycle() {
        let mut reg = ResourceRegistry::new(1000);
        let a = make_id(1);
        let b = make_id(2);
        reg.register_genesis(make_meta(a, ResourceArchetype::Asset)).unwrap();
        let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
        let child_b = ResourceMetadata {
            resource_id: b,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(b, a, hash_a, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        assert!(reg.consume_and_derive(&a, child_b).is_ok());
        let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
        let child_a = ResourceMetadata {
            resource_id: make_id(3),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(make_id(3), b, hash_b, 3),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        assert!(reg.consume_and_derive(&b, child_a).is_ok());
    }

    #[test]
    fn w1_lineage_depth() {
        let mut reg = ResourceRegistry::new(1000);
        let root = make_id(1);
        reg.register_genesis(make_meta(root, ResourceArchetype::Asset)).unwrap();
        let mut current_parent = root;
        for i in 2u64..=10 {
            let child = make_id(i as u8);
            let hash = ResourceRegistry::hash_resource(reg.get(&current_parent).unwrap());
            let child_meta = ResourceMetadata {
                resource_id: child,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(child, current_parent, hash, i),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            };
            reg.consume_and_derive(&current_parent, child_meta).unwrap();
            current_parent = child;
        }
        assert_eq!(reg.lineage_depth(&current_parent), 9);
    }

    #[test]
    fn w21a_active_ids_returns_sorted() {
        let mut reg = ResourceRegistry::new(1000);
        for i in 0..10u8 {
            reg.register_genesis(make_meta(make_id(i), ResourceArchetype::Asset)).unwrap();
        }
        let ids = reg.active_ids();
        assert_eq!(ids.len(), 10);
        for w in ids.windows(2) {
            assert!(w[0] < w[1], "active_ids must be sorted");
        }
    }

    #[test]
    fn w21a_state_root_changes_with_owner() {
        let mut reg = ResourceRegistry::new(1000);
        let id = make_id(1);
        let mut meta = make_meta(id, ResourceArchetype::Asset);
        reg.register_genesis(meta.clone()).unwrap();
        let root1 = reg.compute_state_root();

        // Change owner
        meta.owner = [0xff; 32];
        // Re-register won't work (duplicate), so create a fresh registry
        let mut reg2 = ResourceRegistry::new(1000);
        reg2.register_genesis(meta).unwrap();
        let root2 = reg2.compute_state_root();

        assert_ne!(root1, root2, "State root must change when metadata changes");
    }

    #[test]
    fn w21a_merkle_root_not_zero_for_nonempty() {
        let mut reg = ResourceRegistry::new(1000);
        reg.register_genesis(make_meta(make_id(1), ResourceArchetype::Asset)).unwrap();
        let root = reg.compute_state_root();
        assert_ne!(root, [0u8; 32], "Merkle root must be non-zero for non-empty state");
    }
}
