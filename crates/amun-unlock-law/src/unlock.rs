use amun_quorum_certificate::QuorumCertificate;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct UnlockProof {
    pub previous_locked_round: u64,
    pub previous_locked_value: [u8; 32],
    pub higher_qc: QuorumCertificate,
    pub proof_hash: [u8; 32],
}

impl UnlockProof {
    pub fn new(
        previous_locked_round: u64,
        previous_locked_value: [u8; 32],
        higher_qc: QuorumCertificate,
    ) -> Result<Self, &'static str> {
        if higher_qc.round <= previous_locked_round {
            return Err("unlock requires higher-round QC");
        }

        let mut h = Hasher::new();
        h.update(b"AMUN_UNLOCK_V1");
        h.update(&previous_locked_round.to_le_bytes());
        h.update(&previous_locked_value);
        h.update(&higher_qc.certificate_hash);
        let mut proof_hash = [0u8; 32];
        proof_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Ok(Self { previous_locked_round, previous_locked_value, higher_qc, proof_hash })
    }

    pub fn verify(&self) -> bool {
        if self.higher_qc.round <= self.previous_locked_round {
            return false;
        }
        if !self.higher_qc.verify() {
            return false;
        }
        let mut h = Hasher::new();
        h.update(b"AMUN_UNLOCK_V1");
        h.update(&self.previous_locked_round.to_le_bytes());
        h.update(&self.previous_locked_value);
        h.update(&self.higher_qc.certificate_hash);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.proof_hash
    }
}

pub struct UnlockLaw;

impl UnlockLaw {
    pub fn can_unlock(
        locked_round: u64,
        locked_value: [u8; 32],
        unlock_proof: &UnlockProof,
    ) -> bool {
        unlock_proof.verify()
        && unlock_proof.previous_locked_round == locked_round
        && unlock_proof.previous_locked_value == locked_value
    }
}
