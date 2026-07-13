use serde::{Deserialize, Serialize};

use crate::ResourceId;

/// Derivation type for lineage tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivationType {
    Genesis,
    SingleAncestor,
    MultiAncestor,
    Transformation,
    Split,
    Merge,
    CrossContractSuccessor,
}

/// Tracks how a resource was derived from its parents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLineage {
    pub resource_id: ResourceId,
    pub parent_resource_ids: Vec<ResourceId>,
    pub parent_hashes: Vec<[u8; 32]>,
    pub derivation: DerivationType,
    pub derivation_index: u32,
    pub version: u64,
}

impl ResourceLineage {
    pub fn genesis(resource_id: ResourceId) -> Self {
        Self {
            resource_id,
            parent_resource_ids: vec![],
            parent_hashes: vec![],
            derivation: DerivationType::Genesis,
            derivation_index: 0,
            version: 1,
        }
    }

    pub fn single_ancestor(
        resource_id: ResourceId,
        parent_id: ResourceId,
        parent_hash: [u8; 32],
        version: u64,
    ) -> Self {
        Self {
            resource_id,
            parent_resource_ids: vec![parent_id],
            parent_hashes: vec![parent_hash],
            derivation: DerivationType::SingleAncestor,
            derivation_index: 0,
            version,
        }
    }

    pub fn transformation(
        resource_id: ResourceId,
        parent_id: ResourceId,
        parent_hash: [u8; 32],
        version: u64,
    ) -> Self {
        Self {
            resource_id,
            parent_resource_ids: vec![parent_id],
            parent_hashes: vec![parent_hash],
            derivation: DerivationType::Transformation,
            derivation_index: 0,
            version,
        }
    }

    pub fn split(
        resource_id: ResourceId,
        parent_id: ResourceId,
        parent_hash: [u8; 32],
        version: u64,
        index: u32,
    ) -> Self {
        Self {
            resource_id,
            parent_resource_ids: vec![parent_id],
            parent_hashes: vec![parent_hash],
            derivation: DerivationType::Split,
            derivation_index: index,
            version,
        }
    }

    pub fn merge(
        resource_id: ResourceId,
        parent_ids: Vec<ResourceId>,
        parent_hashes: Vec<[u8; 32]>,
        version: u64,
    ) -> Self {
        Self {
            resource_id,
            parent_resource_ids: parent_ids,
            parent_hashes,
            derivation: DerivationType::Merge,
            derivation_index: 0,
            version,
        }
    }

    pub fn cross_contract_successor(
        resource_id: ResourceId,
        parent_id: ResourceId,
        parent_hash: [u8; 32],
        version: u64,
    ) -> Self {
        Self {
            resource_id,
            parent_resource_ids: vec![parent_id],
            parent_hashes: vec![parent_hash],
            derivation: DerivationType::CrossContractSuccessor,
            derivation_index: 0,
            version,
        }
    }
}
