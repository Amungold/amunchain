// Cryptographic domain separation using Blake3 keyed hashing with constant keys.

use amun_kernel_types::PublicHash32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HashDomain {
    Block = 0x01,
    Transaction = 0x02,
    Vote = 0x03,
    QuorumCertificate = 0x04,
    StateCommitment = 0x05,
    JournalEntry = 0x06,
    ValidatorSet = 0x07,
    EpochBoundary = 0x08,
}

impl HashDomain {
    pub const fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(Self::Block),
            0x02 => Some(Self::Transaction),
            0x03 => Some(Self::Vote),
            0x04 => Some(Self::QuorumCertificate),
            0x05 => Some(Self::StateCommitment),
            0x06 => Some(Self::JournalEntry),
            0x07 => Some(Self::ValidatorSet),
            0x08 => Some(Self::EpochBoundary),
            _ => None,
        }
    }

    const fn key(&self) -> &'static [u8; 32] {
        match self {
            Self::Block => &BLOCK_KEY,
            Self::Transaction => &TX_KEY,
            Self::Vote => &VOTE_KEY,
            Self::QuorumCertificate => &QC_KEY,
            Self::StateCommitment => &STATE_KEY,
            Self::JournalEntry => &JOURNAL_KEY,
            Self::ValidatorSet => &VALSET_KEY,
            Self::EpochBoundary => &EPOCH_KEY,
        }
    }

    pub fn hash(&self, data: &[u8]) -> PublicHash32 {
        let mut hasher = blake3::Hasher::new_keyed(self.key());
        hasher.update(&(data.len() as u32).to_le_bytes());
        hasher.update(data);
        PublicHash32::new(hasher.finalize().into())
    }
}

const BLOCK_KEY: [u8; 32] = [0x01u8; 32];
const TX_KEY: [u8; 32] = [0x02u8; 32];
const VOTE_KEY: [u8; 32] = [0x03u8; 32];
const QC_KEY: [u8; 32] = [0x04u8; 32];
const STATE_KEY: [u8; 32] = [0x05u8; 32];
const JOURNAL_KEY: [u8; 32] = [0x06u8; 32];
const VALSET_KEY: [u8; 32] = [0x07u8; 32];
const EPOCH_KEY: [u8; 32] = [0x08u8; 32];
