use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionEvidence {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub evidence_id: u64,
    pub evidence_hash: [u8; 32],
    pub context_hash: [u8; 32],
    pub journal_entry_hash: [u8; 32],
    pub boundary_hash: [u8; 32],
    pub transition_sequence: u64,
    pub previous_transition_hash: Option<[u8; 32]>,
    pub authorizing_rule_hash: [u8; 32],
    pub pre_state_reference: Option<Vec<u8>>,
    pub post_state_reference: Option<Vec<u8>>,
}

impl ConstitutionalIdentity for TransitionEvidence {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for TransitionEvidence {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_TRANSITION_EVIDENCE);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.evidence_id)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.journal_entry_hash)
            .update_bytes(&self.boundary_hash)
            .update_u64(self.transition_sequence)
            .update_optional_hash(self.previous_transition_hash.as_ref())
            .update_bytes(&self.authorizing_rule_hash);
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0009 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.evidence_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid evidence",
            ));
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.evidence_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.evidence_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Hash mismatch",
            ));
        }
        Ok(())
    }
    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.evidence_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl TransitionEvidence {
    pub fn new(
        evidence_id: u64,
        context_hash: [u8; 32],
        journal_entry_hash: [u8; 32],
        boundary_hash: [u8; 32],
        transition_sequence: u64,
        previous_transition_hash: Option<[u8; 32]>,
        authorizing_rule_hash: [u8; 32],
    ) -> Self {
        let mut e = Self {
            schema_id: 0x0009,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            evidence_id,
            evidence_hash: [0; 32],
            context_hash,
            journal_entry_hash,
            boundary_hash,
            transition_sequence,
            previous_transition_hash,
            authorizing_rule_hash,
            pre_state_reference: None,
            post_state_reference: None,
        };
        e.evidence_hash = e.constitutional_hash();
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_verify() {
        let e = TransitionEvidence::new(1, [0xAB; 32], [0xBB; 32], [0xCC; 32], 0, None, [0xDD; 32]);
        assert!(e.verify().is_ok());
    }
}
