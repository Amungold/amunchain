use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_state_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let mut roots = Vec::new();

    for i in 0..4 {
        let dir = format!("{}/validator{}", prefix, i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let mut store = PersistentValidatorStore::open(&dir).expect("Failed to open store");

        let root_initial = store.state_root();
        println!(
            "Validator {} initial root: {}",
            i,
            hex::encode(root_initial)
        );

        // Use identical resource IDs for all validators to test determinism
        for j in 0..3 {
            let id = ResourceId([j as u8; 32]); // same IDs across all validators
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
                .expect("Failed to register");
        }

        let root_final = store.state_root();
        println!("Validator {} final root:   {}", i, hex::encode(root_final));

        roots.push((root_initial, root_final));
    }

    let first_initial = roots[0].0;
    let all_initial_match = roots.iter().all(|(init, _)| *init == first_initial);
    let first_final = roots[0].1;
    let all_final_match = roots.iter().all(|(_, fin)| *fin == first_final);
    let state_changed = first_initial != first_final;

    if all_initial_match && all_final_match && state_changed {
        println!("\nPASS: State evolution is deterministic");
        println!("Initial root: {}", hex::encode(first_initial));
        println!("Final root:   {}", hex::encode(first_final));
    } else {
        println!("\nFAIL: Determinism violation detected");
        if !all_initial_match {
            println!("  - Initial roots differ");
        }
        if !all_final_match {
            println!("  - Final roots differ");
        }
        if !state_changed {
            println!("  - State did not evolve");
        }
        std::process::exit(1);
    }
}
