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
        eprintln!("Usage: test_replay_stress <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let block_count: u64 = 100;
    let mut mutation_log: Vec<(ResourceId, ConstitutionalProgram)> = Vec::new();

    // Phase 1: Build reference state and record every mutation
    let ref_dir = format!("{}/reference", prefix);
    std::fs::create_dir_all(&ref_dir).expect("Failed to create dir");
    let mut ref_store = PersistentValidatorStore::open(&ref_dir).expect("Failed to open store");

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
        ref_store
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
            contract_id: ResourceId([0u8; 32]),
            caller: [1u8; 32],
            block_height: h,
            block_hash: [h as u8; 32],
            transaction_hash: [h as u8; 32],
            pre_state_root: ref_store.state_root(),
            authority: [2u8; 32],
        };
        let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
        let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
        ConstitutionalRuntime::execute(
            &program,
            &ctx,
            ref_store.registry_mut(),
            &[],
            100_000,
            &mut hot,
            &mut archive,
        )
        .expect("Execution failed");
        ref_store
            .advance(h, [0u8; 32], [0x10; 32], vec![])
            .expect("Advance failed");

        mutation_log.push((resource_id, program));
    }
    let ref_root = ref_store.state_root();
    println!(
        "Reference root after {} blocks: {}",
        block_count,
        hex::encode(ref_root)
    );
    drop(ref_store);

    // Phase 2: Replay on 4 independent stores from scratch
    let mut replay_roots = Vec::new();
    for v in 0..4 {
        let replay_dir = format!("{}/replay{}", prefix, v);
        std::fs::create_dir_all(&replay_dir).expect("Failed to create dir");
        let mut replay_store =
            PersistentValidatorStore::open(&replay_dir).expect("Failed to open store");

        for (h, (resource_id, program)) in mutation_log.iter().enumerate() {
            let height = (h + 1) as u64;
            let meta = ResourceMetadata {
                resource_id: *resource_id,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(*resource_id),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            };
            replay_store
                .registry_mut()
                .register_genesis(meta)
                .expect("Failed to register");

            let ctx = ExecutionContext {
                contract_id: ResourceId([v as u8; 32]),
                caller: [1u8; 32],
                block_height: height,
                block_hash: [height as u8; 32],
                transaction_hash: [height as u8; 32],
                pre_state_root: replay_store.state_root(),
                authority: [2u8; 32],
            };
            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
            ConstitutionalRuntime::execute(
                program,
                &ctx,
                replay_store.registry_mut(),
                &[],
                100_000,
                &mut hot,
                &mut archive,
            )
            .expect("Replay execution failed");
            replay_store
                .advance(height, [0u8; 32], [0x10; 32], vec![])
                .expect("Replay advance failed");
        }
        let replay_root = replay_store.state_root();
        println!("Replay {} final root: {}", v, hex::encode(replay_root));
        replay_roots.push(replay_root);
    }

    // Phase 3: Verify all match reference
    let all_match = replay_roots.iter().all(|r| *r == ref_root);
    if all_match {
        println!("\nPASS: Replay stress test passed ({} blocks)", block_count);
        println!("Final root: {}", hex::encode(ref_root));
    } else {
        println!("\nFAIL: Replay divergence detected");
        for (i, r) in replay_roots.iter().enumerate() {
            println!(
                "Replay {} root: {} (match: {})",
                i,
                hex::encode(*r),
                *r == ref_root
            );
        }
        std::process::exit(1);
    }
}
