use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionCommitment {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub commitment_id: u64,
    pub commitment_hash: [u8; 32],
    pub evidence_hash: [u8; 32],
    pub context_hash: [u8; 32],
    pub journal_entry_hash: [u8; 32],
    pub previous_commitment_hash: Option<[u8; 32]>,
    pub commitment_sequence: u64,
    pub boundary_hash: [u8; 32],
}

impl ConstitutionalIdentity for TransitionCommitment {
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

impl ConstitutionalObject for TransitionCommitment {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_TRANSITION_COMMITMENT);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.commitment_id)
            .update_bytes(&self.evidence_hash)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.journal_entry_hash)
            .update_optional_hash(self.previous_commitment_hash.as_ref())
            .update_u64(self.commitment_sequence)
            .update_bytes(&self.boundary_hash);
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000A || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.commitment_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid commitment",
            ));
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.commitment_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.commitment_id,
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
                self.commitment_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl TransitionCommitment {
    pub fn new(
        commitment_id: u64,
        evidence_hash: [u8; 32],
        context_hash: [u8; 32],
        journal_entry_hash: [u8; 32],
        boundary_hash: [u8; 32],
        commitment_sequence: u64,
        previous_commitment_hash: Option<[u8; 32]>,
    ) -> Self {
        let mut c = Self {
            schema_id: 0x000A,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            commitment_id,
            commitment_hash: [0; 32],
            evidence_hash,
            context_hash,
            journal_entry_hash,
            previous_commitment_hash,
            commitment_sequence,
            boundary_hash,
        };
        c.commitment_hash = c.constitutional_hash();
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_verify() {
        let c =
            TransitionCommitment::new(1, [0xAA; 32], [0xBB; 32], [0xCC; 32], [0xDD; 32], 0, None);
        assert!(c.verify().is_ok());
    }
}
