#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::all)]

pub mod absolute_invariants;
pub mod axioms;
pub mod causality;
pub mod delta_algebra;
pub mod delta_laws;
pub mod derivation;
pub mod engine;
pub mod fork_merge;
pub mod formal_entropy;
pub mod historical_invariants;
pub mod impossibility;
pub mod invariants;
pub mod merge_math;
pub mod meta_amendment;
pub mod postconditions;
pub mod preconditions;
pub mod reconciliation;
pub mod replay_log;
pub mod stability;
pub mod states;
pub mod thermodynamics;
pub mod transitions;
pub mod verifier;

pub use absolute_invariants::AbsoluteInvariant;
pub use axioms::{AxiomVerification, ConstitutionalAxiom, ConstitutionalAxiomEngine};
pub use causality::{CausalHeight, CausalityMetadata, LamportTime, VectorClock};
pub use delta_algebra::ConstitutionalDelta;
pub use delta_laws::DeltaLaw;
pub use derivation::{ConsistencyProof, Derivation, DerivationStep, InferenceRule, Theorem};
pub use engine::{
    ConstitutionalExecutionEngine, ConstitutionalPolicyEngine, ConstitutionalProofEngine,
};
pub use fork_merge::{ForkDeclaration, ForkType, MergeDeclaration, MergeType};
pub use formal_entropy::{
    EntropyCollapseThreshold, EntropyConservationLaws, EntropySink, EntropySinkType, FormalEntropy,
};
pub use historical_invariants::{HistoricalInvariant, HistoricalInvariantEngine};
pub use impossibility::ImpossibilityProof;
pub use invariants::ConstitutionalInvariant;
pub use merge_math::{MergeConflictResolver, MergeResolution};
pub use meta_amendment::{MetaAmendmentEngine, MetaAmendmentLaw, MetaAmendmentScope, MetaQuorum};
pub use postconditions::Postcondition;
pub use preconditions::Precondition;
pub use reconciliation::{ReconciliationEngine, ReconciliationStrategy};
pub use replay_log::{ConstitutionalReplayDAG, ReplayLogEntry};
pub use stability::ConstitutionalStabilityMetrics;
pub use states::{ConstitutionalState, StateTag};
pub use thermodynamics::{ConstitutionalEntropy, StabilityEquations};
pub use transitions::{
    Transition, TransitionAlgebra, TransitionId, TransitionProof, TransitionType,
};
pub use verifier::TransitionVerifier;
