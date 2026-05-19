use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::Round;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsensusRound(pub Round);

impl ConsensusRound {
    pub const fn new(round: u64) -> Self {
        Self(Round::new(round))
    }
    pub const fn value(self) -> u64 {
        self.0.value()
    }
    pub fn next(self) -> Result<Self, amun_failure::FailureContext> {
        Ok(Self(self.0.next()?))
    }
}

impl Default for ConsensusRound {
    fn default() -> Self {
        Self(Round::ZERO)
    }
}

impl CanonicalEncode for ConsensusRound {
    const MAX_ENCODED_SIZE: usize = 8;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}

impl CanonicalDecode for ConsensusRound {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        let (round, len) = Round::decode(input)?;
        Ok((Self(round), len))
    }
}
