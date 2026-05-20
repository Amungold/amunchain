use crate::body::BlockBody;
use crate::header::BlockHeader;
use crate::limits::CONSTITUTIONAL_MAX_BLOCK_BYTES;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, HashDomain, WriteResult};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicHash32;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub PublicHash32);

impl BlockId {
    pub const fn new(hash: PublicHash32) -> Self {
        Self(hash)
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}

impl Block {
    pub fn new(header: BlockHeader, body: BlockBody) -> Self {
        Self { header, body }
    }

    pub fn compute_id(&self) -> BlockId {
        let mut buf = [0u8; BlockHeader::MAX_ENCODED_SIZE];
        let len = {
            let mut writer = amun_codec::BufferWriter::new(&mut buf);
            self.header
                .encode_to_writer(&mut writer)
                .expect("Header must fit MAX_ENCODED_SIZE");
            writer.position()
        };
        BlockId(HashDomain::Block.hash(&buf[..len]))
    }

    pub fn tx_count(&self) -> usize {
        self.body.tx_count()
    }
}

impl CanonicalEncode for Block {
    const MAX_ENCODED_SIZE: usize = CONSTITUTIONAL_MAX_BLOCK_BYTES;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        let header_size = BlockHeader::MAX_ENCODED_SIZE;
        let body_size = self.body.compute_encoded_size()?;
        let total = header_size.checked_add(body_size).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0007, 0x0113)
        })?;
        if total > CONSTITUTIONAL_MAX_BLOCK_BYTES {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x0007,
                0x0114,
            ));
        }
        self.header.encode_to_writer(writer)?;
        self.body.encode_to_writer(writer)?;
        Ok(())
    }
}

impl CanonicalDecode for Block {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        let (header, len1) = BlockHeader::decode(input)?;
        let (body, len2) = BlockBody::decode(&input[len1..])?;
        Ok((Self { header, body }, len1 + len2))
    }
}

impl BlockId {
    pub fn to_public_hash32(&self) -> PublicHash32 {
        self.0
    }
}
