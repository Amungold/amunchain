extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use amun_execution_receipt::{ExecutionReceipt, ExecutionStatus, ExecutionTranscript};
use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};

/// Execute a deterministic block and produce:
/// - execution transcript
/// - final state bytes
/// - final state root
fn execute_block(txs: &[[u8; 32]]) -> (ExecutionTranscript, Vec<u8>, [u8; 32]) {
    let mut transcript = ExecutionTranscript::new();
    let mut current_state = vec![0u8];
    let mut previous_receipt_hash = [0u8; 32];

    for (i, tx_hash) in txs.iter().enumerate() {
        let receipt = execute_transition_with_receipt(
            *tx_hash,
            &current_state,
            |state| {
                let mut next_state = state.to_vec();
                next_state.push((i + 1) as u8);
                TransitionExecutionResult {
                    post_state: next_state,
                    execution_result_hash: [(i + 1) as u8; 32],
                    status: ExecutionStatus::Success,
                }
            },
            previous_receipt_hash,
        )
        .expect("execution must succeed");

        current_state.push((i + 1) as u8);
        previous_receipt_hash = receipt.receipt_hash();
        transcript.add_receipt(receipt);
    }

    let final_root = transcript
        .receipts
        .last()
        .expect("transcript must not be empty")
        .post_state_root;

    (transcript, current_state, final_root)
}

#[test]
fn test_single_block_replay_equivalence() {
    let txs = vec![[0x11; 32], [0x22; 32], [0x33; 32], [0x44; 32], [0x55; 32]];

    let (live_transcript, live_state, live_root) = execute_block(&txs);
    let (replay_transcript, replay_state, replay_root) = execute_block(&txs);

    assert_eq!(live_root, replay_root, "live and replay roots diverged");
    assert_eq!(live_state, replay_state, "live and replay state diverged");
    assert_eq!(
        live_transcript.receipts.len(),
        replay_transcript.receipts.len(),
        "receipt counts diverged"
    );

    for (index, (live, replay)) in live_transcript
        .receipts
        .iter()
        .zip(replay_transcript.receipts.iter())
        .enumerate()
    {
        assert_eq!(
            live.receipt_hash(),
            replay.receipt_hash(),
            "receipt hash mismatch at index {}",
            index
        );
        assert_eq!(live.tx_hash, replay.tx_hash);
        assert_eq!(live.pre_state_root, replay.pre_state_root);
        assert_eq!(live.post_state_root, replay.post_state_root);
        assert_eq!(live.execution_result_hash, replay.execution_result_hash);
        assert_eq!(live.status, replay.status);
        assert_eq!(live.state_changed, replay.state_changed);
        assert_eq!(live.previous_receipt_hash, replay.previous_receipt_hash);
    }

    assert!(live_transcript.verify_transcript().is_ok());
    assert!(replay_transcript.verify_transcript().is_ok());
}

#[test]
fn test_replay_detects_divergent_execution() {
    let txs = vec![[0xAA; 32], [0xBB; 32], [0xCC; 32]];

    let (live_transcript, _live_state, live_root) = execute_block(&txs);

    let mut replay_transcript = ExecutionTranscript::new();
    let mut state = vec![0u8];
    let mut previous_receipt_hash = [0u8; 32];

    for (i, tx_hash) in txs.iter().enumerate() {
        let receipt = execute_transition_with_receipt(
            *tx_hash,
            &state,
            |s| {
                let mut next = s.to_vec();
                next.push((i + 9) as u8); // intentional divergence
                TransitionExecutionResult {
                    post_state: next,
                    execution_result_hash: [(i + 9) as u8; 32],
                    status: ExecutionStatus::Success,
                }
            },
            previous_receipt_hash,
        )
        .expect("corrupted replay must still execute");

        state.push((i + 9) as u8);
        previous_receipt_hash = receipt.receipt_hash();
        replay_transcript.add_receipt(receipt);
    }

    let replay_root = replay_transcript
        .receipts
        .last()
        .expect("replay transcript must not be empty")
        .post_state_root;

    assert_ne!(
        live_root, replay_root,
        "divergent replay unexpectedly matched"
    );

    let live_hashes: Vec<[u8; 32]> = live_transcript
        .receipts
        .iter()
        .map(ExecutionReceipt::receipt_hash)
        .collect();
    let replay_hashes: Vec<[u8; 32]> = replay_transcript
        .receipts
        .iter()
        .map(ExecutionReceipt::receipt_hash)
        .collect();
    assert_ne!(
        live_hashes, replay_hashes,
        "divergent replay produced identical receipts"
    );
}
