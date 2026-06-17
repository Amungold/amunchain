use crate::misbehavior_registry::ValidatorStatus;
use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};

pub struct SlashingCertificateBuilder {
    validator_id: Option<[u8; 32]>,
    score: Option<u64>,
    evidence_ids: Option<Vec<[u8; 32]>>,
    evidence_summary: Option<Vec<EvidenceCount>>,
    penalty_bps: Option<u64>,
    amount_slashed: Option<u64>,
    remaining_stake: Option<u64>,
    offense_count: Option<u32>,
    status: Option<ValidatorStatus>,
    executed_at_height: Option<u64>,
}

impl SlashingCertificateBuilder {
    pub fn new() -> Self {
        Self {
            validator_id: None,
            score: None,
            evidence_ids: None,
            evidence_summary: None,
            penalty_bps: None,
            amount_slashed: None,
            remaining_stake: None,
            offense_count: None,
            status: None,
            executed_at_height: None,
        }
    }

    pub fn validator_id(mut self, v: [u8; 32]) -> Self {
        self.validator_id = Some(v);
        self
    }
    pub fn score(mut self, v: u64) -> Self {
        self.score = Some(v);
        self
    }
    pub fn evidence_ids(mut self, v: Vec<[u8; 32]>) -> Self {
        self.evidence_ids = Some(v);
        self
    }
    pub fn evidence_summary(mut self, v: Vec<EvidenceCount>) -> Self {
        self.evidence_summary = Some(v);
        self
    }
    pub fn penalty_bps(mut self, v: u64) -> Self {
        self.penalty_bps = Some(v);
        self
    }
    pub fn amount_slashed(mut self, v: u64) -> Self {
        self.amount_slashed = Some(v);
        self
    }
    pub fn remaining_stake(mut self, v: u64) -> Self {
        self.remaining_stake = Some(v);
        self
    }
    pub fn offense_count(mut self, v: u32) -> Self {
        self.offense_count = Some(v);
        self
    }
    pub fn status(mut self, v: ValidatorStatus) -> Self {
        self.status = Some(v);
        self
    }
    pub fn executed_at_height(mut self, v: u64) -> Self {
        self.executed_at_height = Some(v);
        self
    }

    pub fn build(self) -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            self.validator_id.unwrap(),
            self.score.unwrap(),
            self.evidence_ids.unwrap_or_default(),
            self.evidence_summary.unwrap_or_default(),
            self.penalty_bps.unwrap(),
            self.amount_slashed.unwrap(),
            self.remaining_stake.unwrap(),
            self.offense_count.unwrap_or(1),
            self.status.unwrap_or(ValidatorStatus::Active),
            self.executed_at_height.unwrap(),
        )
    }
}

impl Default for SlashingCertificateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
