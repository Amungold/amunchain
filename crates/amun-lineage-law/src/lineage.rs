use sha2::{Digest, Sha256};

/// A LineageProof proves that a child protocol is a lawful descendant
/// of a parent protocol. It binds the child's protocol identity to the
/// parent's freeze certificate and golden fixtures.
#[derive(Debug, Clone)]
pub struct LineageProof {
    /// Hash of the parent protocol's freeze certificate
    pub parent_freeze_certificate_hash: [u8; 32],
    /// Hash of the child protocol's freeze certificate
    pub child_freeze_certificate_hash: [u8; 32],
    /// The protocol version of the parent
    pub parent_protocol_version: u32,
    /// The protocol version of the child
    pub child_protocol_version: u32,
    /// Whether the lineage has been cryptographically verified
    pub is_verified: bool,
    /// The lineage proof hash itself
    pub proof_hash: [u8; 32],
}

impl LineageProof {
    pub fn new(
        parent_freeze_hash: [u8; 32],
        child_freeze_hash: [u8; 32],
        parent_version: u32,
        child_version: u32,
    ) -> Self {
        let mut proof = Self {
            parent_freeze_certificate_hash: parent_freeze_hash,
            child_freeze_certificate_hash: child_freeze_hash,
            parent_protocol_version: parent_version,
            child_protocol_version: child_version,
            is_verified: false,
            proof_hash: [0u8; 32],
        };
        proof.proof_hash = proof.compute_hash();
        proof
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"AMUN_LINEAGE_PROOF_V1");
        h.update(self.parent_freeze_certificate_hash);
        h.update(self.child_freeze_certificate_hash);
        h.update(self.parent_protocol_version.to_be_bytes());
        h.update(self.child_protocol_version.to_be_bytes());
        h.finalize().into()
    }

    /// Verify that the child is a direct lawful descendant of the parent.
    /// Requires the parent's golden fixtures to be compatible with the child.
    pub fn verify(&mut self, parent_golden_root: &[u8; 32], child_golden_root: &[u8; 32]) -> bool {
        let computed = self.compute_hash();
        self.is_verified = computed == self.proof_hash && parent_golden_root == child_golden_root;
        self.is_verified
    }
}

/// A LineageCertificate is issued when a lineage proof is verified.
/// It becomes a constitutional artifact that can be referenced by
/// future descendants.
#[derive(Debug, Clone)]
pub struct LineageCertificate {
    pub proof: LineageProof,
    pub issued_at: u64,
    pub certificate_hash: [u8; 32],
}

impl LineageCertificate {
    pub fn new(proof: LineageProof, epoch: u64) -> Self {
        let mut cert = Self {
            proof,
            issued_at: epoch,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"AMUN_LINEAGE_CERTIFICATE_V1");
        h.update(self.proof.proof_hash);
        h.update(self.issued_at.to_be_bytes());
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.certificate_hash && self.proof.is_verified
    }
}

/// Verifies lineage chains across multiple protocol versions.
pub struct LineageVerification;

impl LineageVerification {
    /// Verify a chain of lineage proofs from ancestor to descendant.
    pub fn verify_chain(chain: &[LineageProof]) -> bool {
        if chain.is_empty() {
            return false;
        }
        for window in chain.windows(2) {
            let parent = &window[0];
            let child = &window[1];
            if child.parent_freeze_certificate_hash != parent.child_freeze_certificate_hash {
                return false;
            }
            if child.parent_protocol_version != parent.child_protocol_version {
                return false;
            }
        }
        true
    }

    /// Determine if a protocol version is a lawful descendant of an ancestor.
    pub fn is_lawful_descendant(
        ancestor_version: u32,
        descendant_version: u32,
        chain: &[LineageProof],
    ) -> bool {
        if ancestor_version == descendant_version {
            return true;
        }
        if ancestor_version > descendant_version {
            return false;
        }
        Self::verify_chain(chain)
    }
}
