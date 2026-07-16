use amun_consensus_execution::persistent_state::PersistentConsensusState;
use amun_quorum_certificate::QuorumCertificate;
use amun_chain_position::ChainPosition;
use tempfile::TempDir;

/// Helper: create a minimal QC for testing.
fn make_qc(block_hash: [u8; 32], parent_hash: [u8; 32], round: u64, height: u64) -> QuorumCertificate {
    QuorumCertificate {
        position: ChainPosition::new(0, height),
        round,
        block_hash,
        parent_hash,
        votes: Vec::new(),
        aggregated_signature: None,
    }
}

/// Helper: record a batch of QCs in sequence.
fn record_qc_chain(state: &mut PersistentConsensusState, count: u64, start_round: u64, genesis: [u8; 32]) {
    for i in 0..count {
        let bh = [i as u8; 32];
        let ph = if i == 0 { genesis } else { [(i - 1) as u8; 32] };
        let qc = make_qc(bh, ph, start_round + i, i + 1);
        state.record_qc(&qc).unwrap();
        let _d = state.state_digest();    }
}

#[test]
fn r1_linear_chain_live_equals_replay() {
    let dir = TempDir::new().unwrap();
    let wal_path = dir.path().join("wal").to_str().unwrap().to_string();
    let genesis = [0x11; 32];

    // Phase 1: Live execution
    let mut live = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    record_qc_chain(&mut live, 10, 0, genesis);
    let live_digest = live.state_digest();
    live.shutdown().expect("shutdown");

    // Phase 2: Replay from same WAL
    let replay = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    let replay_digest = replay.state_digest();

    // Assert equivalence
    assert_eq!(live_digest.commit_index, replay_digest.commit_index,
        "commit_index mismatch");
    assert_eq!(live_digest.finalized_height, replay_digest.finalized_height,
        "finalized_height mismatch");
    assert_eq!(live_digest.locked_qc_block, replay_digest.locked_qc_block,
        "locked_qc_block mismatch");
    assert_eq!(live_digest.locked_qc_round, replay_digest.locked_qc_round,
        "locked_qc_round mismatch");
    assert_eq!(live_digest.canonical_tip, replay_digest.canonical_tip,
        "canonical_tip mismatch");
    assert_eq!(live_digest.dag_block_count, replay_digest.dag_block_count,
        "dag_block_count mismatch");
    assert_eq!(live_digest.spine_length, replay_digest.spine_length,
        "spine_length mismatch");
    assert_eq!(live_digest.fork_choice_high_qc_count, replay_digest.fork_choice_high_qc_count,
        "fork_choice_high_qc_count mismatch");
    assert_eq!(live_digest.applied_sequence_count, replay_digest.applied_sequence_count,
        "applied_sequence_count mismatch");
}

#[test]
fn r1_replay_is_idempotent() {
    let dir = TempDir::new().unwrap();
    let wal_path = dir.path().join("wal").to_str().unwrap().to_string();
    let genesis = [0x11; 32];

    // Live execution
    let mut live = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    record_qc_chain(&mut live, 5, 0, genesis);

    // Replay twice
    let replay1 = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    let digest1 = replay1.state_digest();

    let replay2 = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    let digest2 = replay2.state_digest();

    assert_eq!(digest1.commit_index, digest2.commit_index);
    assert_eq!(digest1.canonical_tip, digest2.canonical_tip);
    assert_eq!(digest1.fork_choice_high_qc_count, digest2.fork_choice_high_qc_count);
}

#[test]
fn r1_restart_midstream_preserves_state() {
    let dir = TempDir::new().unwrap();
    let wal_path = dir.path().join("wal").to_str().unwrap().to_string();
    let genesis = [0x11; 32];

    // Phase 1: Record first batch
    let mut state = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    record_qc_chain(&mut state, 3, 0, genesis);
    state.shutdown().expect("shutdown");
    drop(state);

    // Phase 2: Reopen and continue
    let mut state = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    record_qc_chain(&mut state, 3, 3, genesis);
    let continued_digest = state.state_digest();
    state.shutdown().expect("shutdown");
    drop(state);

    // Phase 3: Replay everything from scratch
    let replay = PersistentConsensusState::open(&wal_path, genesis).unwrap();
    let replay_digest = replay.state_digest();

    assert_eq!(continued_digest.commit_index, replay_digest.commit_index);
    assert_eq!(continued_digest.canonical_tip, replay_digest.canonical_tip);
    assert_eq!(continued_digest.dag_block_count, replay_digest.dag_block_count);
}
