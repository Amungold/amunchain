use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
    ResourceRegistry, ResourceState,
};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_bytecode::opcodes::OpCode;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_replay_verifier::replay_verifier::ReplayVerifier;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use std::time::Instant;

fn make_id(seed: u64) -> ResourceId {
    let mut h = [0u8; 32];
    h[0..8].copy_from_slice(&seed.to_le_bytes());
    ResourceId(h)
}

struct Stats { mean: f64, ci95: f64, n: u32 }

fn compute_stats(times: &[f64]) -> Stats {
    let n = times.len() as u32;
    let mean = times.iter().sum::<f64>() / n as f64;
    let variance = times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / n as f64;
    let stddev = variance.sqrt();
    let ci95 = 1.96 * stddev / (n as f64).sqrt();
    Stats { mean, ci95, n }
}

fn measure_us(name: &str, warmup: u32, iterations: u32, mut f: impl FnMut()) -> Stats {
    for _ in 0..warmup { f(); }
    let mut times = Vec::with_capacity(iterations as usize);
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed().as_secs_f64() * 1_000_000.0;
        times.push(elapsed);
    }
    let s = compute_stats(&times);
    println!("{}: {:.2} ± {:.2} µs  (95% CI, n={})", name, s.mean, s.ci95, s.n);
    s
}

fn write_csv(filename: &str, header: &[&str], rows: &[Vec<String>]) {
    let path = format!("results/{}", filename);
    let mut wtr = csv::Writer::from_path(&path).unwrap();
    wtr.write_record(header).unwrap();
    for row in rows { wtr.write_record(row).unwrap(); }
    wtr.flush().unwrap();
    println!("  -> written {}", path);
}

fn build_registry(size: u64) -> ResourceRegistry {
    let mut reg = ResourceRegistry::new((size * 2) as usize);
    for i in 0..size {
        reg.register_genesis(ResourceMetadata {
            resource_id: make_id(i),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(make_id(i)),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }).unwrap();
    }
    reg
}

fn build_deep_chain(depth: u64) -> (ResourceRegistry, ResourceId) {
    let mut reg = ResourceRegistry::new((depth * 2) as usize);
    let root = make_id(0);
    reg.register_genesis(ResourceMetadata {
        resource_id: root, archetype: ResourceArchetype::Asset,
        state: ResourceState::Active, lineage: ResourceLineage::genesis(root),
        contract_id: [1u8; 32], owner: [2u8; 32],
    }).unwrap();
    let mut parent = root;
    for i in 1..=depth {
        let child = make_id(i);
        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
        let version = reg.get(&parent).unwrap().lineage.version + 1;
        reg.consume_and_derive(&parent, ResourceMetadata {
            resource_id: child, archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
            contract_id: [1u8; 32], owner: [2u8; 32],
        }).unwrap();
        parent = child;
    }
    (reg, parent)
}

/// Workload A: Halt — baseline overhead.
fn workload_halt() -> ConstitutionalProgram {
    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
}

/// Workload B: Push10 — compute overhead.
fn workload_push10() -> ConstitutionalProgram {
    ConstitutionalProgram::new(1, 0, 0, vec![
        OpCode::Push(1), OpCode::Push(2), OpCode::Push(3), OpCode::Push(4),
        OpCode::Push(5), OpCode::Push(6), OpCode::Push(7), OpCode::Push(8),
        OpCode::Push(9), OpCode::Push(10), OpCode::Halt,
    ])
}

/// Workload C: Transform — single resource transformation.
fn workload_transform() -> ConstitutionalProgram {
    ConstitutionalProgram::new(2, 0, 0, vec![
        OpCode::Transform { src_handle: 0, type_idx: 0 },
        OpCode::Halt,
    ])
}

/// Workload D: Split — split one asset into 5 children.
fn workload_split() -> ConstitutionalProgram {
    ConstitutionalProgram::new(2, 0, 0, vec![
        OpCode::Push(0),
        OpCode::Split { handle: 0, amount_count: 5 },
        OpCode::Halt,
    ])
}

// ── Experiment 1: State-Scale Invariance ────────────────────
fn exp1_state_scale() {
    println!("\n=== Experiment 1: State-Scale Invariance (Halt) ===");
    let sizes = [1_000u64, 10_000, 100_000, 1_000_000];
    let program = workload_halt();
    let mut rows: Vec<Vec<String>> = vec![];

    for &size in &sizes {
        let mut reg = build_registry(size);
        let ctx = ExecutionContext {
            contract_id: make_id(999), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: reg.compute_state_root(), authority: [2u8; 32],
        };
        let mut hot = HotProofStore::new(1000);
        let mut archive = ProofArchive::new();
        let result = ConstitutionalRuntime::execute(
            &program, &ctx, &mut reg, &[], 100_000, &mut hot, &mut archive,
        ).unwrap();
        let proof = match result {
            PipelineResult::Committed { transition_proof, .. } => transition_proof,
            _ => panic!("Expected Committed"),
        };

        let stats = measure_us(&format!("replay_{}_active", size), 5, 30, || {
            let mut fresh = ResourceRegistry::new((size * 2) as usize);
            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
        });
        rows.push(vec![size.to_string(), format!("{:.4}", stats.mean), format!("{:.4}", stats.ci95)]);
    }
    write_csv("state_scale.csv", &["active_resources", "replay_time_us", "ci95_us"], &rows);
}

// ── Experiment 2: Replay vs Execution (all workloads) ───────
fn exp2_replay_vs_execute() {
    println!("\n=== Experiment 2: Replay vs Execution ===");
    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
        ("halt", workload_halt()),
        ("push10", workload_push10()),
        ("transform", workload_transform()),
        ("split", workload_split()),
    ];
    let size = 10_000u64;
    let mut rows: Vec<Vec<String>> = vec![];

    for (name, program) in &workloads {
        let mut reg = build_registry(size);
        let ctx = ExecutionContext {
            contract_id: make_id(999), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: reg.compute_state_root(), authority: [2u8; 32],
        };

        // Measure execution
        let mut hot = HotProofStore::new(1000);
        let mut archive = ProofArchive::new();
        let exec_stats = measure_us(&format!("execute_{}", name), 5, 30, || {
            let mut r = reg.clone();
            let mut h = hot.clone();
            let mut a = archive.clone();
            match ConstitutionalRuntime::execute(program, &ctx, &mut r, &[], 100_000, &mut h, &mut a) { Ok(_) => {}, Err(e) => { println!("  {} execution failed: {}", name, e); } }
        });

        // Get proof
        let result = ConstitutionalRuntime::execute(
            program, &ctx, &mut reg, &[], 100_000, &mut hot, &mut archive,
        );
        let proof = match result {
            Ok(PipelineResult::Committed { transition_proof, .. }) => transition_proof,
            Ok(PipelineResult::Rejected { transition_proof, .. }) => {
                println!("  {} was rejected, using proof from rejection", name);
                transition_proof
            }
            Err(e) => {
                println!("  {} execution failed: {}", name, e);
                rows.push(vec![name.to_string(), "ERROR".into(), "ERROR".into(), "ERROR".into()]);
                continue;
            }
        };

        // Measure replay
        let replay_stats = measure_us(&format!("replay_{}", name), 5, 30, || {
            let mut fresh = ResourceRegistry::new((size * 2) as usize);
            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
        });

        let speedup = exec_stats.mean / replay_stats.mean;
        println!("  speedup({}) = {:.2}x", name, speedup);
        rows.push(vec![
            name.to_string(),
            format!("{:.4}", exec_stats.mean),
            format!("{:.4}", replay_stats.mean),
            format!("{:.4}", speedup),
        ]);
    }
    write_csv("speedup.csv", &["workload", "execution_us", "replay_us", "speedup"], &rows);
}

// ── Experiment 3: Full Pipeline ─────────────────────────────
fn exp3_full_pipeline() {
    println!("\n=== Experiment 3: Full Pipeline (Halt) ===");
    let tx_counts = [1u64, 10, 100, 1000];
    let program = workload_halt();
    let mut rows: Vec<Vec<String>> = vec![];

    for &n in &tx_counts {
        let stats = measure_us(&format!("pipeline_{}_tx", n), 5, 20, || {
            let mut reg = ResourceRegistry::new((n * 10) as usize);
            let mut hot = HotProofStore::new(10000);
            let mut archive = ProofArchive::new();
            for i in 0..n {
                let ctx = ExecutionContext {
                    contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
                    block_hash: [0u8; 32],
                    transaction_hash: { let mut h = [0u8; 32]; h[0..8].copy_from_slice(&i.to_le_bytes()); h },
                    pre_state_root: reg.compute_state_root(),
                    authority: [2u8; 32],
                };
                let result = ConstitutionalRuntime::execute(
                    &program, &ctx, &mut reg, &[], 100_000, &mut hot, &mut archive,
                ).unwrap();
                if let PipelineResult::Committed { transition_proof, .. } = result {
                    let mut fresh = ResourceRegistry::new(10000);
                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
                }
            }
        });
        rows.push(vec![n.to_string(), format!("{:.4}", stats.mean), format!("{:.4}", stats.ci95)]);
    }
    write_csv("pipeline_latency.csv", &["tx_count", "latency_us", "ci95_us"], &rows);
}

// ── Experiment 4: Cycle Detection ───────────────────────────
fn exp4_cycle_detection() {
    println!("\n=== Experiment 4: Cycle Detection ===");
    let depths = [100u64, 500, 1000, 2000, 5000];
    let mut rows: Vec<Vec<String>> = vec![];

    for &depth in &depths {
        let stats = measure_us(&format!("cycle_detect_depth_{}", depth), 5, 20, || {
            let (mut reg, tip) = build_deep_chain(depth);
            let tip_meta = reg.get(&tip).unwrap();
            let tip_hash = ResourceRegistry::hash_resource(tip_meta);
            let version = tip_meta.lineage.version + 1;
            let new_id = make_id(depth + 100);
            let _ = reg.consume_and_derive(&tip, ResourceMetadata {
                resource_id: new_id, archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(new_id, tip, tip_hash, version),
                contract_id: [1u8; 32], owner: [2u8; 32],
            });
        });
        rows.push(vec![depth.to_string(), format!("{:.4}", stats.mean), format!("{:.4}", stats.ci95)]);
    }
    write_csv("cycle_detection.csv", &["depth", "time_us", "ci95_us"], &rows);
}

// ── Experiment 5: Witness Bundle Size ───────────────────────
fn exp5_witness_size() {
    println!("\n=== Experiment 5: Witness Bundle Size ===");
    let sizes = [1u64, 10, 100, 1000];
    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
        ("halt", workload_halt()),
        ("transform", workload_transform()),
    ];
    let mut rows: Vec<Vec<String>> = vec![];

    for (name, program) in &workloads {
        for &size in &sizes {
            let mut reg = build_registry(size);
            let ctx = ExecutionContext {
                contract_id: make_id(999), caller: [1u8; 32], block_height: 1,
                block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
                pre_state_root: reg.compute_state_root(), authority: [2u8; 32],
            };
            let mut hot = HotProofStore::new(1000);
            let mut archive = ProofArchive::new();
            let result = ConstitutionalRuntime::execute(
                program, &ctx, &mut reg, &[], 100_000, &mut hot, &mut archive,
            ).unwrap();
            let proof = match result {
                PipelineResult::Committed { transition_proof, .. } => transition_proof,
                _ => continue,
            };
            let json_size = serde_json::to_string(&proof).unwrap().len();
            println!("  witness_{}_{}: {} bytes", name, size, json_size);
            rows.push(vec![name.to_string(), size.to_string(), json_size.to_string()]);
        }
    }
    write_csv("witness_size.csv", &["workload", "resources", "witness_bytes"], &rows);
}

// ── Experiment 6: Resource Law Verification ─────────────────
fn exp6_law_verification() {
    println!("\n=== Experiment 6: Resource Law Verification ===");
    let sizes = [1u64, 10, 100, 1000];
    let mut rows: Vec<Vec<String>> = vec![];

    for &size in &sizes {
        let stats = measure_us(&format!("verify_{}_resources", size), 5, 30, || {
            let reg = build_registry(size);
            let _root = reg.compute_state_root();
        });
        rows.push(vec![size.to_string(), format!("{:.4}", stats.mean), format!("{:.4}", stats.ci95)]);
    }
    write_csv("law_verification.csv", &["resources", "time_us", "ci95_us"], &rows);
}

// ── main ─────────────────────────────────────────────────────
fn main() {
    println!("=== AmunChain Experimental Framework — Section 7 ===\n");

    exp1_state_scale();
    exp2_replay_vs_execute();
    exp3_full_pipeline();
    exp4_cycle_detection();
    exp5_witness_size();
    exp6_law_verification();

    println!("\n=== All experiments complete. Results in results/ ===");
}
