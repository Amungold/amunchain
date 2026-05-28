use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReplayLimits {
    pub max_transcript_span: u64,
    pub max_replay_divergence: u64,
    pub max_journal_entries: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdmissibilityLimits {
    pub max_events_per_context: u64,
    pub max_state_reads: u64,
    pub max_state_writes: u64,
    pub max_transition_depth: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProofLimits {
    pub max_proof_chain_depth: u64,
    pub max_receipt_size: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InvariantLimits {
    pub max_invariant_failures: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtifactLimits {
    pub max_constitutional_artifact_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLimits {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub limits_id: u64,
    pub limits_hash: [u8; 32],
    pub governance_origin_hash: Option<[u8; 32]>,
    pub activation_epoch: u64,
    pub supersedes_limits_hash: Option<[u8; 32]>,
    pub replay: ReplayLimits,
    pub admissibility: AdmissibilityLimits,
    pub proof: ProofLimits,
    pub invariant: InvariantLimits,
    pub artifact: ArtifactLimits,
}

impl ExecutionLimits {
    pub fn constitutional_default() -> Self {
        let mut l = Self {
            schema_id: 0x0005,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            limits_id: 1,
            limits_hash: [0; 32],
            governance_origin_hash: None,
            activation_epoch: 1,
            supersedes_limits_hash: None,
            replay: ReplayLimits {
                max_transcript_span: 1_000_000,
                max_replay_divergence: 1000,
                max_journal_entries: 100_000,
            },
            admissibility: AdmissibilityLimits {
                max_events_per_context: 10_000,
                max_state_reads: 100_000,
                max_state_writes: 100_000,
                max_transition_depth: 1000,
            },
            proof: ProofLimits {
                max_proof_chain_depth: 50,
                max_receipt_size: 1_048_576,
            },
            invariant: InvariantLimits {
                max_invariant_failures: 100,
            },
            artifact: ArtifactLimits {
                max_constitutional_artifact_size: 16_777_216,
            },
        };
        l.limits_hash = l.constitutional_hash();
        l
    }

    pub fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_EXECUTION_LIMITS);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.limits_id)
            .update_optional_hash(self.governance_origin_hash.as_ref())
            .update_u64(self.activation_epoch)
            .update_optional_hash(self.supersedes_limits_hash.as_ref());
        h.update_u64(self.replay.max_transcript_span)
            .update_u64(self.replay.max_replay_divergence)
            .update_u64(self.replay.max_journal_entries);
        h.update_u64(self.admissibility.max_events_per_context)
            .update_u64(self.admissibility.max_state_reads)
            .update_u64(self.admissibility.max_state_writes)
            .update_u64(self.admissibility.max_transition_depth);
        h.update_u64(self.proof.max_proof_chain_depth)
            .update_u64(self.proof.max_receipt_size);
        h.update_u64(self.invariant.max_invariant_failures);
        h.update_u64(self.artifact.max_constitutional_artifact_size);
        h.finalize()
    }

    fn e(id: u64, ty: u16, dom: u8, sev: u8, desc: &str) -> ConstitutionalFailure {
        ConstitutionalFailure::new(id, ty, dom, sev, desc)
    }
    pub fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0005 || self.schema_version == 0 {
            return Err(Self::e(
                100,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid schema",
            ));
        }
        Ok(())
    }
    pub fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.limits_hash != self.constitutional_hash() {
            return Err(Self::e(
                101,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Hash mismatch",
            ));
        }
        Ok(())
    }
    pub fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(Self::e(
                102,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
    pub fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        if self.proof.max_proof_chain_depth > self.admissibility.max_transition_depth {
            return Err(Self::e(
                103,
                failure_type::CROSS_CATEGORY_VIOLATION,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Proof depth exceeds transition depth",
            ));
        }
        if self.proof.max_receipt_size > self.artifact.max_constitutional_artifact_size {
            return Err(Self::e(
                104,
                failure_type::CROSS_CATEGORY_VIOLATION,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Receipt size exceeds artifact size",
            ));
        }
        if self.replay.max_transcript_span == 0 || self.admissibility.max_events_per_context == 0 {
            return Err(Self::e(
                105,
                failure_type::ZERO_LIMIT,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Zero limits inadmissible",
            ));
        }
        Ok(())
    }
    pub fn verify(&self) -> Result<(), ConstitutionalFailure> {
        self.verify_structure()?;
        self.verify_semantics()?;
        self.verify_provenance()?;
        self.verify_constitutional()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_default_verify() {
        assert!(ExecutionLimits::constitutional_default().verify().is_ok());
    }
    #[test]
    fn t_hash_det() {
        assert_eq!(
            ExecutionLimits::constitutional_default().limits_hash,
            ExecutionLimits::constitutional_default().limits_hash
        );
    }
}
