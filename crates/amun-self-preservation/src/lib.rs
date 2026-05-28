// Constitutional Self-Preservation
// Meta-constitutional layer: protects the constitution's ability
// to protect itself. Prevents recursive legitimacy paradoxes,
// enforces least invariant violation principle, and detects
// constitutional phase transitions.

pub mod action_principle;
pub mod consistency;
pub mod legitimacy_guards;
pub mod phase_transitions;
pub mod self_modeling;

pub use action_principle::{ConstitutionalAction, LeastInvariantViolation};
pub use consistency::{ConsistencyViolation, MetaConsistency};
pub use legitimacy_guards::{GuardViolation, LegitimacyGuard};
pub use phase_transitions::{PhaseTransition, TransitionType};
pub use self_modeling::{SelfModel, SelfReferenceGuard};
