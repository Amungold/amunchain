/// Constitutional invariants that must be preserved across ALL transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalInvariant {
    /// The civilization must have a valid lineage.
    LineageIntact,
    /// The constitution must be self-consistent.
    ConstitutionConsistent,
    /// Freeze boundaries must not have been violated.
    FreezeBoundariesIntact,
    /// Replay determinism must be preserved.
    ReplayDeterminismPreserved,
    /// The state machine must not be in an impossible configuration.
    NoImpossibleState,
    /// The transition history must be acyclic (no state loops).
    TransitionHistoryAcyclic,
    /// Constitutional identity must not have been silently mutated.
    IdentityNotSilentlyMutated,
}
