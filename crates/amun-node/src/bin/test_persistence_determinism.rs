use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_persistence_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    const HEIGHTS: u64 = 3;
    let mut final_roots_before = Vec::new();
    let mut final_roots_after = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");

        // Phase 1: build state
        let mut store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        for h in 1..=HEIGHTS {
            for j in 0..h {
                let id = ResourceId([h as u8, j as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                let meta = ResourceMetadata {
                    resource_id: id,
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(id),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                };
                store.registry_mut().register_genesis(meta).expect("Failed to register");
            }
            // Advance height to simulate block commitment
            store.advance(h, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
        }
        let root_before = store.state_root();
        println!("Validator {} root before save: {}", v, hex::encode(root_before));
        store.save().expect("Failed to save");
        drop(store);

        // Phase 2: restore and verify
        let mut restored = PersistentValidatorStore::open(&dir).expect("Failed to reopen store");
        restored.restore().expect("Failed to restore");
        let root_after = restored.state_root();
        println!("Validator {} root after restore: {}", v, hex::encode(root_after));

        if root_before != root_after {
            println!("FAIL: Validator {} persistence mismatch", v);
            std::process::exit(1);
        }

        final_roots_before.push(root_before);
        final_roots_after.push(root_after);
    }

    // Verify all validators have identical roots
    let first_before = final_roots_before[0];
    let all_before_match = final_roots_before.iter().all(|r| *r == first_before);
    let first_after = final_roots_after[0];
    let all_after_match = final_roots_after.iter().all(|r| *r == first_after);

    if all_before_match && all_after_match && first_before == first_after {
        println!("\nPASS: Persistence determinism verified across all validators");
        println!("Final root: {}", hex::encode(first_after));
    } else {
        println!("\nFAIL: Persistence determinism violation");
        if !all_before_match { println!("  - Roots before save differ"); }
        if !all_after_match { println!("  - Roots after restore differ"); }
        if first_before != first_after { println!("  - Root mismatch before/after"); }
        std::process::exit(1);
    }
}
