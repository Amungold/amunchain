use amun_canonical_codec::CanonicalHasher;
use amun_lineage::record::{
    ContinuityClass, GovernanceGuarantee, ProofGuarantee, ReplayGuarantee, SnapshotGuarantee,
};

pub const CERTIFICATE_DOMAIN: &[u8] = b"AMUN_EVOLUTION_CERT_V1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalPhysicsProof {
    pub preserves_empty_root: bool,
    pub preserves_max_depth: bool,
    pub preserves_proof_semantics: bool,
    pub preserves_replay_determinism: bool,
    pub preserves_hash_domains: bool,
    pub proof_hash: [u8; 32],
}

impl ConstitutionalPhysicsProof {
    pub fn new(
        preserves_empty_root: bool,
        preserves_max_depth: bool,
        preserves_proof_semantics: bool,
        preserves_replay_determinism: bool,
        preserves_hash_domains: bool,
    ) -> Self {
        let mut proof = Self {
            preserves_empty_root,
            preserves_max_depth,
            preserves_proof_semantics,
            preserves_replay_determinism,
            preserves_hash_domains,
            proof_hash: [0u8; 32],
        };
        proof.proof_hash = proof.compute_hash();
        proof
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_PHYSICS_PROOF_V1");
        h.update(&[self.preserves_empty_root as u8]);
        h.update(&[self.preserves_max_depth as u8]);
        h.update(&[self.preserves_proof_semantics as u8]);
        h.update(&[self.preserves_replay_determinism as u8]);
        h.update(&[self.preserves_hash_domains as u8]);
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.proof_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionActivationCertificate {
    pub parent_constitution: [u8; 32],
    pub child_constitution: [u8; 32],
    pub activation_epoch: u64,
    pub ratification_proof: [u8; 32],
    pub validator_set_hash: [u8; 32],
    pub certificate_signature_root: [u8; 32],
    pub validator_quorum: u64,
    pub total_validators: u64,
    pub quorum_threshold_percent: u8,
    pub parent_lineage_id: [u8; 32],
    pub parent_certificate_hash: [u8; 32],
    pub head_set_hash: [u8; 32],
    pub replay_guarantee: ReplayGuarantee,
    pub snapshot_guarantee: SnapshotGuarantee,
    pub proof_guarantee: ProofGuarantee,
    pub governance_guarantee: GovernanceGuarantee,
    pub continuity_class: ContinuityClass,
    pub physics_proof: ConstitutionalPhysicsProof,
    pub certificate_hash: [u8; 32],
}

impl EvolutionActivationCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent: [u8; 32],
        child: [u8; 32],
        epoch: u64,
        ratification_proof: [u8; 32],
        validator_set_hash: [u8; 32],
        signature_root: [u8; 32],
        quorum: u64,
        total: u64,
        threshold: u8,
        parent_lineage_id: [u8; 32],
        parent_certificate_hash: [u8; 32],
        head_set_hash: [u8; 32],
        replay: ReplayGuarantee,
        snapshot: SnapshotGuarantee,
        proof: ProofGuarantee,
        governance: GovernanceGuarantee,
        continuity: ContinuityClass,
        physics_proof: ConstitutionalPhysicsProof,
    ) -> Self {
        let mut cert = Self {
            parent_constitution: parent,
            child_constitution: child,
            activation_epoch: epoch,
            ratification_proof,
            validator_set_hash,
            certificate_signature_root: signature_root,
            validator_quorum: quorum,
            total_validators: total,
            quorum_threshold_percent: threshold,
            parent_lineage_id,
            parent_certificate_hash,
            head_set_hash,
            replay_guarantee: replay,
            snapshot_guarantee: snapshot,
            proof_guarantee: proof,
            governance_guarantee: governance,
            continuity_class: continuity,
            physics_proof,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(CERTIFICATE_DOMAIN);
        h.update(&self.parent_constitution);
        h.update(&self.child_constitution);
        h.update(&self.activation_epoch.to_le_bytes());
        h.update(&self.ratification_proof);
        h.update(&self.validator_set_hash);
        h.update(&self.certificate_signature_root);
        h.update(&self.validator_quorum.to_le_bytes());
        h.update(&self.total_validators.to_le_bytes());
        h.update(&[self.quorum_threshold_percent]);
        h.update(&self.parent_lineage_id);
        h.update(&self.parent_certificate_hash);
        h.update(&self.head_set_hash);
        h.update(&[self.replay_guarantee.canonical_tag()]);
        h.update(&[self.snapshot_guarantee.canonical_tag()]);
        h.update(&[self.proof_guarantee.canonical_tag()]);
        h.update(&[self.governance_guarantee.canonical_tag()]);
        h.update(&[self.continuity_class.canonical_tag()]);
        h.update(&self.physics_proof.proof_hash);
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.physics_proof.verify() && self.compute_hash() == self.certificate_hash
    }
}
