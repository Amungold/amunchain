// Execution semantics for the state machine.

use crate::economics::ConsensusResourceBudget;
use amun_failure::FailureContext;
use amun_kernel_types::{BlockHash, Gas, StateCommitment};
use heapless::Vec;

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub pre_state: StateCommitment,
    pub block_hash: BlockHash,
    pub tx_index: u32,
    pub gas_limit: Gas,
    pub budget: ConsensusResourceBudget,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub event_type: u16,
    pub payload: Vec<u8, 256>,
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ExecutionResult {
    Success {
        post_state: StateCommitment,
        gas_used: Gas,
        events: Vec<Event, 64>,
    },
    Failure {
        post_state: StateCommitment,
        gas_used: Gas,
        fault: FailureContext,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction;

#[derive(Clone, Debug)]
pub struct Block;

impl Block {
    pub fn hash(&self) -> BlockHash {
        BlockHash::default()
    }
}

pub trait StateMachine {
    fn execute_transaction(&self, ctx: &ExecutionContext, tx: &Transaction) -> ExecutionResult;

    fn execute_block(
        &self,
        pre_state: StateCommitment,
        block: &Block,
        budget: &ConsensusResourceBudget,
    ) -> Result<StateCommitment, FailureContext> {
        let _ = (pre_state, block, budget);
        Ok(StateCommitment::default())
    }
}
