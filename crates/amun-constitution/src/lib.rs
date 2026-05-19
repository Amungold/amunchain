#![no_std]

pub mod activation;
pub mod adaptive_adversary;
pub mod adversary_model;
pub mod canonical_form;
pub mod capacity;
pub mod deterministic_execution;
pub mod economics;
pub mod execution_semantics;
pub mod filesystem_semantics;
pub mod fork_choice;
pub mod liveness;
pub mod operational_semantics;
pub mod quorum_transition;
pub mod refinement_chain;
pub mod rushing_adversary;
pub mod stake_quorum;
pub mod storage;
pub mod synchrony;

// Phase 2C modules
pub mod hash_domains;
pub mod replay;
pub mod gas;
pub mod crypto;

pub use activation::{ActivationSchedule, ActivationStatus};
pub use adversary_model::{AdversaryCapabilities, AdversaryLimitations};
pub use canonical_form::{ConstitutionalCanonicalForm, CURRENT_CCF};
pub use capacity::{constitutional_capacity, ProtocolCapacities};
pub use economics::ConsensusResourceBudget;
pub use execution_semantics::{Event, ExecutionContext, ExecutionResult, StateMachine};
pub use fork_choice::{BlockDAG, ForkChoiceFunction, ForkChoiceState, PreferredChain};
pub use liveness::{LogicalLivenessParameters, TimeoutCertificate};
pub use quorum_transition::QuorumTransitionParameters;
pub use stake_quorum::{StakeWeight, WeightedValidator, WeightedValidatorSet};
pub use storage::{WALEntry, WALEntryState};
pub use synchrony::{GlobalStabilizationTime, SynchronyModel, SynchronyProof};
pub use hash_domains::HashDomain;
pub use replay::ReplayContext;
pub use gas::{GasMeter, gas_costs};

#[cfg(test)]
mod tests;
