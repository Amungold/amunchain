// Operational semantics in K-framework style.

use crate::execution_semantics::{Event, Transaction};
use amun_kernel_types::StateCommitment;
use heapless::Vec;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub computation: Computation,
    pub state: StateCommitment,
    pub gas: u64,
    pub events: Vec<Event, 256>,
}

#[derive(Clone, Debug)]
pub enum Computation {
    ExecuteTx(Transaction),
    ExecuteBlock(Vec<Transaction, 256>),
    Commit,
    Done,
}

pub type RewriteRule = fn(&Configuration) -> Option<Configuration>;

pub fn execute_semantics(_initial: Configuration) -> Configuration {
    Configuration {
        computation: Computation::Done,
        state: StateCommitment::default(),
        gas: 0,
        events: Vec::new(),
    }
}

pub const RULES: &[RewriteRule] = &[];
