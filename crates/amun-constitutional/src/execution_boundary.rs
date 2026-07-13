use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::execution_limits::ExecutionLimits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionBoundary {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub boundary_id: u64,
    pub boundary_hash: [u8; 32],
    pub limits_hash: [u8; 32],
    pub context_hash: [u8; 32],
    pub governance_domain_hash: [u8; 32],
    pub max_events: u64,
    pub max_transitions: u64,
    pub cross_domain_allowed: bool,
    pub transcript_span_at_creation: u64,
    pub parent_boundary_hash: Option<[u8; 32]>,
    pub boundary_sequence: u64,
    pub activation_transcript_position: u64,
    pub expiration_transcript_position: Option<u64>,
}

impl ConstitutionalIdentity for ExecutionBoundary {
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

impl ConstitutionalObject for ExecutionBoundary {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_EXECUTION_BOUNDARY);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.boundary_id)
            .update_bytes(&self.limits_hash)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.governance_domain_hash)
            .update_u64(self.max_events)
            .update_u64(self.max_transitions)
            .update_u8(self.cross_domain_allowed as u8)
            .update_u64(self.transcript_span_at_creation)
            .update_optional_hash(self.parent_boundary_hash.as_ref())
            .update_u64(self.boundary_sequence)
            .update_u64(self.activation_transcript_position)
            .update_optional_u64(self.expiration_transcript_position);
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0007 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.boundary_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid boundary",
            ));
        }
        if self.max_events == 0 || self.max_transitions == 0 {
            return Err(ConstitutionalFailure::new(
                self.boundary_id,
                failure_type::ZERO_LIMIT,
                failure_domain::BOUNDARY,
                severity::FATAL_FAILURE,
                "Zero limits",
            ));
        }
        if let Some(exp) = self.expiration_transcript_position {
            if exp <= self.activation_transcript_position {
                return Err(ConstitutionalFailure::new(
                    self.boundary_id,
                    failure_type::BOUNDARY_VIOLATION,
                    failure_domain::BOUNDARY,
                    severity::FATAL_FAILURE,
                    "Exp before act",
                ));
            }
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.boundary_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.boundary_id,
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
                self.boundary_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl ExecutionBoundary {
    pub fn new(
        boundary_id: u64,
        governance_domain_hash: [u8; 32],
        limits: &ExecutionLimits,
        context_hash: [u8; 32],
        transcript_span: u64,
        boundary_sequence: u64,
        activation_position: u64,
    ) -> Self {
        let mut b = Self {
            schema_id: 0x0007,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            boundary_id,
            boundary_hash: [0; 32],
            limits_hash: limits.limits_hash,
            context_hash,
            governance_domain_hash,
            max_events: limits.admissibility.max_events_per_context,
            max_transitions: limits.admissibility.max_transition_depth,
            cross_domain_allowed: false,
            transcript_span_at_creation: transcript_span,
            parent_boundary_hash: None,
            boundary_sequence,
            activation_transcript_position: activation_position,
            expiration_transcript_position: None,
        };
        b.boundary_hash = b.constitutional_hash();
        b
    }
    pub fn is_active_at(&self, transcript_position: u64) -> bool {
        transcript_position >= self.activation_transcript_position
            && self
                .expiration_transcript_position
                .is_none_or(|exp| transcript_position < exp)
    }
    pub fn with_parent_boundary(mut self, h: [u8; 32]) -> Self {
        self.parent_boundary_hash = Some(h);
        self.boundary_hash = self.constitutional_hash();
        self
    }
    pub fn with_expiration(mut self, pos: u64) -> Self {
        self.expiration_transcript_position = Some(pos);
        self.boundary_hash = self.constitutional_hash();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::ExecutionContext;
    use crate::execution_limits::ExecutionLimits;
    fn mb() -> ExecutionBoundary {
        let limits = ExecutionLimits::constitutional_default();
        let ctx = ExecutionContext::new(1, [0xAB; 32], 0);
        ExecutionBoundary::new(1, [0xAB; 32], &limits, ctx.context_hash, 0, 0, 0)
    }
    #[test]
    fn t_verify() {
        assert!(mb().verify().is_ok());
    }
    #[test]
    fn t_active() {
        let b = mb();
        assert!(b.is_active_at(0));
        assert!(b.is_active_at(100));
    }
    #[test]
    fn t_expiration() {
        let b = mb().with_expiration(50);
        assert!(!b.is_active_at(50));
    }
}
