// CanonicalEncode trait with single source of truth.
// MAX_ENCODED_SIZE is a compile-time bound for buffer allocation.
// The actual encoded size is determined by the writer position,
// not by a separate encoded_size() method. This eliminates the
// possibility of encoded_size() diverging from actual encoding.

use crate::writer::{BufferWriter, CanonicalWriter, HasherWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::{
    Amount, BlockHeight, ChainId, Epoch, Gas, Nonce, PublicKey, Round, Signature, ValidatorId,
};

pub trait CanonicalEncode {
    const MAX_ENCODED_SIZE: usize;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult;

    fn encode(&self, out: &mut [u8]) -> AmunResult<usize> {
        let mut writer = BufferWriter::new(out);
        self.encode_to_writer(&mut writer)?;
        Ok(writer.position())
    }

    fn encode_into_hasher(&self, hasher: &mut blake3::Hasher) -> WriteResult {
        let mut writer = HasherWriter::new(hasher);
        self.encode_to_writer(&mut writer)
    }
}

impl CanonicalEncode for u8 {
    const MAX_ENCODED_SIZE: usize = 1;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&[*self])
    }
}

impl CanonicalEncode for u16 {
    const MAX_ENCODED_SIZE: usize = 2;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.to_le_bytes())
    }
}

impl CanonicalEncode for u32 {
    const MAX_ENCODED_SIZE: usize = 4;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.to_le_bytes())
    }
}

impl CanonicalEncode for u64 {
    const MAX_ENCODED_SIZE: usize = 8;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.to_le_bytes())
    }
}

impl CanonicalEncode for u128 {
    const MAX_ENCODED_SIZE: usize = 16;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.to_le_bytes())
    }
}

impl CanonicalEncode for [u8; 32] {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self)
    }
}

impl CanonicalEncode for [u8; 48] {
    const MAX_ENCODED_SIZE: usize = 48;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self)
    }
}

impl CanonicalEncode for [u8; 96] {
    const MAX_ENCODED_SIZE: usize = 96;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self)
    }
}

macro_rules! impl_encode_newtype_8 {
    ($t:ty) => {
        impl CanonicalEncode for $t {
            const MAX_ENCODED_SIZE: usize = 8;
            fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
                self.0.encode_to_writer(writer)
            }
        }
    };
}

impl_encode_newtype_8!(Epoch);
impl_encode_newtype_8!(Round);
impl_encode_newtype_8!(BlockHeight);
impl_encode_newtype_8!(ChainId);
impl_encode_newtype_8!(Gas);
impl_encode_newtype_8!(Nonce);

impl CanonicalEncode for Amount {
    const MAX_ENCODED_SIZE: usize = 16;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}

impl CanonicalEncode for ValidatorId {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}

impl CanonicalEncode for PublicKey {
    const MAX_ENCODED_SIZE: usize = 48;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}

impl CanonicalEncode for Signature {
    const MAX_ENCODED_SIZE: usize = 96;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.0.encode_to_writer(writer)
    }
}
// PublicHash32 encoding — defined in amun-codec which already depends on kernel-types

// Generic Hash<D> encoding — covers BlockHash, TxHash, VoteHash, etc.
impl<D> CanonicalEncode for amun_kernel_types::Hash<D> {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self.as_bytes())
    }
}

impl CanonicalEncode for amun_kernel_types::PublicHash32 {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self.as_bytes())
    }
}

impl CanonicalEncode for amun_kernel_types::CommitmentHash32 {
    const MAX_ENCODED_SIZE: usize = 32;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(self.as_bytes())
    }
}
