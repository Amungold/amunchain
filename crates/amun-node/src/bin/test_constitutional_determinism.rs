use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::ResourceId;
use amun_vm_kernel::execution_context::ExecutionContext;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_constitutional_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let mut final_roots = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let mut store = PersistentValidatorStore::open(&dir).expect("Failed to open store");

        for h in 1..=3u64 {
            let level = (h % 255) as u8; // safe conversion for test
            let program = ConstitutionalProgram::new(level, 0, 0, vec![]);
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
                    store.advance(h, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
                }
                Err(e) => {
                    eprintln!("FAIL: Execution error at height {}: {}", h, e);
                    std::process::exit(1);
                }
            }
        }
        let root = store.state_root();
        println!("Validator {} final root: {}", v, hex::encode(root));
        final_roots.push(root);
    }

    let first = final_roots[0];
    let all_match = final_roots.iter().all(|r| *r == first);
    if all_match {
        println!("\nPASS: Constitutional runtime execution is deterministic");
        println!("Final root: {}", hex::encode(first));
    } else {
        println!("\nFAIL: Validators diverged");
        for (i, r) in final_roots.iter().enumerate() {
            println!("Validator {} final root: {}", i, hex::encode(*r));
        }
        std::process::exit(1);
    }
}
