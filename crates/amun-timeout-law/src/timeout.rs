use amun_chain_position::ChainPosition;
use blake3::Hasher;

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
        h.update(b"AMUN_TIMEOUT_V1");
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        for (vid, sig) in &timeout_signatures {
            h.update(&vid.to_le_bytes());
            h.update(sig);
        }
        let mut certificate_hash = [0u8; 32];
        certificate_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { position, round, timeout_signatures, certificate_hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_TIMEOUT_V1");
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

pub struct TimeoutLaw;

impl TimeoutLaw {
    pub fn can_advance_round(
        current_round: u64,
        timeout_cert: &TimeoutCertificate,
        threshold_weight: u64,
    ) -> bool {
        if !timeout_cert.verify() {
            return false;
        }
        if timeout_cert.round != current_round {
            return false;
        }

        let mut seen = std::collections::BTreeSet::new();
        for (vid, _) in &timeout_cert.timeout_signatures {
            seen.insert(*vid);
        }

        (seen.len() as u64) >= threshold_weight
    }
}
