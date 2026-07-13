//! ReplayCertificate — cryptographically scoped admissibility envelope.
//!
//! ARCHITECTURAL DISTINCTION:
//!   Certificate VALIDITY is determined within the constitutional kernel.
//!   Certificate AUTHORITY (finality) is determined by the consensus layer.

use crate::certificate_scope::CertificateScope;
use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains::DOMAIN_REPLAY_CERTIFICATE;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCertificate {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub certificate_id: u64,
    pub certificate_hash: ConstitutionalHash,
    pub scope: CertificateScope,
    pub journal_root: ConstitutionalHash,
    pub state_root: ConstitutionalHash,
    pub parent_certificate_hash: Option<ConstitutionalHash>,
    pub attestation_note: Option<Vec<u8>>,
}

impl ConstitutionalIdentity for ReplayCertificate {
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

impl ConstitutionalObject for ReplayCertificate {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(DOMAIN_REPLAY_CERTIFICATE);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.certificate_id)
            .update_u64(self.scope.transcript_start)
            .update_u64(self.scope.transcript_end)
            .update_bytes(&self.scope.context_hash)
            .update_u32(self.scope.constitutional_revision)
            .update_u32(self.scope.replay_revision)
            .update_bytes(&self.scope.boundary_hash)
            .update_u8(self.scope.outcome as u8)
            .update_bytes(&self.journal_root)
            .update_bytes(&self.state_root)
            .update_optional_hash(self.parent_certificate_hash.as_ref());
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000D || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid certificate schema",
            ));
        }
        if self.scope.transcript_end < self.scope.transcript_start {
            return Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::BOUNDARY,
                severity::HARD_FAILURE,
                "Certificate scope end before start",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.certificate_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Certificate hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }

    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        if self.scope.constitutional_revision != self.constitutional_revision
            || self.scope.replay_revision != self.replay_revision
        {
            return Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Scope revision mismatch",
            ));
        }
        Ok(())
    }
}

impl ReplayCertificate {
    pub fn new(
        certificate_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        transcript_start: u64,
        transcript_end: u64,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        outcome: crate::replay_outcome::ReplayOutcome,
        journal_root: ConstitutionalHash,
        state_root: ConstitutionalHash,
        parent_certificate_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let scope = CertificateScope {
            transcript_start,
            transcript_end,
            context_hash,
            constitutional_revision,
            replay_revision,
            boundary_hash,
            outcome,
        };
        let mut c = Self {
            schema_id: 0x000D,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            certificate_id,
            certificate_hash: [0; 32],
            scope,
            journal_root,
            state_root,
            parent_certificate_hash,
            attestation_note: None,
        };
        c.certificate_hash = c.constitutional_hash();
        c
    }

    pub fn outcome(&self) -> crate::replay_outcome::ReplayOutcome {
        self.scope.outcome
    }
    pub fn is_admitted(&self) -> bool {
        self.scope.outcome.is_admitted()
    }

    pub fn verify_scope_against_parent(
        &self,
        parent: &ReplayCertificate,
    ) -> Result<(), ConstitutionalFailure> {
        match self.scope.verify_against_parent(&parent.scope) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConstitutionalFailure::new(
                self.certificate_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Scope monotonicity violation",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::replay_outcome::ReplayOutcome;

    fn mc(
        id: u64,
        start: u64,
        end: u64,
        outcome: ReplayOutcome,
        parent: Option<ConstitutionalHash>,
    ) -> ReplayCertificate {
        ReplayCertificate::new(
            id, 1, 1, start, end, [0xAB; 32], [0xBC; 32], outcome, [0xCD; 32], [0xDE; 32], parent,
        )
    }

    #[test]
    fn test_cert_verifies() {
        assert!(mc(1, 0, 99, ReplayOutcome::Admitted, None).verify().is_ok());
    }
    #[test]
    fn test_hash_det() {
        assert_eq!(
            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
        );
    }
    #[test]
    fn test_scope_affects_hash() {
        assert_ne!(
            mc(1, 0, 49, ReplayOutcome::Admitted, None).certificate_hash,
            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
        );
    }
    #[test]
    fn test_outcome_affects_hash() {
        assert_ne!(
            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
            mc(1, 0, 99, ReplayOutcome::Divergent, None).certificate_hash
        );
    }
    #[test]
    fn test_monotonicity_ok() {
        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
        let c = mc(2, 0, 49, ReplayOutcome::Admitted, Some(p.certificate_hash));
        assert!(c.verify_scope_against_parent(&p).is_ok());
    }

    #[test]
    fn test_monotonicity_violated() {
        // Parent and child have DIFFERENT context hashes → Divergent → rejected
        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
        let c = ReplayCertificate::new(
            2,
            1,
            1,
            0,
            49,
            [0xCD; 32], // Different context than parent's [0xAB; 32]
            [0xBC; 32],
            ReplayOutcome::Admitted,
            [0xCD; 32],
            [0xDE; 32],
            Some(p.certificate_hash),
        );
        assert!(c.verify_scope_against_parent(&p).is_err());
    }

    #[test]
    fn test_invalid_span_rejected() {
        let mut c = mc(1, 100, 50, ReplayOutcome::Admitted, None);
        c.certificate_hash = c.constitutional_hash();
        assert!(c.verify_structure().is_err());
    }
    #[test]
    fn test_revision_mismatch_rejected() {
        let mut c = mc(1, 0, 99, ReplayOutcome::Admitted, None);
        c.scope.constitutional_revision = 2;
        c.certificate_hash = c.constitutional_hash();
        assert!(c.verify_constitutional().is_err());
    }
}
