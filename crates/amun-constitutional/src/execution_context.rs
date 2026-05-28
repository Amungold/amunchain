use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionContext {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub context_id: u64,
    pub context_hash: [u8; 32],
    pub governance_domain_hash: [u8; 32],
    pub replay_lineage_hash: Option<[u8; 32]>,
    pub parent_context_hash: Option<[u8; 32]>,
    pub boundary_hash: Option<[u8; 32]>,
    pub sequence_in_lineage: u64,
}

impl ConstitutionalIdentity for ExecutionContext {
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

impl ConstitutionalObject for ExecutionContext {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_EXECUTION_CONTEXT);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.context_id)
            .update_bytes(&self.governance_domain_hash)
            .update_optional_hash(self.replay_lineage_hash.as_ref())
            .update_optional_hash(self.parent_context_hash.as_ref())
            .update_optional_hash(self.boundary_hash.as_ref())
            .update_u64(self.sequence_in_lineage);
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0006 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.context_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid context",
            ));
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.context_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.context_id,
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
                self.context_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl ExecutionContext {
    pub fn new(
        context_id: u64,
        governance_domain_hash: [u8; 32],
        sequence_in_lineage: u64,
    ) -> Self {
        let mut ctx = Self {
            schema_id: 0x0006,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            context_id,
            context_hash: [0; 32],
            governance_domain_hash,
            replay_lineage_hash: None,
            parent_context_hash: None,
            boundary_hash: None,
            sequence_in_lineage,
        };
        ctx.context_hash = ctx.constitutional_hash();
        ctx
    }
    pub fn with_replay_lineage(mut self, h: [u8; 32]) -> Self {
        self.replay_lineage_hash = Some(h);
        self.context_hash = self.constitutional_hash();
        self
    }
    pub fn with_parent_context(mut self, h: [u8; 32]) -> Self {
        self.parent_context_hash = Some(h);
        self.context_hash = self.constitutional_hash();
        self
    }
    pub fn with_boundary(mut self, h: [u8; 32]) -> Self {
        self.boundary_hash = Some(h);
        self.context_hash = self.constitutional_hash();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_verify() {
        assert!(ExecutionContext::new(1, [0xAB; 32], 0).verify().is_ok());
    }
    #[test]
    fn t_hash() {
        assert_eq!(
            ExecutionContext::new(1, [0xAB; 32], 0).context_hash,
            ExecutionContext::new(1, [0xAB; 32], 0).context_hash
        );
    }
}
