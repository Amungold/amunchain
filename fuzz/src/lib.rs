pub mod snapshot_fuzz {
    use amun_chain_position::ChainPosition;
    use amun_snapshot_constitution::snapshot::{CanonicalSnapshot, SnapshotExecutionContext};

    pub fn fuzz_snapshot_import(data: &[u8]) {
        if data.len() < 72 {
            return;
        }
        let mut genesis_root = [0u8; 32];
        genesis_root.copy_from_slice(&data[0..32]);
        let position = ChainPosition {
            epoch: u64::from_le_bytes(data[32..40].try_into().unwrap_or([0u8; 8])),
            sequence: u64::from_le_bytes(data[40..48].try_into().unwrap_or([0u8; 8])),
        };
        let mut state_root = [0u8; 32];
        if data.len() >= 80 {
            state_root.copy_from_slice(&data[48..80]);
        }
        let remaining = if data.len() > 80 { &data[80..] } else { &[] };
        let mut entries: Vec<([u8; 32], Vec<u8>)> = Vec::new();
        let mut offset = 0;
        while offset + 33 <= remaining.len() && entries.len() < 1000 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&remaining[offset..offset + 32]);
            let val_len = remaining[offset + 32] as usize;
            offset += 33;
            if offset + val_len <= remaining.len() {
                entries.push((key, remaining[offset..offset + val_len].to_vec()));
                offset += val_len;
            } else {
                break;
            }
        }
        let context = SnapshotExecutionContext {
            genesis_root,
            current_position: position,
            current_epoch: position.epoch,
            epoch_seal_hash: Some([0u8; 32]),
            execution_version: 1,
            sealed_epochs: vec![],
        };
        if let Ok(snapshot) = CanonicalSnapshot::new(position, state_root, entries, context) {
            assert!(snapshot.verify(), "verify() failed after successful new()");
        }
    }
}

pub mod resource_fuzz {
    use amun_resource_core::{
        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
        ResourceState,
    };

    /// Fuzz the Resource Registry: genesis registration + consume_and_derive.
    /// This directly targets the get_mut().unwrap() path.
    pub fn fuzz_resource_registry(data: &[u8]) {
        if data.len() < 8 {
            return;
        }

        let mut registry = ResourceRegistry::new(10000);
        let num_ops = (data[0] as usize).min(50);
        let mut offset = 1;
        let mut last_id: Option<ResourceId> = None;

        for _ in 0..num_ops {
            if offset + 32 > data.len() {
                break;
            }

            let mut rid_bytes = [0u8; 32];
            rid_bytes.copy_from_slice(&data[offset..offset + 32]);
            let resource_id = ResourceId(rid_bytes);
            offset += 32;

            // Alternate between genesis and derivation
            if last_id.is_none() || (data[offset.wrapping_sub(1)].is_multiple_of(2)) {
                // Try genesis registration
                let meta = ResourceMetadata {
                    resource_id,
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(resource_id),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                };
                let _ = registry.register_genesis(meta);
                last_id = Some(resource_id);
            } else if let Some(parent_id) = last_id {
                // Try derivation from last registered resource
                let parent = match registry.get(&parent_id) {
                    Some(p) => p,
                    None => continue,
                };
                if !matches!(parent.state, ResourceState::Active) {
                    continue;
                }

                let parent_hash = ResourceRegistry::hash_resource(parent);
                let version = parent.lineage.version.saturating_add(1);

                let child_meta = ResourceMetadata {
                    resource_id,
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(
                        resource_id,
                        parent_id,
                        parent_hash,
                        version,
                    ),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                };
                // This is the critical path — consume_and_derive with fuzzed data
                let _ = registry.consume_and_derive(&parent_id, child_meta);
                last_id = Some(resource_id);
            }
        }

        // Verify registry consistency after fuzzing
        let _ = registry.compute_state_root();
        let _ = registry.total_active();
    }
}

pub mod wal_fuzz {
    pub fn fuzz_wal_roundtrip(data: &[u8]) {
        if let Ok(dir) = tempfile::tempdir() {
            let base_path = dir.path().to_str().unwrap_or("");
            let wal_dir = format!("{}/wal_test", base_path);
            let _ = std::fs::create_dir_all(&wal_dir);
            let seg_path = format!("{}/segment_0000000001.wal", wal_dir);
            let _ = std::fs::write(&seg_path, data);
            let _ = amun_wal::WriteAheadLog::open(&wal_dir);
        }
    }
}
