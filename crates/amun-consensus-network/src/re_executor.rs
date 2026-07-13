// N109.7: Deterministic Re-Execution
// ============================================================================
use crate::execution_receipt::ExecutionReceipt;
use crate::messages::N109BlockProposal;

/// N109.7: Re-execute a block and verify the state root.
pub fn verify_block_execution<F>(
    proposal: &N109BlockProposal,
    mut executor: F,
) -> Result<ExecutionReceipt, String>
where
    F: FnMut(&[u8]) -> Result<[u8; 32], String>,
{
    let computed_root =
        executor(&proposal.block_bytes).map_err(|e| format!("N109.7 EXECUTION_FAILED: {}", e))?;

    if computed_root != proposal.state_root {
        return Err(format!(
            "N109.7 STATE_ROOT_MISMATCH: height={} proposed={} computed={}",
            proposal.height,
            hex::encode(proposal.state_root),
            hex::encode(computed_root),
        ));
    }

    Ok(ExecutionReceipt {
        state_root: computed_root,
        height: proposal.height,
        block_hash: proposal.block_hash,
        success: true,
    })
}
