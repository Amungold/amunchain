use amun_consensus_messages::ConsensusVote;
use std::collections::HashSet;

pub fn check_vote_uniqueness(votes: &[ConsensusVote]) -> bool {
    let mut seen_validators = HashSet::new();
    for vote in votes {
        if !seen_validators.insert(vote.message.validator_id) {
            return false;
        }
    }
    true
}

pub fn are_votes_distinct(vote_a: &ConsensusVote, vote_b: &ConsensusVote) -> bool {
    vote_a.message.validator_id != vote_b.message.validator_id
        || vote_a.message.block_hash != vote_b.message.block_hash
}
