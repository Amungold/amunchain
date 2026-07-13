use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignerBitmap(pub [u8; 32]);

impl SignerBitmap {
    pub const fn new() -> Self {
        Self([0u8; 32])
    }

    pub fn set(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= 256 {
            return Err("SignerBitmap index out of bounds");
        }
        self.0[index / 8] |= 1 << (index % 8);
        Ok(())
    }

    pub fn get(&self, index: usize) -> bool {
        index < 256 && (self.0[index / 8] & (1 << (index % 8))) != 0
    }

    pub fn count_ones(&self) -> usize {
        self.0.iter().map(|b| b.count_ones() as usize).sum()
    }
}

impl Default for SignerBitmap {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalEncode for SignerBitmap {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.0)
    }
}

impl CanonicalDecode for SignerBitmap {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 32 {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0401,
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[..32]);
        Ok((Self(bytes), 32))
    }
}
