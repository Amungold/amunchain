use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidatorIndex(pub u16);

impl ValidatorIndex {
    pub const fn new(index: u16) -> Self {
        Self(index)
    }
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl CanonicalEncode for ValidatorIndex {
    const MAX_ENCODED_SIZE: usize = 2;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}

impl CanonicalDecode for ValidatorIndex {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        let (idx, len) = u16::decode(input)?;
        Ok((Self(idx), len))
    }
}
