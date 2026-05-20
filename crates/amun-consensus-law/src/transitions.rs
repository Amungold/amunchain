#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusPhase {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

pub struct TransitionLaw;

impl TransitionLaw {
    pub fn valid_transition(from: ConsensusPhase, to: ConsensusPhase) -> bool {
        matches!(
            (from, to),
            (ConsensusPhase::Propose, ConsensusPhase::Prevote)
                | (ConsensusPhase::Prevote, ConsensusPhase::Precommit)
                | (ConsensusPhase::Precommit, ConsensusPhase::Commit)
                | (ConsensusPhase::Commit, ConsensusPhase::Propose)
                | (ConsensusPhase::Propose, ConsensusPhase::Propose)
                | (ConsensusPhase::Prevote, ConsensusPhase::Propose)
                | (ConsensusPhase::Precommit, ConsensusPhase::Propose)
        )
    }

    pub fn requires_quorum(phase: ConsensusPhase) -> bool {
        matches!(
            phase,
            ConsensusPhase::Prevote | ConsensusPhase::Precommit | ConsensusPhase::Commit
        )
    }

    pub fn quorum_threshold(n: u64) -> u64 {
        (n * 2 / 3) + 1
    }
}
