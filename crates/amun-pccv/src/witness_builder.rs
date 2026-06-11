use crate::enhanced_proof::{LineageProof, MerkleProof, WitnessBundle};
use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};

pub struct WitnessBuilder;

impl WitnessBuilder {
    pub fn build(
        registry: &ResourceRegistry,
        consumed_ids: &[ResourceId],
        produced_meta: &[ResourceMetadata],
    ) -> WitnessBundle {
        let mut consumed_proofs = Vec::new();
        let mut lineage_proofs = Vec::new();

        for resource_id in consumed_ids {
            if let Some(_metadata) = registry.get(resource_id) {
                let state_root = registry.compute_state_root();
                let merkle_proof = Self::build_merkle_proof(registry, resource_id, state_root);
                consumed_proofs.push(merkle_proof);

                let lineage_proof = Self::build_lineage_proof(registry, resource_id);
                lineage_proofs.push(lineage_proof);
            }
        }

        WitnessBundle {
            consumed_proofs,
            lineage_proofs,
            produced_metadata: produced_meta.to_vec(),
        }
    }

    fn build_merkle_proof(
        registry: &ResourceRegistry,
        resource_id: &ResourceId,
        state_root: [u8; 32],
    ) -> MerkleProof {
        let siblings = Self::compute_siblings(registry, resource_id);
        MerkleProof {
            resource_id: *resource_id,
            state_root,
            siblings,
        }
    }

    fn compute_siblings(
        registry: &ResourceRegistry,
        resource_id: &ResourceId,
    ) -> Vec<([u8; 32], bool)> {
        let mut siblings = Vec::new();
        let active_ids = registry.active_ids();
        if active_ids.is_empty() {
            return siblings;
        }

        // Build leaf hashes for all active resources
        let leaves: Vec<([u8; 32], ResourceId)> = active_ids
            .iter()
            .filter_map(|id| {
                registry
                    .get(id)
                    .map(|meta| (ResourceRegistry::hash_resource(meta), *id))
            })
            .collect();

        // Find index of target resource
        let target_idx = leaves.iter().position(|(_, id)| id == resource_id);
        if target_idx.is_none() {
            return siblings;
        }
        let mut idx = target_idx.unwrap();
        let mut level: Vec<[u8; 32]> = leaves.iter().map(|(hash, _)| *hash).collect();

        // Walk up the Merkle tree
        while level.len() > 1 {
            let is_left = idx % 2 == 0;
            let sibling_idx = if is_left { idx + 1 } else { idx - 1 };

            if sibling_idx < level.len() {
                siblings.push((level[sibling_idx], !is_left));
            }

            let mut next_level = Vec::new();
            for chunk in level.chunks(2) {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"AMUN_MERKLE_NODE_V1");
                hasher.update(&chunk[0]);
                if chunk.len() == 2 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                let hash = hasher.finalize();
                let mut h = [0u8; 32];
                h.copy_from_slice(hash.as_bytes());
                next_level.push(h);
            }
            idx /= 2;
            level = next_level;
        }

        siblings
    }

    fn build_lineage_proof(registry: &ResourceRegistry, resource_id: &ResourceId) -> LineageProof {
        let mut chain = Vec::new();
        let mut current_id = *resource_id;
        let mut genesis_id = *resource_id;

        while let Some(metadata) = registry.get(&current_id) {
            if metadata.lineage.parent_resource_ids.is_empty() {
                genesis_id = current_id;
                break;
            }
            let parent_id = metadata.lineage.parent_resource_ids[0];
            let parent_hash = if metadata.lineage.parent_hashes.len() == 1 {
                metadata.lineage.parent_hashes[0]
            } else {
                [0u8; 32]
            };
            chain.push((parent_id, parent_hash));
            current_id = parent_id;
        }

        LineageProof {
            resource_id: *resource_id,
            genesis_id,
            chain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{ResourceArchetype, ResourceLineage, ResourceMetadata, ResourceState};

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    #[test]
    fn w21_build_witness_from_registry() {
        let mut reg = ResourceRegistry::new(1000);
        let id = make_id(1);
        reg.register_genesis(ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        })
        .unwrap();

        let bundle = WitnessBuilder::build(&reg, &[id], &[]);
        assert_eq!(bundle.lineage_proofs.len(), 1);
        assert_eq!(bundle.lineage_proofs[0].resource_id, id);
        assert_eq!(bundle.lineage_proofs[0].genesis_id, id);
        assert!(bundle.lineage_proofs[0].chain.is_empty());
    }

    #[test]
    fn w21_lineage_proof_follows_chain() {
        let mut reg = ResourceRegistry::new(1000);
        let root = make_id(1);
        let child = make_id(2);
        let grandchild = make_id(3);

        reg.register_genesis(ResourceMetadata {
            resource_id: root,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(root),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        })
        .unwrap();

        let root_hash = ResourceRegistry::hash_resource(reg.get(&root).unwrap());
        reg.consume_and_derive(
            &root,
            ResourceMetadata {
                resource_id: child,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(child, root, root_hash, 2),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            },
        )
        .unwrap();

        let child_hash = ResourceRegistry::hash_resource(reg.get(&child).unwrap());
        reg.consume_and_derive(
            &child,
            ResourceMetadata {
                resource_id: grandchild,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(grandchild, child, child_hash, 3),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            },
        )
        .unwrap();

        let bundle = WitnessBuilder::build(&reg, &[grandchild], &[]);
        assert_eq!(bundle.lineage_proofs.len(), 1);
        let proof = &bundle.lineage_proofs[0];
        assert_eq!(proof.resource_id, grandchild);
        assert_eq!(proof.genesis_id, root);
        assert_eq!(proof.chain.len(), 2);
    }

    #[test]
    fn w21_merkle_proof_has_siblings() {
        let mut reg = ResourceRegistry::new(1000);
        for i in 0..10u8 {
            reg.register_genesis(ResourceMetadata {
                resource_id: make_id(i),
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(make_id(i)),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            })
            .unwrap();
        }
        let bundle = WitnessBuilder::build(&reg, &[make_id(0)], &[]);
        assert_eq!(bundle.consumed_proofs.len(), 1);
        // With 10 leaves, the Merkle proof should have at least 1 sibling
        assert!(
            !bundle.consumed_proofs[0].siblings.is_empty(),
            "Merkle proof for 10-leaf tree must have siblings"
        );
    }
}
