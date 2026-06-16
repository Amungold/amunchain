use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A validator's vote on a governance proposal.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernanceVote {
    Approve,
    Reject,
    Abstain,
}

/// Tracks all votes for a single governance proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalVotes {
    pub proposal_id: [u8; 32],
    votes: BTreeMap<[u8; 32], GovernanceVote>,
}

/// Result of a vote tally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoteTally {
    pub approvals: usize,
    pub rejections: usize,
    pub abstentions: usize,
}

impl ProposalVotes {
    pub fn new(proposal_id: [u8; 32]) -> Self {
        Self {
            proposal_id,
            votes: BTreeMap::new(),
        }
    }

    /// Submit or replace a validator's vote. The latest vote always wins.
    pub fn submit_vote(&mut self, validator_id: [u8; 32], vote: GovernanceVote) {
        self.votes.insert(validator_id, vote);
    }

    /// Count total participating validators.
    pub fn total_votes(&self) -> usize {
        self.votes.len()
    }

    /// Tally the current votes into approvals, rejections, and abstentions.
    pub fn tally(&self) -> VoteTally {
        let mut approvals = 0;
        let mut rejections = 0;
        let mut abstentions = 0;
        for v in self.votes.values() {
            match v {
                GovernanceVote::Approve => approvals += 1,
                GovernanceVote::Reject => rejections += 1,
                GovernanceVote::Abstain => abstentions += 1,
            }
        }
        VoteTally { approvals, rejections, abstentions }
    }

    /// Check whether quorum has been reached (2/3 of total validators).
    pub fn reached_quorum(&self, total_validators: usize) -> bool {
        self.total_votes() * 3 > total_validators * 2
    }

    /// Determine whether the proposal has been approved.
    pub fn is_approved(&self, total_validators: usize) -> bool {
        self.reached_quorum(total_validators)
            && self.tally().approvals > self.tally().rejections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n107_7_vote_submission() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        pv.submit_vote([0xAA; 32], GovernanceVote::Approve);
        assert_eq!(pv.total_votes(), 1);
    }

    #[test]
    fn n107_7_duplicate_vote_replaces_old_vote() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        pv.submit_vote([0xAA; 32], GovernanceVote::Approve);
        pv.submit_vote([0xAA; 32], GovernanceVote::Reject);
        assert_eq!(pv.total_votes(), 1);
        let tally = pv.tally();
        assert_eq!(tally.rejections, 1);
        assert_eq!(tally.approvals, 0);
    }

    #[test]
    fn n107_7_quorum_required() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        // 2 validators out of 4 -> no quorum
        pv.submit_vote([1u8; 32], GovernanceVote::Approve);
        pv.submit_vote([2u8; 32], GovernanceVote::Approve);
        assert!(!pv.reached_quorum(4));
    }

    #[test]
    fn n107_7_majority_approval() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        pv.submit_vote([1u8; 32], GovernanceVote::Approve);
        pv.submit_vote([2u8; 32], GovernanceVote::Approve);
        pv.submit_vote([3u8; 32], GovernanceVote::Reject);
        // Quorum: 3/4 > 2/3 -> yes; approvals: 2 > rejections: 1
        assert!(pv.is_approved(4));
    }

    #[test]
    fn n107_7_majority_rejection() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        pv.submit_vote([1u8; 32], GovernanceVote::Reject);
        pv.submit_vote([2u8; 32], GovernanceVote::Reject);
        pv.submit_vote([3u8; 32], GovernanceVote::Approve);
        // Quorum: 3/4; approvals: 1, rejections: 2 -> not approved
        assert!(!pv.is_approved(4));
    }

    #[test]
    fn n107_7_abstain_counted_for_quorum() {
        let mut pv = ProposalVotes::new([1u8; 32]);
        pv.submit_vote([1u8; 32], GovernanceVote::Approve);
        pv.submit_vote([2u8; 32], GovernanceVote::Approve);
        pv.submit_vote([3u8; 32], GovernanceVote::Abstain);
        // Quorum: 3/4; approvals: 2, rejections: 0 -> approved
        assert!(pv.is_approved(4));
    }

    #[test]
    fn n107_7_empty_vote_set() {
        let pv = ProposalVotes::new([1u8; 32]);
        assert!(!pv.reached_quorum(4));
        assert!(!pv.is_approved(4));
    }
}
