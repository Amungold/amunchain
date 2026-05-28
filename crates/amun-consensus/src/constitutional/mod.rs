//! Constitutional Core - Single Source of Truth

mod hashable;
mod transition;

pub use hashable::{ConstitutionalHashable, ConstitutionalState, ConstitutionalTransition};
pub use transition::{VerifiedTransitionWitness, ExecutionWitness, WitnessChainVerifier};
