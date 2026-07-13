use crate::constitutional_hasher::ConstitutionalHasher;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

pub mod severity {
    pub const WARNING: u8 = 0x01;
    pub const SOFT_FAILURE: u8 = 0x02;
    pub const HARD_FAILURE: u8 = 0x03;
    pub const FATAL_FAILURE: u8 = 0x04;
}
pub mod failure_domain {
    pub const STRUCTURAL: u8 = 0x01;
    pub const SEMANTIC: u8 = 0x02;
    pub const PROVENANCE: u8 = 0x03;
    pub const CONSTITUTIONAL: u8 = 0x04;
    pub const ADMISSIBILITY: u8 = 0x05;
    pub const INVARIANT: u8 = 0x06;
    pub const REPLAY: u8 = 0x07;
    pub const BOUNDARY: u8 = 0x08;
}
pub mod failure_type {
    pub const HASH_MISMATCH: u16 = 0x01;
    pub const INVALID_SCHEMA: u16 = 0x02;
    pub const MISSING_REVISION: u16 = 0x03;
    pub const ZERO_LIMIT: u16 = 0x04;
    pub const REPLAY_DIVERGENCE: u16 = 0x05;
    pub const BOUNDARY_VIOLATION: u16 = 0x06;
    pub const INVARIANT_BROKEN: u16 = 0x07;
    pub const PROVENANCE_INVALID: u16 = 0x08;
    pub const ADMISSIBILITY_DENIED: u16 = 0x09;
    pub const CROSS_CATEGORY_VIOLATION: u16 = 0x0A;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalFailure {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub failure_id: u64,
    pub failure_type: u16,
    pub failure_hash: ConstitutionalHash,
    pub failure_domain: u8,
    pub failure_severity: u8,
    pub invariant_id: Option<u64>,
    pub replay_domain: Option<u8>,
    pub transcript_position: Option<u64>,
    pub causal_parent_hash: Option<ConstitutionalHash>,
    pub execution_context_hash: Option<ConstitutionalHash>,
    pub observed_value: Option<Vec<u8>>,
    pub expected_value: Option<Vec<u8>>,
    pub description: Option<Vec<u8>>,
    pub failure_revision: u32,
    pub invariant_lineage_root: Option<ConstitutionalHash>,
    pub parent_failure_hash: Option<ConstitutionalHash>,
}

impl ConstitutionalFailure {
    pub fn new(
        failure_id: u64,
        failure_type: u16,
        failure_domain: u8,
        failure_severity: u8,
        desc: &str,
    ) -> Self {
        let mut f = Self {
            schema_id: 0x0004,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            failure_id,
            failure_type,
            failure_hash: [0; 32],
            failure_domain,
            failure_severity,
            invariant_id: None,
            replay_domain: None,
            transcript_position: None,
            causal_parent_hash: None,
            execution_context_hash: None,
            observed_value: None,
            expected_value: None,
            description: Some(desc.as_bytes().to_vec()),
            failure_revision: 1,
            invariant_lineage_root: None,
            parent_failure_hash: None,
        };
        f.failure_hash = f.constitutional_hash();
        f
    }

    pub fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_CONSTITUTIONAL_FAILURE);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.failure_id)
            .update_u16(self.failure_type)
            .update_u32(self.failure_revision)
            .update_u8(self.failure_domain)
            .update_u8(self.failure_severity)
            .update_optional_u64(self.invariant_id)
            .update_optional_u64(self.replay_domain.map(|x| x as u64))
            .update_optional_u64(self.transcript_position)
            .update_optional_hash(self.causal_parent_hash.as_ref())
            .update_optional_hash(self.execution_context_hash.as_ref())
            .update_optional_hash(self.invariant_lineage_root.as_ref())
            .update_optional_hash(self.parent_failure_hash.as_ref());
        h.finalize()
    }

    pub fn verify_structure(&self) -> Result<(), Self> {
        if self.schema_id != 0x0004 || self.schema_version == 0 {
            return Err(Self::new(
                self.failure_id + 1,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid schema",
            ));
        }
        if self.failure_domain == 0 || self.failure_severity == 0 || self.failure_type == 0 {
            return Err(Self::new(
                self.failure_id + 2,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Missing attribution",
            ));
        }
        Ok(())
    }
    pub fn verify_semantics(&self) -> Result<(), Self> {
        if self.failure_hash != self.constitutional_hash() {
            return Err(Self::new(
                self.failure_id + 3,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Hash mismatch",
            ));
        }
        Ok(())
    }
    pub fn verify_provenance(&self) -> Result<(), Self> {
        if self.constitutional_revision == 0
            || self.replay_revision == 0
            || self.failure_revision == 0
        {
            return Err(Self::new(
                self.failure_id + 4,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
    pub fn verify_admissibility_graph(&self) -> Result<(), Self> {
        if self.failure_severity == severity::FATAL_FAILURE
            && self.invariant_lineage_root.is_none()
            && self.failure_domain == failure_domain::CONSTITUTIONAL
        {
            return Err(Self::new(
                self.failure_id + 5,
                failure_type::PROVENANCE_INVALID,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Fatal failures require invariant lineage",
            ));
        }
        Ok(())
    }
    pub fn verify_constitutional(&self) -> Result<(), Self> {
        self.verify_admissibility_graph()?;
        Ok(())
    }
    pub fn verify(&self) -> Result<(), Self> {
        self.verify_structure()?;
        self.verify_semantics()?;
        self.verify_provenance()?;
        self.verify_constitutional()?;
        Ok(())
    }

    pub fn with_replay_domain(mut self, d: u8) -> Self {
        self.replay_domain = Some(d);
        self.failure_hash = self.constitutional_hash();
        self
    }
    pub fn with_transcript_position(mut self, p: u64) -> Self {
        self.transcript_position = Some(p);
        self.failure_hash = self.constitutional_hash();
        self
    }
    pub fn with_execution_context(mut self, h: ConstitutionalHash) -> Self {
        self.execution_context_hash = Some(h);
        self.failure_hash = self.constitutional_hash();
        self
    }
    pub fn with_causal_parent(mut self, h: ConstitutionalHash) -> Self {
        self.causal_parent_hash = Some(h);
        self.failure_hash = self.constitutional_hash();
        self
    }
    pub fn with_invariant(mut self, id: u64, lineage: ConstitutionalHash) -> Self {
        self.invariant_id = Some(id);
        self.invariant_lineage_root = Some(lineage);
        self.failure_hash = self.constitutional_hash();
        self
    }
    pub fn with_parent(mut self, parent_hash: ConstitutionalHash) -> Self {
        self.parent_failure_hash = Some(parent_hash);
        self.failure_hash = self.constitutional_hash();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn m() -> ConstitutionalFailure {
        ConstitutionalFailure::new(
            1,
            failure_type::HASH_MISMATCH,
            failure_domain::SEMANTIC,
            severity::HARD_FAILURE,
            "X",
        )
    }
    #[test]
    fn t_verify() {
        assert!(m().verify().is_ok());
    }
    #[test]
    fn t_hash() {
        assert_eq!(m().failure_hash, m().failure_hash);
    }
    #[test]
    fn t_desc_not_in_hash() {
        let f1 = ConstitutionalFailure::new(
            1,
            failure_type::HASH_MISMATCH,
            failure_domain::SEMANTIC,
            severity::HARD_FAILURE,
            "A",
        );
        let f2 = ConstitutionalFailure::new(
            1,
            failure_type::HASH_MISMATCH,
            failure_domain::SEMANTIC,
            severity::HARD_FAILURE,
            "B",
        );
        assert_eq!(f1.failure_hash, f2.failure_hash);
    }
}
