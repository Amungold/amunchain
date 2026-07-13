use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use std::time::Instant;

fn make_id(seed: u64) -> ResourceId {
    let mut hash = [0u8; 32];
    hash[0..8].copy_from_slice(&seed.to_le_bytes());
    ResourceId(hash)
}

fn make_genesis(id: ResourceId, archetype: ResourceArchetype) -> ResourceMetadata {
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
fn stress_001_10k_genesis_resources() {
    let mut reg = ResourceRegistry::new(100_000);
    let count = 10_000;
    let start = Instant::now();
    for i in 0..count {
        let id = make_id(i as u64);
        reg.register_genesis(make_genesis(id, ResourceArchetype::Asset))
            .unwrap();
    }
    let elapsed = start.elapsed();
    assert_eq!(reg.total(), count as usize);
    println!(
        "10k genesis: {:?} ({:.0} ops/sec)",
        elapsed,
        count as f64 / elapsed.as_secs_f64()
    );
}

#[test]
fn stress_002_deep_lineage_chain() {
    let mut reg = ResourceRegistry::new(10_000);
    let root_id = make_id(0);
    reg.register_genesis(make_genesis(root_id, ResourceArchetype::Asset))
        .unwrap();

    let depth = 2000;
    let start = Instant::now();
    let mut parent_id = root_id;
    for i in 1..=depth {
        let child_id = make_id(i as u64);
        let (parent_hash, version) = {
            let parent = reg.get(&parent_id).unwrap();
            (
                ResourceRegistry::hash_resource(parent),
                parent.lineage.version + 1,
            )
        };
        let child = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.consume_and_derive(&parent_id, child).unwrap();
        parent_id = child_id;
    }
    let elapsed = start.elapsed();
    assert_eq!(reg.lineage_depth(&parent_id), depth as usize);
    println!(
        "{} deep lineage: {:?} ({:.0} derivations/sec)",
        depth,
        elapsed,
        depth as f64 / elapsed.as_secs_f64()
    );
}

#[test]
fn stress_003_wide_fanout() {
    // Creates 1000 independent genesis resources, then derives one child
    // from each. This tests the registry's ability to handle many
    // independent shallow lineages — the most common real-world pattern
    // (many accounts, each with a short transaction history).
    let mut reg = ResourceRegistry::new(100_000);
    let count = 1000;
    let start = Instant::now();

    // Phase 1: genesis
    for i in 0..count {
        let id = make_id(i as u64);
        reg.register_genesis(make_genesis(id, ResourceArchetype::Asset))
            .unwrap();
    }

    // Phase 2: one derivation from each root
    for i in 0..count {
        let parent_id = make_id(i as u64);
        let child_id = make_id(10000 + i as u64);
        let (parent_hash, version) = {
            let parent = reg.get(&parent_id).unwrap();
            (
                ResourceRegistry::hash_resource(parent),
                parent.lineage.version + 1,
            )
        };
        let child = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.consume_and_derive(&parent_id, child).unwrap();
    }

    let elapsed = start.elapsed();
    assert_eq!(reg.total(), count * 2);
    assert_eq!(reg.total_active(), count); // all roots consumed, only children active
    println!(
        "Wide fanout ({} genesis + {} derivations): {:?}",
        count, count, elapsed
    );
}

#[test]
fn stress_004_state_root_10k() {
    let mut reg = ResourceRegistry::new(200_000);
    let count = 10_000;
    for i in 0..count {
        let id = make_id(i as u64);
        reg.register_genesis(make_genesis(id, ResourceArchetype::Asset))
            .unwrap();
    }
    let start = Instant::now();
    let root = reg.compute_state_root();
    let elapsed = start.elapsed();
    assert_ne!(root, [0u8; 32]);
    println!("State root 10k active: {elapsed:?}");
}

#[test]
fn stress_005_lookup_under_load() {
    let mut reg = ResourceRegistry::new(200_000);
    let count = 50_000;
    for i in 0..count {
        let id = make_id(i as u64);
        reg.register_genesis(make_genesis(id, ResourceArchetype::Asset))
            .unwrap();
    }
    let start = Instant::now();
    let mut found = 0;
    for i in 0..count {
        if reg.get(&make_id(i as u64)).is_some() {
            found += 1;
        }
    }
    let elapsed = start.elapsed();
    assert_eq!(found, count as usize);
    println!(
        "Lookup 50k resources: {:?} ({:.0} lookups/sec)",
        elapsed,
        count as f64 / elapsed.as_secs_f64()
    );
}

#[test]
fn stress_006_parent_verification_under_load() {
    let mut reg = ResourceRegistry::new(200_000);
    let root_id = make_id(0);
    reg.register_genesis(make_genesis(root_id, ResourceArchetype::Asset))
        .unwrap();

    let chain_length = 1000;
    let mut parent_id = root_id;
    for i in 1..=chain_length {
        let child_id = make_id(i as u64);
        let (parent_hash, version) = {
            let parent = reg.get(&parent_id).unwrap();
            (
                ResourceRegistry::hash_resource(parent),
                parent.lineage.version + 1,
            )
        };
        let child = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.consume_and_derive(&parent_id, child).unwrap();
        parent_id = child_id;
    }

    let start = Instant::now();
    let mut current = parent_id;
    let mut verified = 0;
    while let Some(meta) = reg.get(&current) {
        verified += 1;
        if meta.lineage.parent_resource_ids.is_empty() {
            break;
        }
        let parent = reg.get(&meta.lineage.parent_resource_ids[0]).unwrap();
        let parent_hash = ResourceRegistry::hash_resource(parent);
        assert_eq!(meta.lineage.parent_hashes[0], parent_hash);
        current = meta.lineage.parent_resource_ids[0];
    }
    let elapsed = start.elapsed();
    assert_eq!(verified, chain_length + 1);
    println!("Parent verification {chain_length} deep: {elapsed:?}");
}

#[test]
fn stress_007_cycle_detection_at_depth() {
    let mut reg = ResourceRegistry::new(200_000);
    let depth = 5000;
    let root_id = make_id(0);
    reg.register_genesis(make_genesis(root_id, ResourceArchetype::Asset))
        .unwrap();

    let mut parent_id = root_id;
    for i in 1..=depth {
        let child_id = make_id(i as u64);
        let (parent_hash, version) = {
            let parent = reg.get(&parent_id).unwrap();
            (
                ResourceRegistry::hash_resource(parent),
                parent.lineage.version + 1,
            )
        };
        let child = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.consume_and_derive(&parent_id, child).unwrap();
        parent_id = child_id;
    }

    let start = Instant::now();
    let (tip_hash, version) = {
        let tip = reg.get(&parent_id).unwrap();
        (
            ResourceRegistry::hash_resource(tip),
            tip.lineage.version + 1,
        )
    };
    let new_child = ResourceMetadata {
        resource_id: make_id(99999),
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(make_id(99999), parent_id, tip_hash, version),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    };
    let result = reg.consume_and_derive(&parent_id, new_child);
    let elapsed = start.elapsed();
    assert!(result.is_ok());
    println!("Cycle check at depth {depth}: {elapsed:?}");
}
