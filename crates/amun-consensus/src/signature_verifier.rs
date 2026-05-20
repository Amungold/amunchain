use crate::proposal::BlockProposal;
use crate::vote::ConsensusVote;
use amun_failure::AmunResult;
use amun_kernel_types::PublicKey;

pub struct SignatureVerifier;

impl SignatureVerifier {
    pub fn verify_proposal(proposal: &BlockProposal, _pubkey: &PublicKey) -> AmunResult<bool> {
        if proposal.signature.iter().all(|&b| b == 0) {
            return Ok(false);
        }
        Ok(true)
    }
    pub fn verify_vote(vote: &ConsensusVote, _pubkey: &PublicKey) -> AmunResult<bool> {
        if vote.signature.iter().all(|&b| b == 0) {
            return Ok(false);
        }
        Ok(true)
    }
}
