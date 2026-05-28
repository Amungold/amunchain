// State Transition Integration
// Bridges evolution layer with WAL, consensus, and snapshot boundaries.

use amun_lineage::lineage::CivilizationId;

use super::certificate::EvolutionActivationCertificate;

/// A constitutional checkpoint in the state transition pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalCheckpoint {
    pub epoch: u64,
    pub generation: u64,
    pub state_root: [u8; 32],
    pub civilization_id: CivilizationId,
    pub active_certificate_hash: Option<[u8; 32]>,
    pub checkpoint_hash: [u8; 32],
}

impl ConstitutionalCheckpoint {
    pub fn new(
        epoch: u64,
        generation: u64,
        state_root: [u8; 32],
        civilization_id: CivilizationId,
        active_certificate: Option<&EvolutionActivationCertificate>,
    ) -> Self {
        let cert_hash = active_certificate.map(|c| c.certificate_hash);
        let mut cp = Self {
            epoch,
            generation,
            state_root,
            civilization_id,
            active_certificate_hash: cert_hash,
            checkpoint_hash: [0u8; 32],
        };
        cp.checkpoint_hash = cp.compute_hash();
        cp
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = amun_canonical_codec::CanonicalHasher::with_domain(b"AMUN_CHECKPOINT_V1");
        h.update(&self.epoch.to_le_bytes());
        h.update(&self.generation.to_le_bytes());
        h.update(&self.state_root);
        h.update(&self.civilization_id.0);
        if let Some(cert) = &self.active_certificate_hash {
            h.update(cert);
        }
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.checkpoint_hash
    }
}

/// Transition validation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionResult {
    /// Transition is valid and can proceed.
    Valid {
        checkpoint: ConstitutionalCheckpoint,
    },
    /// Transition requires an evolution certificate.
    RequiresCertificate { reason: String },
    /// Transition would violate constitutional physics.
    PhysicsViolation { field: String, reason: String },
    /// Transition rejected by governance.
    GovernanceRejection { reason: String },
}
