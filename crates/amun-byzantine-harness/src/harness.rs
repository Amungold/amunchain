use amun_chain_position::ChainPosition;
use amun_consensus_messages::{ConsensusPhase, ConsensusVote};
use amun_safety_laws::{check_vote_uniqueness, detect_equivocation, EquivocationEvidence};

/// Harness for testing Byzantine scenarios
pub struct ByzantineHarness;

impl ByzantineHarness {
    /// Check vote uniqueness in a set of votes
    pub fn check_uniqueness(votes: &[ConsensusVote]) -> bool {
        check_vote_uniqueness(votes)
    }

    /// Detect equivocation in a set of votes
    pub fn detect_equivocation(votes: &[ConsensusVote]) -> Vec<EquivocationEvidence> {
        detect_equivocation(votes)
    }

    /// Create a test vote
    pub fn create_vote(
        validator_id: u64,
        round: u64,
        phase: ConsensusPhase,
        block_hash: [u8; 32],
    ) -> ConsensusVote {
        ConsensusVote::new(
            validator_id,
            ChainPosition::new(0, round + 1),
            round,
            phase,
            Some(block_hash),
            [validator_id as u8; 64],
            25,
        )
    }
}
