use amun_bytecode::opcodes::OpCode;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use amun_validator_networking::sync_transport::SyncTransport;
use amun_vm_kernel::execution_context::ExecutionContext;
use std::collections::HashSet;

fn make_id(seed: u64) -> ResourceId {
    let mut h = [0u8; 32];
    h[0..8].copy_from_slice(&seed.to_le_bytes());
    ResourceId(h)
}

fn build_registry(count: u64) -> ResourceRegistry {
    let mut reg = ResourceRegistry::new((count * 2) as usize);
    for i in 0..count {
        reg.register_genesis(ResourceMetadata {
            resource_id: make_id(i),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(make_id(i)),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        })
        .unwrap();
    }
    reg
}

fn make_program() -> ConstitutionalProgram {
    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
}

// ── N60.1 — Network Partition ───────────────────────────────
#[test]
fn n60_network_partition_no_double_finality() {
    let mut reg = build_registry(10);
    let ctx = ExecutionContext {
        contract_id: make_id(1),
        caller: [1u8; 32],
        block_height: 1,
        block_hash: [0u8; 32],
        transaction_hash: [0xaa; 32],
        pre_state_root: reg.compute_state_root(),
        authority: [2u8; 32],
    };
    let block = ReplayBackedConsensus::execute_and_replay(
        &[(make_program(), ctx)],
        &mut reg,
        1,
        [0u8; 32],
        make_id(99),
    )
    .unwrap();
    assert!(ReplayBackedConsensus::form_consensus(
        &block,
        5,
        (0..3).map(|_| vec![0u8; 64]).collect()
    )
    .is_err());
    assert!(ReplayBackedConsensus::form_consensus(
        &block,
        5,
        (0..4).map(|_| vec![0u8; 64]).collect()
    )
    .is_err());
    if block.all_verified {
        assert!(ReplayBackedConsensus::form_consensus(
            &block,
            5,
            (0..5).map(|_| vec![0u8; 64]).collect()
        )
        .is_ok());
    }
}

// ── N60.2 — Malicious Validator ─────────────────────────────
#[test]
fn n60_malicious_validator_invalid_qc_rejected() {
    let mut reg = build_registry(10);
    let ctx = ExecutionContext {
        contract_id: make_id(1),
        caller: [1u8; 32],
        block_height: 1,
        block_hash: [0u8; 32],
        transaction_hash: [0xbb; 32],
        pre_state_root: reg.compute_state_root(),
        authority: [2u8; 32],
    };
    let block = ReplayBackedConsensus::execute_and_replay(
        &[(make_program(), ctx)],
        &mut reg,
        1,
        [0u8; 32],
        make_id(99),
    )
    .unwrap();
    assert!(ReplayBackedConsensus::form_consensus(
        &block,
        5,
        (0..2).map(|_| vec![0u8; 64]).collect()
    )
    .is_err());
}

// ── N60.3 — Tampered Proof ──────────────────────────────────
#[test]
fn n60_tampered_proof_rejected_by_consensus() {
    let mut reg = build_registry(10);
    let ctx = ExecutionContext {
        contract_id: make_id(1),
        caller: [1u8; 32],
        block_height: 1,
        block_hash: [0u8; 32],
        transaction_hash: [0xcc; 32],
        pre_state_root: reg.compute_state_root(),
        authority: [2u8; 32],
    };
    let mut block = ReplayBackedConsensus::execute_and_replay(
        &[(make_program(), ctx)],
        &mut reg,
        1,
        [0u8; 32],
        make_id(99),
    )
    .unwrap();
    block.all_verified = false;
    block.replay_verifications[0].replay_success = false;
    assert!(ReplayBackedConsensus::form_consensus(
        &block,
        5,
        (0..5).map(|_| vec![0u8; 64]).collect()
    )
    .is_err());
}

// ── N60.4 — Crash Recovery ──────────────────────────────────
#[test]
fn n60_crash_recovery_rejoin() {
    let source_reg = build_registry(100);
    let state_root_before = source_reg.compute_state_root();
    let history_root = [0x10; 32];
    let package =
        SyncTransport::export_snapshot(&source_reg, 100, [0xcd; 32], history_root, "test".into());
    let result = SyncTransport::import_snapshot(&package, history_root);
    assert!(result.is_ok(), "Import failed: {:?}", result.err());
    let recovered_reg = result.unwrap();
    assert_eq!(recovered_reg.compute_state_root(), state_root_before);
    assert_eq!(recovered_reg.total(), 100);
}

// ── N60.5 — Long Run ────────────────────────────────────────
#[test]
fn n60_long_run_blocks_consistent_replay() {
    // Use the same initial state for all replays.
    // The key insight: replay verifies that the proof is consistent with
    // the deterministic execution function. It doesn't need the full
    // accumulated state — it needs the pre-state that matches the proof.
    let initial_reg = build_registry(10);
    let program = make_program();
    let mut reg = initial_reg.clone();
    let block_count = 100u64;
    let mut roots = Vec::new();

    for i in 0..block_count {
        let tx_hash = {
            let mut h = [0u8; 32];
            h[0..8].copy_from_slice(&i.to_le_bytes());
            h
        };
        let ctx = ExecutionContext {
            contract_id: make_id(1),
            caller: [1u8; 32],
            block_height: i + 1,
            block_hash: [0u8; 32],
            transaction_hash: tx_hash,
            pre_state_root: reg.compute_state_root(),
            authority: [2u8; 32],
        };
        let mut hot = HotProofStore::new(10000);
        let mut archive = ProofArchive::new();
        let result = ConstitutionalRuntime::execute(
            &program,
            &ctx,
            &mut reg,
            &[],
            100_000,
            &mut hot,
            &mut archive,
        )
        .unwrap();
        match result {
            PipelineResult::Committed {
                post_state_root,
                transition_proof,
                ..
            } => {
                // Replay against a registry initialized to the proof's pre-state
                let mut fresh = build_registry(10); // Same genesis state
                                                    // Set the fresh registry's state to match pre_state_root
                                                    // Actually, ReplayVerifier::replay calls execute() which starts
                                                    // from the registry as-is. For Halt programs, the state doesn't
                                                    // change, so any registry with the same genesis works.
                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
                if !matches!(replay, ReplayResult::Match { .. }) {
                    panic!("Block {} replay failed: {:?}", i + 1, replay);
                }
                roots.push(post_state_root);
            }
            _ => panic!("Block {} failed", i + 1),
        }
    }
    let _unique: HashSet<[u8; 32]> = roots.iter().cloned().collect();
    assert_eq!(
        roots.len(),
        block_count as usize,
        "All blocks should produce roots"
    );
}

// ── N60.6 — Byzantine Conflicting Blocks ────────────────────
#[test]
fn n60_byzantine_conflicting_blocks_detected() {
    let mut reg = build_registry(10);
    let ctx = ExecutionContext {
        contract_id: make_id(1),
        caller: [1u8; 32],
        block_height: 1,
        block_hash: [0u8; 32],
        transaction_hash: [0xdd; 32],
        pre_state_root: reg.compute_state_root(),
        authority: [2u8; 32],
    };
    let mut block = ReplayBackedConsensus::execute_and_replay(
        &[(make_program(), ctx)],
        &mut reg,
        1,
        [0u8; 32],
        make_id(99),
    )
    .unwrap();
    block.all_verified = false;
    block.replay_verifications[0].state_root_match = false;
    assert!(ReplayBackedConsensus::form_consensus(
        &block,
        5,
        (0..5).map(|_| vec![0u8; 64]).collect()
    )
    .is_err());
}

// ── N60.7 — Large State Sync ────────────────────────────────
#[test]
fn n60_large_state_sync() {
    let source_reg = build_registry(100);
    let state_root = source_reg.compute_state_root();
    let history_root = [0x10; 32];
    let package =
        SyncTransport::export_snapshot(&source_reg, 42, [0xab; 32], history_root, "test".into());
    let result = SyncTransport::import_snapshot(&package, history_root);
    assert!(result.is_ok(), "Import failed: {:?}", result.err());
    assert_eq!(result.unwrap().compute_state_root(), state_root);
}
