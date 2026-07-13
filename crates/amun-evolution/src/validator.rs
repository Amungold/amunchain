use super::certificate::EvolutionActivationCertificate;
use amun_lineage::record::{
    EvolutionProof, GovernanceGuarantee, ReplayGuarantee, SnapshotGuarantee,
};

pub struct EvolutionValidator;

impl EvolutionValidator {
    pub fn verify_consistency(
        proof: &EvolutionProof,
        certificate: &EvolutionActivationCertificate,
    ) -> bool {
        proof.replay_guarantee == certificate.replay_guarantee
            && proof.snapshot_guarantee == certificate.snapshot_guarantee
            && proof.proof_guarantee == certificate.proof_guarantee
            && proof.governance_guarantee == certificate.governance_guarantee
            && proof.verify()
            && certificate.verify()
    }

    pub fn preserves_continuity(proof: &EvolutionProof) -> bool {
        proof.replay_guarantee.rank() >= ReplayGuarantee::Deterministic.rank()
            && proof.snapshot_guarantee.rank() >= SnapshotGuarantee::Compatible.rank()
            && proof.governance_guarantee.rank() >= GovernanceGuarantee::Migrated.rank()
    }
}
