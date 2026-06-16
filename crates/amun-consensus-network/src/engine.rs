use crate::messages::{
    ConsensusVote, EquivocationProof, FinalityCertificate, QuorumCertificate, SignedVote,
};
use crate::misbehavior::MisbehaviorRegistry;
use crate::validator_status::ValidatorStatusRegistry;
use amun_validator_identity::ValidatorKeyRegistry;
use std::collections::HashMap;

/// A single consensus round for one block height.
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
        self.proposed_block_hash = Some(block_hash);
        self.proposed_state_root = Some(state_root);
    }

    /// Add a validator's vote. Rejects duplicate voters.
    pub fn add_vote(&mut self, vote: ConsensusVote) -> Result<(), String> {
        if self.proposed_block_hash.is_none() {
            self.proposed_block_hash = Some(vote.block_hash);
            self.proposed_state_root = Some(vote.state_root);
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
        self.votes.push(vote);
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
            "QC_CHECK: approvals={} power={} total={} quorum={}",
            approvals.len(),
            approval_power,
            total_voting_power,
            quorum_met
        );

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

        if !qc.verify() {
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

#[derive(Clone, Debug, Default)]
pub struct ConsensusMetrics {
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
}

impl ConsensusMetrics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_qc_formed(&mut self, _h: u64) {
        self.qcs_formed += 1;
    }
    pub fn record_block_finalized(&mut self, _h: u64) {
        self.blocks_finalized += 1;
    }
    pub fn record_vote(&mut self) {
        self.votes_received += 1;
    }
    pub fn summary(&self) -> String {
        format!(
            "qcs:{} final:{} votes:{}",
            self.qcs_formed, self.blocks_finalized, self.votes_received
        )
    }
}

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

pub struct ConsensusEngine {
    pub validator_id: [u8; 32],
    pub total_validators: usize,
    pub validator_ids: Vec<[u8; 32]>,
    pub current_height: u64,
    pub history_root: [u8; 32],
    pub metrics: ConsensusMetrics,
    pub rounds: HashMap<u64, ConsensusRound>,
    pub needs_catchup: bool,
    pub node_state: NodeState,
    pub validator_status: Option<std::sync::Arc<std::sync::Mutex<ValidatorStatusRegistry>>>,
    pub misbehavior_registry: MisbehaviorRegistry,
    pub validator_keys: ValidatorKeyRegistry,
    pub validator_powers: HashMap<[u8; 32], u64>,
    pub total_voting_power: u64,
    finality_chain: Vec<FinalityCertificate>,
}

impl ConsensusEngine {
    pub fn new(validator_id: [u8; 32], total_validators: usize) -> Self {
        Self {
            validator_id,
            total_validators,
            validator_ids: Vec::new(),
            current_height: 0,
            history_root: [0u8; 32],
            metrics: ConsensusMetrics::new(),
            rounds: HashMap::new(),
            needs_catchup: false,
            node_state: NodeState::Active,
            validator_status: None,
            misbehavior_registry: MisbehaviorRegistry::new(),
            validator_keys: ValidatorKeyRegistry::new(),
            validator_powers: HashMap::new(),
            total_voting_power: 0,
            finality_chain: Vec::new(),
        }
    }

    /// Start a new round for the given height.
    /// Register a validator with its derived id and public key.
    pub fn register_validator(&mut self, validator_id: [u8; 32], public_key: [u8; 32]) {
        self.validator_ids.push(validator_id);
        self.validator_keys.insert(validator_id, public_key);
    }

    /// Register a validator with both PeerId and ValidatorId (unified identity).
    pub fn register_validator_identity(
        &mut self,
        peer_id: [u8; 32],
        validator_id: [u8; 32],
        public_key: [u8; 32],
        voting_power: u64,
    ) {
        self.validator_ids.push(validator_id);
        self.validator_keys
            .register_identity(peer_id, validator_id, public_key);
        self.validator_powers.insert(validator_id, voting_power);
        self.total_voting_power += voting_power;
    }

    pub fn start_round(&mut self, height: u64, proposer_id: [u8; 32]) {
        self.rounds
            .entry(height)
            .or_insert_with(|| ConsensusRound::new(height, proposer_id));
    }

    /// Get a mutable reference to the current round.
    pub fn round_mut(&mut self, height: u64) -> Option<&mut ConsensusRound> {
        self.rounds.get_mut(&height)
    }

    /// Process a vote for a round.
    pub fn process_vote(&mut self, vote: ConsensusVote) -> Result<(), String> {
        if self.is_suspended(&vote.voter_id) {
            return Err(format!("Validator {:?} is suspended", &vote.voter_id[..4]));
        }
        // N105.3: Verify signature if registry populated
        if !self.validator_keys.is_empty() {
            let pk = self
                .validator_keys
                .get(&vote.voter_id)
                .ok_or_else(|| format!("Unknown validator {:?}", &vote.voter_id[..4]))?;
            let payload = amun_validator_identity::vote_signing_payload(
                &vote.voter_id,
                vote.height,
                &vote.block_hash,
                &vote.state_root,
                vote.approve,
                vote.timestamp,
            );
            if !amun_validator_identity::verify_ed25519(pk, &payload, &vote.signature) {
                return Err(format!(
                    "Invalid vote signature from {:?}",
                    &vote.voter_id[..4]
                ));
            }
        }
        let height = vote.height;
        if height <= self.current_height {
            eprintln!(
                "STALE_VOTE: vote_h={} current_h={} validator={:?}",
                height,
                self.current_height,
                &vote.voter_id[..4]
            );
            return Err(format!(
                "Stale vote height {} <= current {}",
                height, self.current_height
            ));
        }
        let future_window = std::cmp::max(50, self.current_height / 100);
        if height > self.current_height + future_window {
            self.needs_catchup = true;
            return Err(format!(
                "Future vote height {} > current+{} {}",
                height, future_window, self.current_height
            ));
        }
        if !self.rounds.contains_key(&height) {
            self.start_round(height, vote.voter_id);
        }
        let round = self
            .rounds
            .get_mut(&height)
            .ok_or_else(|| format!("No active round at height {}", height))?;
        eprintln!(
            "PROCESS_VOTE: validator={:?} height={} hash={:?}",
            &vote.voter_id[..4],
            vote.height,
            &vote.block_hash[..4]
        );
        self.metrics.record_vote();
        let result = round.add_vote(vote.clone());
        if let Err(ref e) = &result {
            if e.contains("Equivocation detected") {
                let existing = round
                    .votes
                    .iter()
                    .find(|v| v.voter_id == vote.voter_id)
                    .unwrap();
                let signed_a = SignedVote {
                    vote: existing.clone(),
                    signature: existing.signature,
                };
                let signed_b = SignedVote {
                    vote: vote.clone(),
                    signature: vote.signature,
                };
                let proof = EquivocationProof {
                    validator_id: vote.voter_id,
                    height: vote.height,
                    round: round.height,
                    vote_a: signed_a,
                    vote_b: signed_b,
                    detected_at_height: self.current_height,
                };
                if let Ok(hash) = self.misbehavior_registry.add_proof(proof) {
                    eprintln!(
                        "EVIDENCE_RECORDED: proof_hash={:?} validator={:?}",
                        &hash[..4],
                        &vote.voter_id[..4]
                    );
                    if crate::slashing::should_slash(&self.misbehavior_registry, &vote.voter_id) {
                        if let Some(ref registry) = self.validator_status {
                            let until = self.current_height + 100;
                            registry.lock().unwrap().set_status(
                                vote.voter_id,
                                crate::validator_status::ValidatorStatus::Suspended {
                                    until_height: until,
                                },
                            );
                            eprintln!(
                                "VALIDATOR_SLASHED: validator={:?} until={}",
                                &vote.voter_id[..4],
                                until
                            );
                        }
                    }
                }
            }
        }
        result
    }

    /// Count validators that are NOT suspended.
    pub fn active_validator_count(&self) -> usize {
        if self.validator_ids.is_empty() {
            // Fallback: use sequential IDs
            let mut active = 0;
            for i in 0..self.total_validators {
                let id = [(i + 1) as u8; 32];
                if !self.is_suspended(&id) {
                    active += 1;
                }
            }
            return active;
        }
        self.validator_ids
            .iter()
            .filter(|id| !self.is_suspended(id))
            .count()
    }

    /// Try to advance: form QC, finalize, update history.
    pub fn try_advance(
        &mut self,
        height: u64,
        history_root: [u8; 32],
    ) -> Option<FinalityCertificate> {
        let active = self.active_validator_count();
        eprintln!(
            "ADVANCE_DIAG: try_advance h={} active={}/{}",
            height, active, self.total_validators
        );
        let round = self.rounds.get_mut(&height)?;

        if round.qc.is_none() {
            round.try_form_qc(active, &self.validator_powers, self.total_voting_power)?;
        }

        let cert = round.finalize(history_root)?;
        self.current_height = height;
        self.history_root = history_root;
        self.metrics.record_qc_formed(height);
        self.metrics.record_block_finalized(height);
        self.finality_chain.push(cert.clone());
        let keep_from = height.saturating_sub(64);
        self.rounds.retain(|h, _| *h >= keep_from);
        Some(cert)
    }

    /// Get the proposer for a given height (round-robin), skipping suspended validators.
    pub fn proposer_for(&self, height: u64) -> usize {
        let base = ((height - 1) as usize) % self.total_validators;
        let mut idx = base;
        for _ in 0..self.total_validators {
            let validator_id = [(idx + 1) as u8; 32];
            if !self.is_suspended(&validator_id) {
                return idx;
            }
            idx = (idx + 1) % self.total_validators;
        }
        base
    }

    pub fn is_finalized(&self, height: u64) -> bool {
        self.rounds.get(&height).is_some_and(|r| r.complete)
    }

    // N101.6: Check if a validator is currently suspended
    fn is_suspended(&self, validator_id: &[u8; 32]) -> bool {
        if let Some(ref registry) = self.validator_status {
            let reg = registry.lock().unwrap();
            reg.is_suspended(validator_id, self.current_height)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n68_round_propose_vote_qc_finalize() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);

        let proposer = [1u8; 32];
        engine.start_round(1, proposer);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        // 3 validators approve (>2/3 of 4)
        for id in [1u8, 2, 3] {
            let vote = ConsensusVote {
                voter_id: [id; 32],
                height: 1,
                block_hash: [0xAA; 32],
                state_root: [0xBB; 32],
                approve: true,
                signature: [0u8; 64],
                timestamp: 1000,
            };
            engine.process_vote(vote).unwrap();
        }

        let cert = engine.try_advance(1, [0xCC; 32]).unwrap();
        assert_eq!(cert.height, 1);
        assert_eq!(cert.block_hash, [0xAA; 32]);
        assert_eq!(engine.current_height, 1);
        assert_eq!(engine.history_root, [0xCC; 32]);
        assert!(engine.is_finalized(1));
    }

    #[test]
    fn n68_insufficient_quorum_no_qc() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        // Only 2 approvals (50%, not >66%)
        for id in [1u8, 2] {
            engine
                .process_vote(ConsensusVote {
                    voter_id: [id; 32],
                    height: 1,
                    block_hash: [0xAA; 32],
                    state_root: [0xBB; 32],
                    approve: true,
                    signature: [0u8; 64],
                    timestamp: 1000,
                })
                .unwrap();
        }

        let qc = engine
            .round_mut(1)
            .unwrap()
            .try_form_qc(4, &HashMap::new(), 0);
        assert!(qc.is_none(), "Should not form QC with only 2/4 votes");
    }

    #[test]
    fn n68_duplicate_vote_rejected() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        let vote = ConsensusVote {
            voter_id: [1u8; 32],
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote.clone()).is_ok());
        assert!(
            engine.process_vote(vote).is_err(),
            "Duplicate vote must be rejected"
        );
    }

    #[test]
    fn n68_byzantine_wrong_height_rejected() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);

        let vote = ConsensusVote {
            voter_id: [2u8; 32],
            height: 99, // Wrong height
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote).is_err());
    }

    #[test]
    fn n68_multi_round_consensus() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);

        for height in 1..=3 {
            let proposer_idx = engine.proposer_for(height);
            let proposer = [(proposer_idx + 1) as u8; 32];
            engine.start_round(height, proposer);
            engine
                .round_mut(height)
                .unwrap()
                .propose([height as u8; 32], [0xBB; 32]);

            for id in 1..=3 {
                engine
                    .process_vote(ConsensusVote {
                        voter_id: [id; 32],
                        height,
                        block_hash: [height as u8; 32],
                        state_root: [0xBB; 32],
                        approve: true,
                        signature: [0u8; 64],
                        timestamp: 1000,
                    })
                    .unwrap();
            }

            let history = [height as u8; 32];
            let cert = engine.try_advance(height, history).unwrap();
            assert_eq!(cert.height, height);
            assert!(engine.is_finalized(height));
        }

        assert_eq!(engine.current_height, 3);
        assert_eq!(engine.history_root, [3u8; 32]);
    }

    // N105.3 test: signature enforced when registry populated
    #[test]
    fn n105_signature_required_when_registry_populated() {
        use ed25519_dalek::{Signer, SigningKey};
        use rand::rngs::OsRng;
        let mut engine = ConsensusEngine::new([0u8; 32], 4);

        // Generate a keypair, derive validator id
        let sk = SigningKey::generate(&mut OsRng);
        let pk = sk.verifying_key().to_bytes();
        let vid = amun_validator_identity::derive_validator_id(&pk);
        // Register this validator
        engine.register_validator(vid, pk);
        engine.start_round(1, vid);

        // Create a validly signed vote
        let payload_ok = amun_validator_identity::vote_signing_payload(
            &vid,
            1,
            &[0xAA; 32],
            &[0xBB; 32],
            true,
            1000,
        );
        let sig_ok = sk.sign(&payload_ok).to_bytes();
        let vote_ok = ConsensusVote {
            voter_id: vid,
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: sig_ok,
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote_ok).is_ok());

        // Unsigned vote (all zeros signature) must be rejected
        let vote_bad = ConsensusVote {
            voter_id: vid,
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote_bad).is_err());
    }
}
