use amun_chain_position::ChainPosition;
use blake3::Hasher;

/// A timeout certificate: proves that 2/3+ validators timed out.
#[derive(Debug, Clone)]
pub struct TimeoutCertificate {
    pub position: ChainPosition,
    pub round: u64,
    pub timeout_signatures: Vec<(u64, [u8; 64])>,
    pub certificate_hash: [u8; 32],
}

impl TimeoutCertificate {
    pub fn new(
        position: ChainPosition,
        round: u64,
        timeout_signatures: Vec<(u64, [u8; 64])>,
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_TIMEOUT_CERT_V1");
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        for (vid, sig) in &timeout_signatures {
            h.update(&vid.to_le_bytes());
            h.update(sig);
        }
        let mut certificate_hash = [0u8; 32];
        certificate_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self {
            position,
            round,
            timeout_signatures,
            certificate_hash,
        }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_TIMEOUT_CERT_V1");
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        for (vid, sig) in &self.timeout_signatures {
            h.update(&vid.to_le_bytes());
            h.update(sig);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.certificate_hash
    }
}
