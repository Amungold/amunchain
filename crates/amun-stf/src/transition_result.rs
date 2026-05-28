extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use amun_execution_receipt::{ExecutionReceipt, ExecutionStatus};

pub struct TransitionExecutionResult {
    pub post_state: Vec<u8>,
    pub execution_result_hash: [u8; 32],
    pub status: ExecutionStatus,
}

pub fn execute_transition_with_receipt(
    tx_hash: [u8; 32],
    pre_state: &[u8],
    execute_logic: impl FnOnce(&[u8]) -> TransitionExecutionResult,
    previous_receipt_hash: [u8; 32],
) -> Result<ExecutionReceipt, String> {
    let pre_root = {
        use amun_state_root::root::{StateLeaf, StateRootEngine};
        let leaves = vec![StateLeaf {
            key: String::from("state"),
            value: pre_state.to_vec(),
        }];
        StateRootEngine::domain_root(&leaves).map_err(|e| format!("pre_root: {}", e))?
    };

    let result = execute_logic(pre_state);

    let post_root = {
        use amun_state_root::root::{StateLeaf, StateRootEngine};
        let leaves = vec![StateLeaf {
            key: String::from("state"),
            value: result.post_state.clone(),
        }];
        StateRootEngine::domain_root(&leaves).map_err(|e| format!("post_root: {}", e))?
    };

    let state_changed = pre_root != post_root;

    let receipt = ExecutionReceipt::new(
        tx_hash,
        pre_root,
        post_root,
        result.execution_result_hash,
        result.status,
        state_changed,
        previous_receipt_hash,
    );

    receipt.verify_consistency().map_err(alloc::string::String::from)?;
    Ok(receipt)
}
