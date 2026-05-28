//! Vote serialization

use crate::constitutional_vote::ConstitutionalVote;

pub struct CanonicalVoteSerializer;

impl CanonicalVoteSerializer {
    pub fn to_canonical_bytes(vote: &ConstitutionalVote) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&vote.block_height.to_be_bytes());
        bytes
    }
    
    pub fn vote_hash(vote: &ConstitutionalVote) -> [u8; 32] {
        let bytes = Self::to_canonical_bytes(vote);
        blake3::hash(&bytes).into()
    }
}
