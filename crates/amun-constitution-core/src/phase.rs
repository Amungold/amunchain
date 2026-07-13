/// Consensus phases in constitutional order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstitutionalPhase {
    Propose,
    Prevote,
    Precommit,
}

impl ConstitutionalPhase {
    pub fn tag(&self) -> u8 {
        match self {
            ConstitutionalPhase::Propose => 0,
            ConstitutionalPhase::Prevote => 1,
            ConstitutionalPhase::Precommit => 2,
        }
    }
}

/// Check if a phase transition is legal.
pub fn is_legal_phase_transition(from: ConstitutionalPhase, to: ConstitutionalPhase) -> bool {
    matches!(
        (from, to),
        (ConstitutionalPhase::Propose, ConstitutionalPhase::Prevote)
            | (ConstitutionalPhase::Prevote, ConstitutionalPhase::Precommit)
            | (ConstitutionalPhase::Precommit, ConstitutionalPhase::Propose)
            | (ConstitutionalPhase::Prevote, ConstitutionalPhase::Propose)
    )
}
