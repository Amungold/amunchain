use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsensusPhase {
    Proposal = 0x01,
    PrepareVote = 0x02,
    PreCommitVote = 0x03,
    CommitVote = 0x04,
    TimeoutVote = 0x05,
    TimeoutCertificate = 0x06,
}

impl ConsensusPhase {
    pub const fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(Self::Proposal),
            0x02 => Some(Self::PrepareVote),
            0x03 => Some(Self::PreCommitVote),
            0x04 => Some(Self::CommitVote),
            0x05 => Some(Self::TimeoutVote),
            0x06 => Some(Self::TimeoutCertificate),
            _ => None,
        }
    }

    pub const fn as_byte(self) -> u8 {
        self as u8
    }

    pub const fn is_vote(self) -> bool {
        matches!(
            self,
            Self::PrepareVote | Self::PreCommitVote | Self::CommitVote | Self::TimeoutVote
        )
    }

    pub const fn is_vote_qc_type(self) -> bool {
        matches!(
            self,
            Self::PrepareVote | Self::PreCommitVote | Self::CommitVote
        )
    }

    pub const fn is_timeout_cert_type(self) -> bool {
        matches!(self, Self::TimeoutCertificate)
    }

    pub const fn is_qc_type(self) -> bool {
        self.is_vote_qc_type() || self.is_timeout_cert_type()
    }
}

impl CanonicalEncode for ConsensusPhase {
    const MAX_ENCODED_SIZE: usize = 1;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&[self.as_byte()])
    }
}

impl CanonicalDecode for ConsensusPhase {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.is_empty() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0101,
            ));
        }
        Self::from_byte(input[0]).map(|p| (p, 1)).ok_or_else(|| {
            amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::MalformedEncoding,
                0x0006,
                0x0102,
            )
        })
    }
}
