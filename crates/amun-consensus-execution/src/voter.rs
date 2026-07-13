use amun_consensus_messages::{ConsensusPhase, ConsensusVote};
use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;
use std::collections::{BTreeMap, HashSet};

/// Equivocation proof for a validator voting for different blocks
#[derive(Debug, Clone)]
pub struct EquivocationProof {
    pub validator_id: u64,
    pub round: u64,
    pub phase: ConsensusPhase,
    pub first_block: [u8; 32],
    pub second_block: [u8; 32],
}

/// Block-hash partitioned vote aggregator with equivocation detection
#[derive(Debug, Clone)]
pub struct VoteAggregator {
    prevotes: BTreeMap<(u64, [u8; 32]), BTreeMap<u64, ConsensusVote>>,
    precommits: BTreeMap<(u64, [u8; 32]), BTreeMap<u64, ConsensusVote>>,
    first_votes: BTreeMap<(u64, ConsensusPhase, u64), [u8; 32]>,
    evidence: Vec<EquivocationProof>,
    slashed: HashSet<u64>,
    validator_set: ValidatorSet,
}

impl VoteAggregator {
    pub fn new(validator_set: ValidatorSet) -> Self {
        Self {
            prevotes: BTreeMap::new(),
            precommits: BTreeMap::new(),
            first_votes: BTreeMap::new(),
            evidence: Vec::new(),
            slashed: HashSet::new(),
            validator_set,
        }
    }

    pub fn add_vote(&mut self, vote: ConsensusVote) -> Result<Option<QuorumCertificate>, String> {
        let round = vote.message.round;
        let vid = vote.message.validator_id;
        let phase = vote.message.phase;
        let block_hash = vote.message.block_hash.ok_or("Missing block hash")?;

        if self.slashed.contains(&vid) {
            return Ok(None);
        }

        // Equivocation check
        let eq_key = (round, phase, vid);
        if let Some(&existing_block) = self.first_votes.get(&eq_key) {
            if existing_block != block_hash {
                let proof = EquivocationProof {
                    validator_id: vid,
                    round,
                    phase,
                    first_block: existing_block,
                    second_block: block_hash,
                };
                self.evidence.push(proof);
                self.slashed.insert(vid);
                return Err(format!(
                    "Equivocation detected: validator {} voted for {:?} and {:?} in round {} phase {:?}",
                    vid, existing_block, block_hash, round, phase
                ));
            }
        } else {
            self.first_votes.insert(eq_key, block_hash);
        }

        let key = (round, block_hash);

        match phase {
            ConsensusPhase::Prevote => {
                let round_votes = self.prevotes.entry(key).or_default();
                if round_votes.contains_key(&vid) {
                    return Ok(None);
                }
                round_votes.insert(vid, vote.clone());

                let total_weight: u64 = round_votes
                    .values()
                    .filter(|v| !self.slashed.contains(&v.message.validator_id))
                    .filter_map(|v| self.validator_set.get_validator(v.message.validator_id))
                    .map(|vi| vi.stake)
                    .sum();

                if self.validator_set.has_quorum(total_weight) {
                    let votes: Vec<ConsensusVote> = round_votes.values().cloned().collect();
                    return Ok(Some(QuorumCertificate::new(
                        vote.message.position,
                        round,
                        block_hash,
                        [0u8; 32],
                        votes,
                    )));
                }
                Ok(None)
            }
            ConsensusPhase::Precommit => {
                let round_votes = self.precommits.entry(key).or_default();
                if round_votes.contains_key(&vid) {
                    return Ok(None);
                }
                round_votes.insert(vid, vote.clone());

                let total_weight: u64 = round_votes
                    .values()
                    .filter(|v| !self.slashed.contains(&v.message.validator_id))
                    .filter_map(|v| self.validator_set.get_validator(v.message.validator_id))
                    .map(|vi| vi.stake)
                    .sum();

                if self.validator_set.has_quorum(total_weight) {
                    let votes: Vec<ConsensusVote> = round_votes.values().cloned().collect();
                    return Ok(Some(QuorumCertificate::new(
                        vote.message.position,
                        round,
                        block_hash,
                        [0u8; 32],
                        votes,
                    )));
                }
                Ok(None)
            }
        }
    }

    pub fn has_voted(&self, round: u64, block_hash: &[u8; 32], validator_id: u64) -> bool {
        let key = (round, *block_hash);
        self.prevotes
            .get(&key)
            .map(|m| m.contains_key(&validator_id))
            .unwrap_or(false)
            || self
                .precommits
                .get(&key)
                .map(|m| m.contains_key(&validator_id))
                .unwrap_or(false)
    }

    pub fn leading_block(&self, round: u64) -> Option<([u8; 32], u64)> {
        let mut best: Option<([u8; 32], u64)> = None;
        for ((r, bh), votes) in &self.prevotes {
            if *r == round {
                let weight: u64 = votes
                    .values()
                    .filter(|v| !self.slashed.contains(&v.message.validator_id))
                    .filter_map(|v| self.validator_set.get_validator(v.message.validator_id))
                    .map(|vi| vi.stake)
                    .sum();
                if best.map(|(_, w)| weight > w).unwrap_or(true) {
                    best = Some((*bh, weight));
                }
            }
        }
        best
    }

    pub fn get_evidence(&self) -> &[EquivocationProof] {
        &self.evidence
    }

    pub fn is_slashed(&self, validator_id: u64) -> bool {
        self.slashed.contains(&validator_id)
    }
}
