use amun_execution_receipt::{ExecutionStatus, ExecutionTranscript};
use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};

#[test]
fn test_truth_bound_execution_creates_valid_receipt() {
    let tx_hash = [0xAB; 32];
    let pre_state = vec![1u8, 2, 3];
    let previous_receipt_hash = [0x00; 32];

    let receipt = execute_transition_with_receipt(
        tx_hash,
        &pre_state,
        |state| {
            let mut new_state = state.to_vec();
            new_state.push(4); // mutate state
            TransitionExecutionResult {
                post_state: new_state,
                execution_result_hash: [0xCD; 32],
                status: ExecutionStatus::Success,
            }
        },
        previous_receipt_hash,
    )
    .expect("receipt creation should succeed");

    // Verify receipt consistency
    assert!(receipt.verify_consistency().is_ok());
    // State was mutated, so state_changed must be true
    assert!(receipt.state_changed);
    // Roots must differ
    assert_ne!(receipt.pre_state_root, receipt.post_state_root);
}

#[test]
fn test_truth_bound_execution_detects_anomaly() {
    let tx_hash = [0xAB; 32];
    let pre_state = vec![1u8, 2, 3];
    let previous_receipt_hash = [0x00; 32];

    // Execute a "mutation" that doesn't actually change state
    let receipt = execute_transition_with_receipt(
        tx_hash,
        &pre_state,
        |state| {
            TransitionExecutionResult {
                post_state: state.to_vec(), // unchanged
                execution_result_hash: [0xCD; 32],
                status: ExecutionStatus::Success,
            }
        },
        previous_receipt_hash,
    )
    .expect("receipt creation should succeed");

    // Verify consistency: state_changed must be false because roots are equal
    assert!(!receipt.state_changed);
    assert_eq!(receipt.pre_state_root, receipt.post_state_root);
}

#[test]
fn test_transcript_continuity_with_evolving_state() {
    let mut transcript = ExecutionTranscript::new();
    let mut current_state = vec![1u8]; // initial state (actual bytes, not root)
    let mut prev_hash = [0x00; 32];

    for i in 0..5 {
        let tx_hash = [i as u8; 32];
        let receipt = execute_transition_with_receipt(
            tx_hash,
            &current_state,
            |state| {
                let mut new_state = state.to_vec();
                new_state.push(i + 1); // deterministic evolution
                TransitionExecutionResult {
                    post_state: new_state,
                    execution_result_hash: [0xCD; 32],
                    status: ExecutionStatus::Success,
                }
            },
            prev_hash,
        )
        .expect("receipt creation should succeed");

        // Advance state for next iteration — evolve actual state, not commitment.
        let mut next_state = current_state.clone();
        next_state.push(i + 1);
        current_state = next_state;

        prev_hash = receipt.receipt_hash();
        transcript.add_receipt(receipt);
    }

    // Transcript must be internally consistent
    assert!(transcript.verify_transcript().is_ok());
    // All receipts should indicate state changed
    for receipt in &transcript.receipts {
        assert!(receipt.state_changed);
    }
}
