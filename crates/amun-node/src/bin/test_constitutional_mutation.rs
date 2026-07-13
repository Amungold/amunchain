use amun_bytecode::opcodes::OpCode;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::ResourceId;
use amun_vm_kernel::execution_context::ExecutionContext;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_constitutional_mutation <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let mut final_roots = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let mut store = PersistentValidatorStore::open(&dir).expect("Failed to open store");

        let root0 = store.state_root();
        println!("Validator {} | Height 0 root: {}", v, hex::encode(root0));

        for h in 1..=3u64 {
            let program = ConstitutionalProgram::new(
                2,
                0,
                0,
                vec![
                    OpCode::Push(0),
                    OpCode::Split {
                        handle: 0,
                        amount_count: 5,
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
                Ok(_r) => {
                    store
                        .advance(h, [0u8; 32], [0x10; 32], vec![])
                        .expect("Failed to advance");
                    let current_root = store.state_root();
                    println!(
                        "Validator {} | Height {} root: {}",
                        v,
                        h,
                        hex::encode(current_root)
                    );
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
        let final_root = store.state_root();
        final_roots.push(final_root);
    }

    let first = final_roots[0];
    let all_match = final_roots.iter().all(|r| *r == first);
    if all_match {
        println!("\nPASS: Constitutional state mutation is deterministic");
        println!("Final root: {}", hex::encode(first));
    } else {
        println!("\nFAIL: Validators diverged");
        for (i, r) in final_roots.iter().enumerate() {
            println!("Validator {} final root: {}", i, hex::encode(*r));
        }
        std::process::exit(1);
    }
}
