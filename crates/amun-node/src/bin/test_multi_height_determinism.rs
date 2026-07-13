use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_multi_height_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    const HEIGHTS: u64 = 5;
    let mut final_roots = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let mut store = PersistentValidatorStore::open(&dir).expect("Failed to open store");

        let mut prev_root = store.state_root();
        println!(
            "Validator {} | Height 0 root: {}",
            v,
            hex::encode(prev_root)
        );

        for h in 1..=HEIGHTS {
            // Add h resources at this height, all deterministic
            for j in 0..h {
                let id = ResourceId([
                    h as u8, j as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]);
                let meta = ResourceMetadata {
                    resource_id: id,
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(id),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                };
                store
                    .registry_mut()
                    .register_genesis(meta)
                    .expect("Failed to register resource");
            }
            let current_root = store.state_root();
            println!(
                "Validator {} | Height {} root: {}",
                v,
                h,
                hex::encode(current_root)
            );
            // Verify state changed at each height (if resources were added)
            if h > 0 && current_root == prev_root {
                println!(
                    "FAIL: Validator {} height {} root did not change from height {}",
                    v,
                    h,
                    h - 1
                );
                std::process::exit(1);
            }
            prev_root = current_root;
        }
        final_roots.push(prev_root);
    }

    // Verify all final roots identical
    let first = final_roots[0];
    let all_match = final_roots.iter().all(|r| *r == first);
    if all_match {
        println!("\nPASS: Multi-height state evolution is deterministic");
        println!(
            "Final root after {} heights: {}",
            HEIGHTS,
            hex::encode(first)
        );
    } else {
        println!("\nFAIL: Final roots diverge");
        for (i, r) in final_roots.iter().enumerate() {
            println!("Validator {} final root: {}", i, hex::encode(*r));
        }
        std::process::exit(1);
    }
}
