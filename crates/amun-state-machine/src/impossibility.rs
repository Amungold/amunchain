use super::states::StateTag;
use super::transitions::TransitionType;

/// Proofs that certain transitions are constitutionally impossible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImpossibilityProof {
    /// Genesis can never go directly to Extinct
    GenesisToExtinctImpossible,
    /// Hostile forks cannot preserve replay
    HostileForkReplayPreservationImpossible,
    /// Incompatible freeze maps cannot merge
    IncompatibleFreezeMapMergeImpossible,
    /// Extinct civilizations cannot transition
    ExtinctCannotTransition { attempted: TransitionType },
    /// Frozen civilizations cannot amend
    FrozenCannotAmend,
}

impl ImpossibilityProof {
    pub fn prove(from: StateTag, transition: TransitionType) -> Option<Self> {
        match (from, transition) {
            (StateTag::Genesis, TransitionType::Extinct) => Some(Self::GenesisToExtinctImpossible),
            (StateTag::Extinct, _) => Some(Self::ExtinctCannotTransition {
                attempted: transition,
            }),
            (StateTag::Frozen, TransitionType::ProposeAmendment) => Some(Self::FrozenCannotAmend),
            _ => None,
        }
    }
}
