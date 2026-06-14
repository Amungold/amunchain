use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use amun_validator_runtime::validator_node::ValidatorNode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_runtime_mutation_integration <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    const BLOCKS: u64 = 3;
    let mut final_roots = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let node_id = ResourceId([v as u8; 32]);
        let mut node = ValidatorNode::new(node_id, &dir).expect("Failed to create node");

        let mut prev_root = node.store.state_root();
        println!("Validator {} | Height 0 root: {}", v, hex::encode(prev_root));

        for h in 1..=BLOCKS {
            // Add resources directly to the registry before proposing
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
                node.store.registry_mut().register_genesis(meta).expect("Failed to register");
            }
            node.propose_block(h).expect("Failed to propose block");
            let current_root = node.store.state_root();
            println!("Validator {} | Height {} root: {}", v, h, hex::encode(current_root));

            if current_root == prev_root {
                println!("FAIL: State root did not evolve at height {}", h);
                std::process::exit(1);
            }
            prev_root = current_root;
        }
        final_roots.push(prev_root);
    }

    // Verify all validators reach the same final state
    let first = final_roots[0];
    let all_match = final_roots.iter().all(|r| *r == first);
    if all_match {
        println!("\nPASS: Runtime mutation integration verified");
        println!("Final root after {} blocks: {}", BLOCKS, hex::encode(first));
    } else {
        println!("\nFAIL: Validators diverged");
        for (i, r) in final_roots.iter().enumerate() {
            println!("Validator {} final root: {}", i, hex::encode(*r));
        }
        std::process::exit(1);
    }
}
