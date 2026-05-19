// Consensus resource budget.

use amun_failure::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};

pub const MAX_HASH_OPS_PER_BLOCK: u64 = 10_000;
pub const MAX_SIGNATURE_OPS_PER_BLOCK: u64 = 1_000;
pub const MAX_CALL_DEPTH: usize = 32;

#[derive(Clone, Debug)]
pub struct ConsensusResourceBudget {
    pub hash_ops_remaining: u64,
    pub signature_ops_remaining: u64,
    pub stack_depth_remaining: usize,
}

impl ConsensusResourceBudget {
    pub fn new() -> Self {
        Self {
            hash_ops_remaining: MAX_HASH_OPS_PER_BLOCK,
            signature_ops_remaining: MAX_SIGNATURE_OPS_PER_BLOCK,
            stack_depth_remaining: MAX_CALL_DEPTH,
        }
    }

    pub fn consume_hash_op(&mut self) -> AmunResult<()> {
        self.hash_ops_remaining = self.hash_ops_remaining.checked_sub(1).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::CryptoBudgetExceeded,
                module_ids::AMUN_CONSTITUTION,
                operation_ids::BUDGET_HASH,
            )
        })?;
        Ok(())
    }

    pub fn consume_signature_op(&mut self) -> AmunResult<()> {
        self.signature_ops_remaining =
            self.signature_ops_remaining.checked_sub(1).ok_or_else(|| {
                FailureContext::new(
                    ConstitutionalFault::CryptoBudgetExceeded,
                    module_ids::AMUN_CONSTITUTION,
                    operation_ids::BUDGET_SIGNATURE,
                )
            })?;
        Ok(())
    }
}

impl Default for ConsensusResourceBudget {
    fn default() -> Self {
        Self::new()
    }
}
