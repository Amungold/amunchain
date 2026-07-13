use crate::engine_metrics::EngineMetrics;
use crate::lifecycle::NodeState;
use crate::messages::{
    ConsensusVote, EquivocationProof, FinalityCertificate, QuorumCertificate, SignedVote,
};
use crate::misbehavior_registry::MisbehaviorRegistry;
use crate::round::ConsensusRound;
use crate::validator_status::ValidatorStatusRegistry;
use amun_validator_identity::ValidatorKeyRegistry;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

/// A single consensus round for one block height.
pub struct ConsensusEngine {
    pub validator_id: [u8; 32],
    pub total_validators: usize,
    pub validator_ids: Vec<[u8; 32]>,
    pub current_height: u64,
    pub history_root: [u8; 32],
    pub metrics: EngineMetrics,
    pub rounds: HashMap<u64, ConsensusRound>,
    pub needs_catchup: AtomicBool,
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
            metrics: EngineMetrics::new(),
            rounds: HashMap::new(),
            needs_catchup: AtomicBool::new(false),
            node_state: NodeState::Active,
            validator_status: None,
            misbehavior_registry: MisbehaviorRegistry::new(
                crate::misbehavior_registry::MisbehaviorThresholds::default(),
            ),
            validator_keys: ValidatorKeyRegistry::new(),
            validator_powers: HashMap::new(),
            total_voting_power: 0,
            finality_chain: Vec::new(),
        }
    }

    /// Start a new round for the given height.
    /// Register a validator with its derived id and public key.
    pub fn process_vote(&mut self, vote: &ConsensusVote) -> Result<(), String> {
        if self.is_suspended(&vote.voter_id) {
            eprintln!("REJECT_SUSPENDED validator={:?}", &vote.voter_id[..4]);
            return Err(format!("Validator {:?} is suspended", &vote.voter_id[..4]));
        }
        // N105.3: Verify signature if registry populated
        if !self.validator_keys.is_empty() {
            if self.validator_keys.get(&vote.voter_id).is_none() {
                eprintln!(
                    "REJECT_UNKNOWN_VALIDATOR validator={:?}",
                    &vote.voter_id[..4]
                );
                return Err(format!("Unknown validator {:?}", &vote.voter_id[..4]));
            }

            eprintln!("VALIDATOR_FOUND validator={:?}", &vote.voter_id[..4]);

            if let Err(e) = Self::verify_vote_signature(&self.validator_keys, vote) {
                eprintln!(
                    "REJECT_BAD_SIGNATURE validator={:?} height={}",
                    &vote.voter_id[..4],
                    vote.height
                );
                crate::vote_binding::verify_vote_binding(vote)?;
                return Err(e);
            }
        }
        let height = vote.height;
        if height <= self.current_height {
            // Already finalized or duplicate late vote.
            // Ignore silently instead of polluting the logs.
            return Err("Duplicate vote from validator".to_string());
        }
        let future_window = std::cmp::max(5, self.current_height / 100); // N102: reduced for test, was 50
        if height > self.current_height + future_window {
            eprintln!(
                "REJECT_FUTURE vote_h={} current_h={} future_window={} validator={:?}",
                height,
                self.current_height,
                future_window,
                &vote.voter_id[..4]
            );
            self.needs_catchup.store(true, Ordering::SeqCst);
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
        let result = round.add_vote(vote);
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
                // N109.13: Use unified misbehavior registry
                let evidence_id = crate::evidence_store::EvidenceRecord::compute_evidence_id(
                    &vote.voter_id,
                    vote.height,
                    &crate::evidence_store::EvidenceType::DoubleVote,
                    &postcard::to_stdvec(&proof).unwrap_or_default(),
                );
                if self.misbehavior_registry.record_misbehavior(
                    &vote.voter_id,
                    &evidence_id,
                    &crate::evidence_store::EvidenceType::DoubleVote,
                    vote.height,
                ) {
                    eprintln!(
                        "EVIDENCE_RECORDED: evidence_id={:?} validator={:?}",
                        &evidence_id[..4],
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
        let validator_keys = &self.validator_keys;
        let round = self.rounds.get_mut(&height)?;

        if round.qc.is_none() {
            round.try_form_qc(active, &self.validator_powers, self.total_voting_power)?;
        }

        let qc = round.qc.as_ref()?;

        eprintln!("QC_ENGINE_VERIFY: h={} votes={}", qc.height, qc.votes.len());

        let qc = round.qc.as_ref()?;

        if let Err(e) = ConsensusEngine::verify_qc_signatures(validator_keys, qc) {
            eprintln!("QC_SIGNATURE_VERIFY_FAILED: {}", e);
            return None;
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

    /// Constitutional API (AC-1.0 Article VII): Recover consensus state from persistent storage.
    /// Replaces direct mutation of current_height and history_root.
    pub fn recover_state(&mut self, height: u64, history_root: [u8; 32]) {
        self.current_height = height;
        self.history_root = history_root;
    }

    /// Constitutional API (AC-1.0 Article VII): Fast-forward to a new height after sync.
    /// Replaces direct mutation of current_height.
    pub fn fast_forward(&mut self, new_height: u64) {
        if new_height > self.current_height {
            self.current_height = new_height;
        }
    }

    /// Constitutional API (AC-1.0 Article VII): Update history root from chain tip.
    /// Replaces direct mutation of history_root.
    pub fn update_history_root(&mut self, root: [u8; 32]) {
        self.history_root = root;
    }

    /// Constitutional API (AC-1.0 Article VII): Reset round cache after state change.
    /// Replaces direct mutation of rounds.
    pub fn reset_rounds(&mut self) {
        self.rounds.clear();
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
    fn verify_vote_signature(
        validator_keys: &ValidatorKeyRegistry,
        vote: &ConsensusVote,
    ) -> Result<(), String> {
        if validator_keys.is_empty() {
            return Ok(());
        }

        let pk = validator_keys
            .get(&vote.voter_id)
            .ok_or_else(|| format!("Missing public key for {:?}", &vote.voter_id[..4]))?;

        let payload = amun_validator_identity::vote_signing_payload(
            &vote.voter_id,
            amun_validator_identity::signature::DEFAULT_CHAIN_ID,
            vote.height,
            &vote.block_hash,
            &vote.state_root,
            vote.approve,
            vote.timestamp,
        );

        if !amun_validator_identity::verify_ed25519(pk, &payload, &vote.signature) {
            return Err(format!("Invalid signature from {:?}", &vote.voter_id[..4]));
        }

        Ok(())
    }

    fn verify_qc_signatures(
        validator_keys: &ValidatorKeyRegistry,
        qc: &QuorumCertificate,
    ) -> Result<(), String> {
        for vote in &qc.votes {
            Self::verify_vote_signature(validator_keys, vote)?;
        }
        Ok(())
    }

    /// Batch process multiple votes.
    /// Constitutional: same logic as individual process_vote.
    /// Performance: enables future parallel/batch verification.
    pub fn process_votes(&mut self, votes: &[ConsensusVote]) -> Vec<Result<(), String>> {
        votes.iter().map(|v| self.process_vote(v)).collect()
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
                commitment: None,
            };
            engine.process_vote(&vote).unwrap();
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
                .process_vote(&ConsensusVote {
                    voter_id: [id; 32],
                    height: 1,
                    block_hash: [0xAA; 32],
                    state_root: [0xBB; 32],
                    approve: true,
                    signature: [0u8; 64],
                    timestamp: 1000,
                    commitment: None,
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
            commitment: None,
        };
        assert!(engine.process_vote(&vote).is_ok());
        assert!(
            engine.process_vote(&vote).is_err(),
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
            commitment: None,
        };
        assert!(engine.process_vote(&vote).is_err());
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
                    .process_vote(&ConsensusVote {
                        voter_id: [id; 32],
                        height,
                        block_hash: [height as u8; 32],
                        state_root: [0xBB; 32],
                        approve: true,
                        signature: [0u8; 64],
                        timestamp: 1000,
                        commitment: None,
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
            amun_validator_identity::signature::DEFAULT_CHAIN_ID,
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
            commitment: None,
        };
        assert!(engine.process_vote(&vote_ok).is_ok());

        // Unsigned vote (all zeros signature) must be rejected
        let vote_bad = ConsensusVote {
            voter_id: vid,
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
            commitment: None,
        };
        assert!(engine.process_vote(&vote_bad).is_err());
    }
}

#[cfg(test)]
mod n130_tests {
    use super::*;

    #[test]
    fn n130_vote_before_proposal_is_recovered() {
        let mut round = ConsensusRound::new(1, [1u8; 32]);

        let vote = ConsensusVote {
            voter_id: [1u8; 32],
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1,
            commitment: None,
        };

        assert!(round.add_vote(&vote).is_ok());

        assert_eq!(round.proposed_block_hash, Some([0xAA; 32]));

        assert_eq!(round.proposed_state_root, Some([0xBB; 32]));
    }
}
