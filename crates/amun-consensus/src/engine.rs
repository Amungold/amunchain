use crate::proposal::BlockProposal;
use crate::qc::QuorumCert;
use crate::round::{RoundPhase, RoundState};
use crate::safety::SafetyRules;
use crate::signature_verifier::SignatureVerifier;
use crate::validator::ValidatorSet;
use crate::vote::ConsensusVote;
use crate::vote_tracker::VoteTracker;
use amun_consensus_types::ConsensusPhase;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::{Epoch, PublicHash32, PublicKey};
use heapless::Vec;

pub struct ConsensusEngine {
    pub round_state: RoundState,
    pub validator_set: ValidatorSet,
    pub locked_qc: Option<QuorumCert>,
    pub highest_qc: Option<QuorumCert>,
    pub pending_votes: Vec<ConsensusVote, 64>,
    pub vote_tracker: VoteTracker,
    pub finalized_blocks: Vec<PublicHash32, 64>,
}

impl ConsensusEngine {
    pub fn new(epoch: Epoch) -> Self {
        Self {
            round_state: RoundState::new(epoch),
            validator_set: ValidatorSet::new(),
            locked_qc: None,
            highest_qc: None,
            pending_votes: Vec::new(),
            vote_tracker: VoteTracker::new(),
            finalized_blocks: Vec::new(),
        }
    }

    pub fn process_proposal(
        &mut self,
        proposal: &BlockProposal,
        proposer_pubkey: &PublicKey,
    ) -> AmunResult<Option<ConsensusVote>> {
        if proposal.round != self.round_state.round {
            return Ok(None);
        }
        if !SignatureVerifier::verify_proposal(proposal, proposer_pubkey)? {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000D,
                0x0100,
            ));
        }
        let block_hash = proposal.block_hash();
        let locked_round = self.locked_qc.as_ref().map(|qc| qc.round);
        let locked_hash = self.locked_qc.as_ref().map(|qc| qc.block_hash);
        if !SafetyRules::check_lock_respected(proposal.round, locked_round, locked_hash, block_hash)
        {
            return Ok(None);
        }
        if !SafetyRules::check_no_equivocation(
            proposal.round,
            block_hash,
            locked_round,
            locked_hash,
        ) {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000C,
                0x0100,
            ));
        }
        let phase = match self.round_state.phase {
            RoundPhase::Proposal => ConsensusPhase::PrepareVote,
            RoundPhase::Prepare => ConsensusPhase::PreCommitVote,
            RoundPhase::PreCommit => ConsensusPhase::CommitVote,
            _ => return Ok(None),
        };
        let validator_index = self
            .validator_set
            .get(amun_consensus_types::ValidatorIndex::new(0))
            .map(|v| v.index)
            .unwrap_or(amun_consensus_types::ValidatorIndex::new(0));
        let vote = ConsensusVote::new(phase, proposal.round, block_hash, validator_index);
        self.vote_tracker
            .record_vote(vote.round, vote.validator_index)?;
        self.add_pending_vote(vote.clone())?;
        self.validator_set
            .record_vote(validator_index, vote.round.value())?;
        Ok(Some(vote))
    }

    fn add_pending_vote(&mut self, vote: ConsensusVote) -> AmunResult<()> {
        if self
            .pending_votes
            .iter()
            .any(|v| v.validator_index == vote.validator_index)
        {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000D,
                0x0101,
            ));
        }
        self.pending_votes.push(vote).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000D, 0x0102)
        })?;
        Ok(())
    }

    pub fn finalize_block(&mut self, block_hash: PublicHash32) -> AmunResult<()> {
        self.finalized_blocks.push(block_hash).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000D, 0x0105)
        })?;
        Ok(())
    }

    pub fn advance_round(&mut self) -> AmunResult<()> {
        self.round_state.advance_round()?;
        self.pending_votes.clear();
        self.vote_tracker
            .advance_round(self.round_state.round.value());
        Ok(())
    }

    pub fn update_locked_qc(&mut self, qc: &QuorumCert) -> AmunResult<()> {
        if !SafetyRules::check_quorum(qc, &self.validator_set) {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000D,
                0x0103,
            ));
        }
        if !SafetyRules::check_no_duplicate_signers(qc) {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000D,
                0x0104,
            ));
        }
        let should_update = match &self.locked_qc {
            Some(existing) => qc.round > existing.round,
            None => true,
        };
        if should_update {
            self.locked_qc = Some(qc.clone());
        }
        let should_update_highest = match &self.highest_qc {
            Some(existing) => qc.round > existing.round,
            None => true,
        };
        if should_update_highest {
            self.highest_qc = Some(qc.clone());
        }
        Ok(())
    }
}
