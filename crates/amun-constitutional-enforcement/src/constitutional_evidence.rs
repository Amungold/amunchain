// ============================================================================
// N127A — Constitutional Evidence Interface
// ============================================================================
// Replaces individual bool parameters with structured evidence types.
// Each evidence type has a documented source in the AmunChain architecture.

use serde::{Deserialize, Serialize};

/// N127A.1: Unified constitutional evidence passed to the enforcement kernel.
/// Every field has a documented architectural source (see N127A.2-N127A.5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalEvidence {
    // N127A.2: From ExecutionEngine
    pub state_root_valid: bool,
    pub signatures_valid: bool,
    pub no_double_spend: bool,
    pub transition_valid: bool,

    // N127A.3: From ReplayVerifier (N109.7 architectural guarantee)
    pub replay_deterministic: bool,

    // N127A.4: From AuthorityRegistry
    pub governance_valid: bool,

    // N127A.5: From QuorumCertificate
    pub chain_continuous: bool,
    pub finality_supermajority: bool,

    // From EvidenceStore (slashing certificates)
    pub slashing_bound: bool,
    pub evidence_valid: bool,
}

impl ConstitutionalEvidence {
    /// N127A.1: Create evidence with all fields explicitly set.
    /// No defaults — every field must be consciously provided.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
    ) -> Self {
        Self {
            state_root_valid,
            chain_continuous,
            signatures_valid,
            no_double_spend,
            slashing_bound,
            governance_valid,
            replay_deterministic,
            finality_supermajority,
            transition_valid,
            evidence_valid,
        }
    }

    /// N127A.2: Build from execution engine results.
    pub fn from_execution(
        state_root_valid: bool,
        signatures_valid: bool,
        no_double_spend: bool,
        transition_valid: bool,
    ) -> ExecutionEvidence {
        ExecutionEvidence {
            state_root_valid,
            signatures_valid,
            no_double_spend,
            transition_valid,
        }
    }

    /// N127A.3: Build from replay verification.
    pub fn from_replay(replay_deterministic: bool) -> ReplayEvidence {
        ReplayEvidence {
            replay_deterministic,
        }
    }

    /// N127A.4: Build from governance/authority registry.
    pub fn from_governance(governance_valid: bool) -> GovernanceEvidence {
        GovernanceEvidence { governance_valid }
    }

    /// N127A.5: Build from QC verification.
    pub fn from_qc(chain_continuous: bool, finality_supermajority: bool) -> QcEvidence {
        QcEvidence {
            chain_continuous,
            finality_supermajority,
        }
    }
}

// ============================================================================
// N127A.2 — Execution Evidence Adapter
// ============================================================================
/// Evidence sourced from the ExecutionEngine (N109.7, N109.8).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEvidence {
    pub state_root_valid: bool,
    pub signatures_valid: bool,
    pub no_double_spend: bool,
    pub transition_valid: bool,
}

// ============================================================================
// N127A.3 — Replay Evidence Adapter
// ============================================================================
/// Evidence sourced from ReplayVerifier or N109.7 architectural guarantee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvidence {
    pub replay_deterministic: bool,
}

// ============================================================================
// N127A.4 — Governance Evidence Adapter
// ============================================================================
/// Evidence sourced from AuthorityRegistry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvidence {
    pub governance_valid: bool,
}

// ============================================================================
// N127A.5 — QC Evidence Adapter
// ============================================================================
/// Evidence sourced from QuorumCertificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcEvidence {
    pub chain_continuous: bool,
    pub finality_supermajority: bool,
}

// ============================================================================
// N127A.1 — Constitutional Evidence Builder
// ============================================================================
/// Assembles evidence from all adapters into a single ConstitutionalEvidence.
pub struct ConstitutionalEvidenceBuilder;

impl ConstitutionalEvidenceBuilder {
    pub fn build(
        execution: ExecutionEvidence,
        replay: ReplayEvidence,
        governance: GovernanceEvidence,
        qc: QcEvidence,
        slashing_bound: bool,
        evidence_valid: bool,
    ) -> ConstitutionalEvidence {
        ConstitutionalEvidence {
            state_root_valid: execution.state_root_valid,
            signatures_valid: execution.signatures_valid,
            no_double_spend: execution.no_double_spend,
            transition_valid: execution.transition_valid,
            replay_deterministic: replay.replay_deterministic,
            governance_valid: governance.governance_valid,
            chain_continuous: qc.chain_continuous,
            finality_supermajority: qc.finality_supermajority,
            slashing_bound,
            evidence_valid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n127a_1_evidence_struct_creation() {
        let evidence =
            ConstitutionalEvidence::new(true, true, true, true, true, true, true, true, true, true);
        assert!(evidence.state_root_valid);
        assert!(evidence.finality_supermajority);
    }

    #[test]
    fn n127a_1_builder_assembles_all_evidence() {
        let exec = ExecutionEvidence {
            state_root_valid: true,
            signatures_valid: true,
            no_double_spend: true,
            transition_valid: true,
        };
        let replay = ReplayEvidence {
            replay_deterministic: true,
        };
        let gov = GovernanceEvidence {
            governance_valid: true,
        };
        let qc = QcEvidence {
            chain_continuous: true,
            finality_supermajority: true,
        };

        let evidence = ConstitutionalEvidenceBuilder::build(exec, replay, gov, qc, true, true);

        assert!(evidence.state_root_valid);
        assert!(evidence.replay_deterministic);
        assert!(evidence.governance_valid);
        assert!(evidence.finality_supermajority);
        assert!(evidence.slashing_bound);
        assert!(evidence.evidence_valid);
    }

    #[test]
    fn n127a_1_evidence_defaults_are_explicit() {
        // All fields must be explicitly set — no Default derive
        let evidence = ConstitutionalEvidence::new(
            true, false, true, true, false, true, true, false, true, true,
        );
        assert!(!evidence.chain_continuous);
        assert!(!evidence.finality_supermajority);
        assert!(evidence.state_root_valid);
    }
}
