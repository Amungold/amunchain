use amun_bytecode::opcodes::OpCode;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use amun_vm_kernel::execution_context::ExecutionContext;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_constitutional_stress <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let block_count: u64 = 100;
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

        for h in 1..=block_count {
            let resource_id = ResourceId([h as u8; 32]);
            let meta = ResourceMetadata {
                resource_id,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(resource_id),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            };
            store
                .registry_mut()
                .register_genesis(meta)
                .expect("Failed to register");

            let program = ConstitutionalProgram::new(
                2,
                0,
                0,
                vec![
                    OpCode::Push(0),
                    OpCode::Split {
                        handle: 0,
                        amount_count: 3,
                    },
                    OpCode::Halt,
                ],
            );

            let ctx = ExecutionContext {
                contract_id: ResourceId([v as u8; 32]),
                caller: [1u8; 32],
                block_height: h,
                block_hash: [h as u8; 32],
                transaction_hash: [h as u8; 32],
                pre_state_root: store.state_root(),
                authority: [2u8; 32],
            };

            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();

            let result = ConstitutionalRuntime::execute(
                &program,
                &ctx,
                store.registry_mut(),
                &[],
                100_000,
                &mut hot,
                &mut archive,
            );

            match result {
                Ok(_) => {
                    store
                        .advance(h, [0u8; 32], [0x10; 32], vec![])
                        .expect("Failed to advance");
                    let current_root = store.state_root();

                    // Print only every 10th block to reduce output
                    if h % 10 == 0 || h == block_count {
                        println!(
                            "Validator {} | Height {} root: {}",
                            v,
                            h,
                            hex::encode(current_root)
                        );
                    }

                    if current_root == prev_root {
                        eprintln!(
                            "FAIL: Validator {} state root did not change at height {}",
                            v, h
                        );
                        std::process::exit(1);
                    }
                    prev_root = current_root;
                }
                Err(e) => {
                    eprintln!(
                        "FAIL: Validator {} execution error at height {}: {}",
                        v, h, e
                    );
                    std::process::exit(1);
                }
            }
        }
        final_roots.push(prev_root);
    }

    let first = final_roots[0];
    let all_match = final_roots.iter().all(|r| *r == first);
    if all_match {
        println!(
            "\nPASS: Constitutional stress test passed ({} blocks)",
            block_count
        );
        println!("Final root: {}", hex::encode(first));
    } else {
        println!("\nFAIL: Validators diverged during stress test");
        for (i, r) in final_roots.iter().enumerate() {
            println!("Validator {} final root: {}", i, hex::encode(*r));
        }
        std::process::exit(1);
    }
}
