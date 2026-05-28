use sha2::{Digest, Sha256};

/// A MigrationWitness proves that state was successfully migrated
/// from a parent protocol to a child protocol without breaking
/// constitutional invariants.
#[derive(Debug, Clone)]
pub struct MigrationWitness {
    /// The state root under the parent protocol
    pub parent_state_root: [u8; 32],
    /// The state root under the child protocol after migration
    pub child_state_root: [u8; 32],
    /// Whether replay determinism was preserved during migration
    pub replay_preserved: bool,
    /// Whether the constitutional identity was preserved
    pub identity_preserved: bool,
    /// Hash of the migration rules applied
    pub migration_rules_hash: [u8; 32],
    /// The witness hash
    pub witness_hash: [u8; 32],
}

impl MigrationWitness {
    pub fn new(
        parent_root: [u8; 32],
        child_root: [u8; 32],
        replay_preserved: bool,
        identity_preserved: bool,
        rules_hash: [u8; 32],
    ) -> Self {
        let mut witness = Self {
            parent_state_root: parent_root,
            child_state_root: child_root,
            replay_preserved,
            identity_preserved,
            migration_rules_hash: rules_hash,
            witness_hash: [0u8; 32],
        };
        witness.witness_hash = witness.compute_hash();
        witness
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"AMUN_MIGRATION_WITNESS_V1");
        h.update(self.parent_state_root);
        h.update(self.child_state_root);
        h.update([self.replay_preserved as u8]);
        h.update([self.identity_preserved as u8]);
        h.update(self.migration_rules_hash);
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.witness_hash
    }
}

/// A MigrationCertificate authorizes state transition from a parent
/// protocol to a child protocol.
#[derive(Debug, Clone)]
pub struct MigrationCertificate {
    pub lineage_proof_hash: [u8; 32],
    pub witness: MigrationWitness,
    pub certificate_hash: [u8; 32],
}

impl MigrationCertificate {
    pub fn new(lineage_proof_hash: [u8; 32], witness: MigrationWitness) -> Self {
        let mut cert = Self {
            lineage_proof_hash,
            witness,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"AMUN_MIGRATION_CERTIFICATE_V1");
        h.update(self.lineage_proof_hash);
        h.update(self.witness.witness_hash);
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.certificate_hash && self.witness.verify()
    }
}

/// MigrationRules define the allowed transformations when migrating
/// state from a parent protocol to a child protocol.
#[derive(Debug, Clone)]
pub struct MigrationRules {
    pub parent_protocol_version: u32,
    pub child_protocol_version: u32,
    pub requires_replay: bool,
    pub requires_golden_compatibility: bool,
    pub max_invariant_breaks: u32,
    pub rules_hash: [u8; 32],
}

impl MigrationRules {
    pub fn new(
        parent_version: u32,
        child_version: u32,
        requires_replay: bool,
        requires_golden: bool,
        max_breaks: u32,
    ) -> Self {
        let mut rules = Self {
            parent_protocol_version: parent_version,
            child_protocol_version: child_version,
            requires_replay,
            requires_golden_compatibility: requires_golden,
            max_invariant_breaks: max_breaks,
            rules_hash: [0u8; 32],
        };
        rules.rules_hash = rules.compute_hash();
        rules
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"AMUN_MIGRATION_RULES_V1");
        h.update(self.parent_protocol_version.to_be_bytes());
        h.update(self.child_protocol_version.to_be_bytes());
        h.update([self.requires_replay as u8]);
        h.update([self.requires_golden_compatibility as u8]);
        h.update(self.max_invariant_breaks.to_be_bytes());
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.rules_hash
    }
}
