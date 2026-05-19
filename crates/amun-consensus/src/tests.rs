#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::engine::ConsensusEngine;
    use crate::round::{RoundState, RoundPhase};
    use crate::validator::ValidatorSet;
    use amun_kernel_types::*;
    use amun_consensus_types::*;

    #[test]
    fn test_phase_mapping() { assert!(ConsensusPhase::PrepareVote.is_vote()); assert!(!ConsensusPhase::Proposal.is_vote()); }
    #[test]
    fn test_consensus_engine_init() { let e = ConsensusEngine::new(Epoch::new(1)); assert_eq!(e.round_state.phase, RoundPhase::Proposal); assert!(e.locked_qc.is_none()); assert!(e.highest_qc.is_none()); assert_eq!(e.finalized_blocks.len(), 0); }
    #[test]
    fn test_round_state_initial() { let s = RoundState::new(Epoch::new(1)); assert_eq!(s.phase, RoundPhase::Proposal); assert_eq!(s.round, ConsensusRound::new(0)); }
    #[test]
    fn test_round_phase_advance() { let mut s = RoundState::new(Epoch::new(1)); s.advance_phase().unwrap(); assert_eq!(s.phase, RoundPhase::Prepare); s.advance_phase().unwrap(); assert_eq!(s.phase, RoundPhase::PreCommit); s.advance_phase().unwrap(); assert_eq!(s.phase, RoundPhase::Commit); s.advance_phase().unwrap(); assert_eq!(s.phase, RoundPhase::Proposal); }
    #[test]
    fn test_validator_set_quorum() { let mut vs = ValidatorSet::new(); for i in 0..4u8 { vs.add_validator(ValidatorId::new([i;32]), PublicKey::new([0u8;48])).unwrap(); } assert_eq!(vs.byzantine_threshold(), 1); assert_eq!(vs.quorum_threshold(), 3); assert!(vs.is_quorum(3)); assert!(!vs.is_quorum(2)); }
    #[test]
    fn test_finalize_block() { let mut e = ConsensusEngine::new(Epoch::new(1)); e.finalize_block(PublicHash32::new([1u8;32])).unwrap(); e.finalize_block(PublicHash32::new([2u8;32])).unwrap(); assert_eq!(e.finalized_blocks.len(), 2); }
    #[test]
    fn test_finalize_block_deterministic() { let mut e1 = ConsensusEngine::new(Epoch::new(1)); let mut e2 = ConsensusEngine::new(Epoch::new(1)); let h1 = PublicHash32::new([0xAAu8;32]); let h2 = PublicHash32::new([0xBBu8;32]); e1.finalize_block(h1).unwrap(); e1.finalize_block(h2).unwrap(); e2.finalize_block(h1).unwrap(); e2.finalize_block(h2).unwrap(); assert_eq!(e1.finalized_blocks.len(), e2.finalized_blocks.len()); }
}
