use crate::state::{ConsensusState, ConsensusStep};
use crate::pacemaker::Pacemaker;
use crate::vote_collector::VoteCollector;
use crate::validator::ValidatorSet;
use crate::types::{BlockProposal, Vote, VoteType, QuorumCertificate};
use crate::action::{ConsensusAction, ActionLog};

pub struct RoundStateMachine {
    pub local_validator_id: [u8; 32],
    pub state: ConsensusState,
    pacemaker: Pacemaker,
    vote_collector: VoteCollector,
    pub pending_commit_qc: Option<QuorumCertificate>,
    pub last_committed_height: u64,
    pub committed_qc: Option<QuorumCertificate>,
    pub pending_actions: Vec<ConsensusAction>,
    pub action_log: ActionLog,
}

impl RoundStateMachine {
    pub fn new(height: u64, pacemaker: Pacemaker, local_validator_id: [u8; 32]) -> Self {
        Self {
            local_validator_id,
            state: ConsensusState::new(height),
            pacemaker,
            vote_collector: VoteCollector::new(),
            pending_commit_qc: None,
            last_committed_height: height.saturating_sub(1),
            committed_qc: None,
            pending_actions: Vec::new(),
            action_log: ActionLog::default(),
        }
    }

    fn emit(&mut self, action: ConsensusAction) {
        let step_name = format!("{:?}", self.state.step);
        self.action_log.record(
            action.clone(),
            self.state.height,
            self.state.round,
            &step_name,
            self.local_validator_id,
        );
        self.pending_actions.push(action);
    }

    pub fn propose(&mut self, block_hash: [u8; 32], proposer_id: [u8; 32]) {
        if self.state.step != ConsensusStep::Propose { return; }
        self.emit(ConsensusAction::BroadcastProposal(BlockProposal {
            height: self.state.height,
            block_hash,
            proposer: proposer_id,
            round: self.state.round,
            timestamp: 0,
        }));
    }

    pub fn accept_proposal(&mut self, block_hash: [u8; 32]) {
        if self.state.step == ConsensusStep::Propose {
            self.state.see_valid(block_hash);
            self.state.advance_step();
            self.emit(ConsensusAction::BroadcastPrevote(Vote {
                height: self.state.height, block_hash,
                voter: self.local_validator_id, round: self.state.round,
                vote_type: VoteType::Prevote, timestamp: 0,
            }));
        }
    }

    pub fn process_vote(&mut self, vote: Vote, validator_set: &ValidatorSet) -> Option<QuorumCertificate> {
        if vote.height != self.state.height || vote.round != self.state.round { return None; }
        let expected = match self.state.step {
            ConsensusStep::Prevote => VoteType::Prevote,
            ConsensusStep::Precommit => VoteType::Precommit,
            _ => return None,
        };
        if vote.vote_type != expected { return None; }
        let qc = self.vote_collector.add_vote(vote, validator_set);
        if qc.is_some() {
            match self.state.step {
                ConsensusStep::Prevote => {
                    if let Some(ref qc) = qc { self.state.see_valid(qc.block_hash); }
                    self.state.advance_step();
                    self.emit(ConsensusAction::BroadcastPrecommit(Vote {
                        height: self.state.height,
                        block_hash: qc.as_ref().map(|q| q.block_hash).unwrap_or([0u8; 32]),
                        voter: self.local_validator_id, round: self.state.round,
                        vote_type: VoteType::Precommit, timestamp: 0,
                    }));
                }
                ConsensusStep::Precommit => {
                    self.state.advance_step();
                    self.pending_commit_qc = qc.clone();
                    if let Some(ref qc) = qc {
                        self.emit(ConsensusAction::Commit(qc.clone()));
                    }
                }
                _ => {}
            }
        }
        qc
    }

    pub fn finalize_commit(&mut self) -> Result<QuorumCertificate, &'static str> {
        let qc = self.pending_commit_qc.take().ok_or("no pending commit")?;
        self.last_committed_height = self.state.height;
        self.state.commit()?;
        self.vote_collector.reset();
        self.committed_qc = Some(qc.clone());
        Ok(qc)
    }

    pub fn advance_round(&mut self) -> Result<(), &'static str> {
        let from = self.state.round;
        self.state.advance_round()?;
        self.emit(ConsensusAction::AdvanceRound { from, to: self.state.round });
        self.vote_collector.reset();
        Ok(())
    }

    pub fn current_timeout_ms(&self) -> Option<u64> { self.pacemaker.timeout_ms(self.state.round, self.state.step) }
    pub fn is_committed(&self) -> bool { self.pending_commit_qc.is_some() && self.state.step == ConsensusStep::Commit }
}
