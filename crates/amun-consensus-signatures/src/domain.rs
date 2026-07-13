use blake3::Hasher;

/// Domain-separated signing contexts to prevent cross-message replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureDomain {
    Vote,
    Proposal,
    QuorumCertificate,
    Attestation,
    UnlockProof,
}

impl SignatureDomain {
    pub fn tag(&self) -> &[u8] {
        match self {
            SignatureDomain::Vote => b"AMUN_SIGN_VOTE_V1",
            SignatureDomain::Proposal => b"AMUN_SIGN_PROPOSAL_V1",
            SignatureDomain::QuorumCertificate => b"AMUN_SIGN_QC_V1",
            SignatureDomain::Attestation => b"AMUN_SIGN_ATTEST_V1",
            SignatureDomain::UnlockProof => b"AMUN_SIGN_UNLOCK_V1",
        }
    }

    /// Compute canonical signing bytes for a message.
    pub fn sign_bytes(&self, message_hash: &[u8; 32], chain_id: u64) -> Vec<u8> {
        let mut h = Hasher::new();
        h.update(self.tag());
        h.update(&chain_id.to_le_bytes());
        h.update(message_hash);
        h.finalize().as_bytes().to_vec()
    }
}
