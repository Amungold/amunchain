// Byzantine evidence system.

use amun_kernel_types::{BlockHash, Epoch, PublicKey, Round, Signature, StateCommitment};
use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvidenceType {
    Equivocation = 0x01,
    InvalidVote = 0x02,
    InvalidProposal = 0x03,
    LockViolation = 0x04,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquivocationPosition {
    pub epoch: Epoch,
    pub round: Round,
    pub step: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedMessage {
    pub validator: PublicKey,
    pub message_hash: BlockHash,
    pub signature: Signature,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleProof {
    pub root: StateCommitment,
    pub path: Vec<(StateCommitment, bool), 32>,
    pub leaf: [u8; 64],
    pub leaf_len: u16,
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceProof {
    Equivocation {
        position: EquivocationPosition,
        first: SignedMessage,
        second: SignedMessage,
    },
    InvalidVote {
        vote: SignedMessage,
        invalidity_proof: MerkleProof,
    },
}

#[derive(Clone, Debug)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub accused_validator: PublicKey,
    pub epoch: Epoch,
    pub round: Round,
    pub proof: EvidenceProof,
}

impl Evidence {
    pub fn compute_hash(&self) -> BlockHash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[self.evidence_type as u8]);
        hasher.update(&self.accused_validator.0);
        hasher.update(&self.epoch.0.to_le_bytes());
        hasher.update(&self.round.0.to_le_bytes());
        BlockHash::new(hasher.finalize().into())
    }

    pub fn verify(&self) -> Result<(), &'static str> {
        match &self.proof {
            EvidenceProof::Equivocation { first, second, .. } => {
                if first.message_hash.as_bytes() == second.message_hash.as_bytes() {
                    return Err("Not equivocation: same message hash");
                }
                Ok(())
            }
            EvidenceProof::InvalidVote { .. } => Ok(()),
        }
    }
}
