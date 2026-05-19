use amun_kernel_types::PublicHash32;
use amun_codec::{CanonicalEncode, CanonicalDecode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub tx_hash: PublicHash32,
    pub tx_index: u32,
    pub gas_used: u64,
    pub logs_hash: PublicHash32,
    pub return_code: u8,
}

impl ExecutionReceipt {
    pub const MAX_ENCODED_SIZE: usize = 77;
    pub fn new(tx_hash: PublicHash32, tx_index: u32, gas_used: u64, logs_hash: PublicHash32, return_code: u8) -> Self {
        Self { tx_hash, tx_index, gas_used, logs_hash, return_code }
    }
}

impl CanonicalEncode for ExecutionReceipt {
    const MAX_ENCODED_SIZE: usize = ExecutionReceipt::MAX_ENCODED_SIZE;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.tx_hash.encode_to_writer(writer)?;
        self.tx_index.encode_to_writer(writer)?;
        self.gas_used.encode_to_writer(writer)?;
        self.logs_hash.encode_to_writer(writer)?;
        writer.write_bytes(&[self.return_code])?;
        Ok(())
    }
}

impl CanonicalDecode for ExecutionReceipt {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < Self::MAX_ENCODED_SIZE {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall, 0x0008, 0x0300));
        }
        let (tx_hash, len1) = PublicHash32::decode(input)?;
        let (tx_index, len2) = u32::decode(&input[len1..])?;
        let pos = len1+len2;
        let (gas_used, len3) = u64::decode(&input[pos..])?;
        let pos = pos+len3;
        let (logs_hash, len4) = PublicHash32::decode(&input[pos..])?;
        Ok((Self { tx_hash, tx_index, gas_used, logs_hash, return_code: input[pos+len4] }, pos+len4+1))
    }
}
