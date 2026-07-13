use super::states::ConstitutionalState;

/// A postcondition that must be satisfied after a transition completes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Postcondition {
    /// The resulting state must match the expected target state.
    StateMatches {
        expected: ConstitutionalState,
        actual: ConstitutionalState,
    },
    /// The state hash must be valid after the transition.
    StateHashValid,
    /// The transition must have produced a valid transition ID.
    TransitionIdValid,
    /// Constitutional invariants must still hold.
    InvariantsPreserved,
}
