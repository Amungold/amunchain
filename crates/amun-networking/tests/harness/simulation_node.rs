// Allow dead code in test harness — these are used by v2 scenarios
#![allow(dead_code)]
use amun_consensus::{types::BlockProposal, validator::Validator, round_state_machine::RoundStateMachine};
use amun_networking::transport::MockTransport;
use amun_constitutional_block::Blockchain;
use std::collections::HashSet;
use std::fmt;

pub struct SimulationNodeCore {
    pub validator: Validator,
    pub transport: MockTransport,
    pub state_machine: RoundStateMachine,
    pub last_proposal: Option<BlockProposal>,
    pub seen_proposals: HashSet<[u8; 32]>,
    pub blockchain: Blockchain,
}

impl SimulationNodeCore {
    pub fn new(validator: Validator, state_machine: RoundStateMachine) -> Self {
        Self {
            validator,
            transport: MockTransport::new(),
            state_machine,
            last_proposal: None,
            seen_proposals: HashSet::new(),
            blockchain: Blockchain::new(),
        }
    }
}

impl fmt::Debug for SimulationNodeCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimulationNodeCore")
            .field("validator", &self.validator)
            .field("last_proposal", &self.last_proposal)
            .field("seen_proposals_count", &self.seen_proposals.len())
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct ScenarioNodeState {}
