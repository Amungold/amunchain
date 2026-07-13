use super::certificate::EvolutionActivationCertificate;
use amun_lineage::record::EvolutionRecord;

/// Executes a constitutional evolution.
/// This is the bridge between frozen kernel and evolving civilization.
pub struct EvolutionExecutor;

impl EvolutionExecutor {
    /// Validate that an evolution can proceed.
    pub fn validate(
        _record: &EvolutionRecord,
        certificate: &EvolutionActivationCertificate,
    ) -> Result<bool, EvolutionError> {
        if !certificate.verify() {
            return Err(EvolutionError::InvalidCertificate);
        }
        if certificate.validator_quorum == 0 {
            return Err(EvolutionError::NoQuorum);
        }
        Ok(true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvolutionError {
    InvalidCertificate,
    NoQuorum,
    ReplayBreak,
    SnapshotBreak,
    ProofBreak,
    GovernanceBreak,
}
