use crate::messages::{ConsensusVote, FinalityCertificate, QuorumCertificate};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct ConsensusRound {
    pub height: u64,
    pub proposer_id: [u8; 32],
    pub proposed_block_hash: Option<[u8; 32]>,
    pub proposed_state_root: Option<[u8; 32]>,
    pub votes: Vec<ConsensusVote>,
    pub qc: Option<QuorumCertificate>,
    pub finality: Option<FinalityCertificate>,
    pub complete: bool,
    pub double_vote_evidence: Vec<DoubleVoteEvidence>,
}

impl ConsensusRound {
    pub fn new(height: u64, proposer_id: [u8; 32]) -> Self {
        Self {
            height,
            proposer_id,
            proposed_block_hash: None,
            proposed_state_root: None,
            votes: Vec::new(),
            qc: None,
            finality: None,
            complete: false,
            double_vote_evidence: Vec::new(),
        }
    }

    /// Proposer sets the block for this round.
    pub fn propose(&mut self, block_hash: [u8; 32], state_root: [u8; 32]) {
        // N130.3: never overwrite an already established proposal
        if self.proposed_block_hash.is_some() {
            return;
        }

        self.proposed_block_hash = Some(block_hash);
        self.proposed_state_root = Some(state_root);
    }

    /// Add a validator's vote. Rejects duplicate voters.
    pub fn add_vote(&mut self, vote: &ConsensusVote) -> Result<(), String> {
        // N130.3: recover proposal if vote arrives before proposal
        if self.proposed_block_hash.is_none() {
            self.proposed_block_hash = Some(vote.block_hash);
            self.proposed_state_root = Some(vote.state_root);

            eprintln!(
                "PROPOSAL_RECOVERED: h={} hash={:?}",
                self.height,
                &vote.block_hash[..4]
            );
        }

        if vote.height != self.height {
            return Err(format!(
                "Vote height {} != round height {}",
                vote.height, self.height
            ));
        }
        // Check for equivocation FIRST (before checking proposed_block_hash)
        if let Some(existing) = self.votes.iter().find(|v| v.voter_id == vote.voter_id) {
            if existing.block_hash != vote.block_hash {
                return Err("Equivocation detected: double vote for different blocks".into());
            }
            return Err("Duplicate vote from validator".into());
        }
        if let Some(hash) = self.proposed_block_hash {
            if vote.block_hash != hash {
                return Err("Vote for different block".into());
            }
        }
        self.votes.push(vote.clone());
        Ok(())
    }

    /// Try to form a QC from collected votes.
    /// Returns Some(QC) if >2/3 validators approved.
    pub fn try_form_qc(
        &mut self,
        total_validators: usize,
        validator_powers: &HashMap<[u8; 32], u64>,
        total_voting_power: u64,
    ) -> Option<QuorumCertificate> {
        let approvals: Vec<ConsensusVote> =
            self.votes.iter().filter(|v| v.approve).cloned().collect();
        let approval_count = approvals.len() as u64;

        let (approval_power, quorum_met) = if total_voting_power > 0 {
            // Weighted voting: sum the power of each approving validator
            let power = approvals
                .iter()
                .map(|v| validator_powers.get(&v.voter_id).copied().unwrap_or(0))
                .sum();
            (power, power * 3 > total_voting_power * 2)
        } else {
            // Legacy fallback: count votes when powers are not set
            (
                approval_count,
                approval_count * 3 > total_validators as u64 * 2,
            )
        };

        eprintln!(
            "ROUND_DIAG: h={} votes={} approvals={} power={}/{}",
            self.height,
            self.votes.len(),
            approvals.len(),
            approval_power,
            if total_voting_power > 0 {
                total_voting_power
            } else {
                total_validators as u64
            }
        );

        eprintln!(
            "QC_CHECK: approvals={} votes={} power={} total={} quorum={} validators={}",
            approvals.len(),
            self.votes.len(),
            approval_power,
            total_voting_power,
            quorum_met,
            validator_powers.len()
        );

        for (vid, power) in validator_powers.iter() {
            eprintln!("QC_VALIDATOR: id={:?} power={}", &vid[..4], power);
        }

        if !quorum_met {
            return None;
        }

        eprintln!(
            "QC_FORMED: h={} approvals={} power={}",
            self.height,
            approvals.len(),
            approval_power
        );

        let qc = QuorumCertificate {
            height: self.height,
            block_hash: self.proposed_block_hash?,
            state_root: self.proposed_state_root?,
            votes: approvals,
            approval_power,
            total_voting_power: if total_voting_power > 0 {
                total_voting_power
            } else {
                total_validators as u64
            },
        };

        if let Err(e) = qc.verify_strict(validator_powers) {
            eprintln!("QC_VERIFY_FAILED: {}", e);
            return None;
        }
        self.qc = Some(qc.clone());
        Some(qc)
    }

    /// Finalize the round with a QC, producing a finality certificate.
    pub fn finalize(&mut self, history_root: [u8; 32]) -> Option<FinalityCertificate> {
        let qc = self.qc.as_ref()?;
        let cert = FinalityCertificate {
            height: self.height,
            block_hash: qc.block_hash,
            state_root: qc.state_root,
            history_root,
            qc: qc.clone(),
            timestamp: 0,
        };
        self.finality = Some(cert.clone());
        self.complete = true;
        Some(cert)
    }
}

/// Full consensus engine managing rounds and history.
/// State machine for node lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Bootstrapping,
    CatchingUp,
    JoiningConsensus,
    Active,
}

/// Evidence of equivocation — same validator voted for two different blocks at the same height.
#[derive(Debug, Clone)]
pub struct DoubleVoteEvidence {
    pub validator_id: [u8; 32],
    pub height: u64,
    pub vote_a: crate::messages::ConsensusVote,
    pub vote_b: crate::messages::ConsensusVote,
}
