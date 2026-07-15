use crate::error::{PlatformResult, StateMachineError, StateMachineErrorCode};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RuntimeState {
    Created,
    Provisioning,
    IdentityReady,
    StorageReady,
    Bootstrapping,
    GenesisReady,
    NetworkReady,
    Discovering,
    Syncing,
    Verifying,
    Candidate,
    Voting,
    Maintenance,
    Upgrading,
    Recovering,
    Draining,
    Suspended,
    Retired,
}

impl RuntimeState {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(RuntimeState::Created),
            1 => Some(RuntimeState::Provisioning),
            2 => Some(RuntimeState::IdentityReady),
            3 => Some(RuntimeState::StorageReady),
            4 => Some(RuntimeState::Bootstrapping),
            5 => Some(RuntimeState::GenesisReady),
            6 => Some(RuntimeState::NetworkReady),
            7 => Some(RuntimeState::Discovering),
            8 => Some(RuntimeState::Syncing),
            9 => Some(RuntimeState::Verifying),
            10 => Some(RuntimeState::Candidate),
            11 => Some(RuntimeState::Voting),
            12 => Some(RuntimeState::Maintenance),
            13 => Some(RuntimeState::Upgrading),
            14 => Some(RuntimeState::Recovering),
            15 => Some(RuntimeState::Draining),
            16 => Some(RuntimeState::Suspended),
            17 => Some(RuntimeState::Retired),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeTransition {
    pub from: RuntimeState,
    pub to: RuntimeState,
    pub required_capabilities: Vec<String>,
    pub required_checks: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct StateMachine {
    transitions: BTreeMap<RuntimeState, Vec<RuntimeState>>,
}

impl StateMachine {
    pub fn new() -> Self {
        let mut transitions = BTreeMap::new();
        transitions.insert(RuntimeState::Created, vec![RuntimeState::Provisioning]);
        transitions.insert(
            RuntimeState::Provisioning,
            vec![RuntimeState::IdentityReady],
        );
        transitions.insert(
            RuntimeState::IdentityReady,
            vec![RuntimeState::StorageReady],
        );
        transitions.insert(
            RuntimeState::StorageReady,
            vec![RuntimeState::Bootstrapping],
        );
        transitions.insert(
            RuntimeState::Bootstrapping,
            vec![RuntimeState::GenesisReady],
        );
        transitions.insert(RuntimeState::GenesisReady, vec![RuntimeState::NetworkReady]);
        transitions.insert(RuntimeState::NetworkReady, vec![RuntimeState::Discovering]);
        transitions.insert(RuntimeState::Discovering, vec![RuntimeState::Syncing]);
        transitions.insert(
            RuntimeState::Syncing,
            vec![RuntimeState::Verifying, RuntimeState::Recovering],
        );
        transitions.insert(
            RuntimeState::Verifying,
            vec![RuntimeState::Syncing, RuntimeState::Candidate],
        );
        transitions.insert(RuntimeState::Candidate, vec![RuntimeState::Voting]);
        transitions.insert(
            RuntimeState::Voting,
            vec![
                RuntimeState::Maintenance,
                RuntimeState::Upgrading,
                RuntimeState::Recovering,
                RuntimeState::Draining,
                RuntimeState::Suspended,
            ],
        );
        transitions.insert(RuntimeState::Maintenance, vec![RuntimeState::Voting]);
        transitions.insert(RuntimeState::Upgrading, vec![RuntimeState::Voting]);
        transitions.insert(RuntimeState::Recovering, vec![RuntimeState::Voting]);
        transitions.insert(RuntimeState::Draining, vec![RuntimeState::Retired]);
        transitions.insert(RuntimeState::Suspended, vec![RuntimeState::Retired]);
        transitions.insert(RuntimeState::Retired, vec![]);
        StateMachine { transitions }
    }

    pub fn can_transition(&self, from: RuntimeState, to: RuntimeState) -> bool {
        self.transitions
            .get(&from)
            .map(|a| a.contains(&to))
            .unwrap_or(false)
    }

    pub fn validate_transition(&self, from: RuntimeState, to: RuntimeState) -> PlatformResult<()> {
        if !self.can_transition(from, to) {
            return Err(crate::error::PlatformError::StateMachine(
                StateMachineError::new(
                    StateMachineErrorCode::IllegalTransition,
                    format!("Illegal transition from {:?} to {:?}", from, to),
                ),
            ));
        }
        Ok(())
    }

    pub fn allowed_next_states(&self, current: RuntimeState) -> Vec<RuntimeState> {
        self.transitions.get(&current).cloned().unwrap_or_default()
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
