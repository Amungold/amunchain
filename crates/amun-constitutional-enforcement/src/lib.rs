pub mod constitutional_evidence;
pub mod evidence_providers;
pub mod evidence_records;
pub mod proof_engine;
pub mod state_transition;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstitutionalVerdict {
    Constitutional,
    Unconstitutional {
        violations: Vec<ConstitutionalViolation>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstitutionalViolation {
    pub law: ConstitutionalLaw,
    pub description: String,
    pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConstitutionalLaw {
    StateRootIntegrity,
    ChainContinuity,
    SignatureValidity,
    NoDoubleSpend,
    SlashingEvidenceBinding,
    ValidatorSetGovernance,
    ReplayDeterminism,
    FinalitySupermajority,
    StateTransitionValidity,
    EvidenceValidity,
}

#[derive(Debug, Default)]
pub struct ConstitutionalEnforcementKernel {
    pub active_laws: Vec<ConstitutionalLaw>,
    pub verdict_history: Vec<ConstitutionalVerdict>,
    pub constitutional_count: u64,
    pub unconstitutional_count: u64,
}

impl ConstitutionalEnforcementKernel {
    /// N127A: Review a block using structured constitutional evidence.
    /// This is the preferred interface — all evidence is explicitly typed.
    pub fn review_block_with_evidence(
        &mut self,
        height: u64,
        evidence: &crate::constitutional_evidence::ConstitutionalEvidence,
    ) -> ConstitutionalVerdict {
        self.review_block(
            height,
            evidence.state_root_valid,
            evidence.chain_continuous,
            evidence.signatures_valid,
            evidence.no_double_spend,
            evidence.slashing_bound,
            evidence.governance_valid,
            evidence.replay_deterministic,
            evidence.finality_supermajority,
            evidence.transition_valid,
            evidence.evidence_valid,
        )
    }

    /// Legacy interface — still supported, delegates to review_block_with_evidence.
    pub fn new() -> Self {
        Self {
            active_laws: vec![
                ConstitutionalLaw::StateRootIntegrity,
                ConstitutionalLaw::ChainContinuity,
                ConstitutionalLaw::SignatureValidity,
                ConstitutionalLaw::NoDoubleSpend,
                ConstitutionalLaw::SlashingEvidenceBinding,
                ConstitutionalLaw::ValidatorSetGovernance,
                ConstitutionalLaw::ReplayDeterminism,
                ConstitutionalLaw::FinalitySupermajority,
                ConstitutionalLaw::StateTransitionValidity,
                ConstitutionalLaw::EvidenceValidity,
            ],
            verdict_history: Vec::new(),
            constitutional_count: 0,
            unconstitutional_count: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn review_block(
        &mut self,
        height: u64,
        state_root_valid: bool,
        chain_continuous: bool,
        signatures_valid: bool,
        no_double_spend: bool,
        slashing_bound: bool,
        governance_valid: bool,
        replay_deterministic: bool,
        finality_supermajority: bool,
        transition_valid: bool,
        evidence_valid: bool,
    ) -> ConstitutionalVerdict {
        let mut violations = Vec::new();
        let checks: Vec<(&ConstitutionalLaw, bool, &str)> = vec![
            (
                &ConstitutionalLaw::StateRootIntegrity,
                state_root_valid,
                "State root mismatch",
            ),
            (
                &ConstitutionalLaw::ChainContinuity,
                chain_continuous,
                "Chain discontinuity",
            ),
            (
                &ConstitutionalLaw::SignatureValidity,
                signatures_valid,
                "Invalid signatures",
            ),
            (
                &ConstitutionalLaw::NoDoubleSpend,
                no_double_spend,
                "Double spend detected",
            ),
            (
                &ConstitutionalLaw::SlashingEvidenceBinding,
                slashing_bound,
                "Slashing lacks evidence",
            ),
            (
                &ConstitutionalLaw::ValidatorSetGovernance,
                governance_valid,
                "Governance violation",
            ),
            (
                &ConstitutionalLaw::ReplayDeterminism,
                replay_deterministic,
                "Replay divergence",
            ),
            (
                &ConstitutionalLaw::FinalitySupermajority,
                finality_supermajority,
                "No supermajority",
            ),
            (
                &ConstitutionalLaw::StateTransitionValidity,
                transition_valid,
                "Invalid transition",
            ),
            (
                &ConstitutionalLaw::EvidenceValidity,
                evidence_valid,
                "Invalid evidence",
            ),
        ];
        for (law, passed, description) in &checks {
            if self.active_laws.contains(law) && !passed {
                violations.push(ConstitutionalViolation {
                    law: (*law).clone(),
                    description: description.to_string(),
                    height,
                });
            }
        }
        let verdict = if violations.is_empty() {
            self.constitutional_count += 1;
            ConstitutionalVerdict::Constitutional
        } else {
            self.unconstitutional_count += 1;
            ConstitutionalVerdict::Unconstitutional { violations }
        };
        self.verdict_history.push(verdict.clone());
        verdict
    }

    pub fn is_law_active(&self, law: &ConstitutionalLaw) -> bool {
        self.active_laws.contains(law)
    }
    pub fn activate_law(&mut self, law: ConstitutionalLaw) {
        if !self.active_laws.contains(&law) {
            self.active_laws.push(law);
        }
    }
    pub fn deactivate_law(&mut self, law: &ConstitutionalLaw) {
        self.active_laws.retain(|l| l != law);
    }
    pub fn compliance_ratio(&self) -> f64 {
        let total = self.constitutional_count + self.unconstitutional_count;
        if total == 0 {
            1.0
        } else {
            self.constitutional_count as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n123_constitutional_block_accepted() {
        let mut k = ConstitutionalEnforcementKernel::new();
        assert_eq!(
            k.review_block(100, true, true, true, true, true, true, true, true, true, true),
            ConstitutionalVerdict::Constitutional
        );
        assert_eq!(k.constitutional_count, 1);
    }

    #[test]
    fn n123_unconstitutional_block_rejected() {
        let mut k = ConstitutionalEnforcementKernel::new();
        match k.review_block(
            100, false, true, true, true, true, true, true, true, true, true,
        ) {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert_eq!(violations.len(), 1);
                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
            }
            _ => panic!("Expected Unconstitutional"),
        }
    }

    #[test]
    fn n123_multiple_violations() {
        let mut k = ConstitutionalEnforcementKernel::new();
        match k.review_block(
            200, false, false, true, true, true, true, true, true, true, false,
        ) {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert_eq!(violations.len(), 3)
            }
            _ => panic!("Expected multiple"),
        }
    }

    #[test]
    fn n123_deactivated_law() {
        let mut k = ConstitutionalEnforcementKernel::new();
        k.deactivate_law(&ConstitutionalLaw::StateRootIntegrity);
        assert_eq!(
            k.review_block(100, false, true, true, true, true, true, true, true, true, true),
            ConstitutionalVerdict::Constitutional
        );
    }

    #[test]
    fn n123_compliance_ratio() {
        let mut k = ConstitutionalEnforcementKernel::new();
        k.review_block(
            1, true, true, true, true, true, true, true, true, true, true,
        );
        k.review_block(
            2, true, true, true, true, true, true, true, true, true, true,
        );
        k.review_block(
            3, true, true, true, true, true, true, true, true, true, true,
        );
        k.review_block(
            4, false, true, true, true, true, true, true, true, true, true,
        );
        assert_eq!(k.compliance_ratio(), 0.75);
    }

    #[test]
    fn n123_verdict_history() {
        let mut k = ConstitutionalEnforcementKernel::new();
        k.review_block(
            1, true, true, true, true, true, true, true, true, true, true,
        );
        k.review_block(
            2, false, true, true, true, true, true, true, true, true, true,
        );
        assert_eq!(k.verdict_history.len(), 2);
    }
}
