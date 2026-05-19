use amun_failure::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::{
    Amount, BlockHeight, ChainId, Epoch, Gas, Nonce, PublicKey, Round, Signature, ValidatorId,
};

pub trait CanonicalDecode: Sized {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)>;

    fn decode_exact(input: &[u8]) -> AmunResult<Self> {
        let (value, consumed) = Self::decode(input)?;
        if consumed != input.len() {
            return Err(FailureContext::new(
                ConstitutionalFault::TrailingBytesDetected,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE_EXACT,
            ));
        }
        Ok(value)
    }
}

impl CanonicalDecode for u8 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.is_empty() {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        Ok((input[0], 1))
    }
}

impl CanonicalDecode for u16 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 2 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        Ok((u16::from_le_bytes([input[0], input[1]]), 2))
    }
}

impl CanonicalDecode for u32 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 4 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&input[..4]);
        Ok((u32::from_le_bytes(bytes), 4))
    }
}

impl CanonicalDecode for u64 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 8 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[..8]);
        Ok((u64::from_le_bytes(bytes), 8))
    }
}

impl CanonicalDecode for u128 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 16 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&input[..16]);
        Ok((u128::from_le_bytes(bytes), 16))
    }
}

impl CanonicalDecode for [u8; 32] {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 32 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[..32]);
        Ok((bytes, 32))
    }
}

impl CanonicalDecode for [u8; 48] {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 48 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 48];
        bytes.copy_from_slice(&input[..48]);
        Ok((bytes, 48))
    }
}

impl CanonicalDecode for [u8; 96] {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 96 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 96];
        bytes.copy_from_slice(&input[..96]);
        Ok((bytes, 96))
    }
}

macro_rules! impl_decode_newtype {
    ($t:ty, $inner:ty) => {
        impl CanonicalDecode for $t {
            fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
                let (val, len) = <$inner>::decode(input)?;
                Ok((Self(val), len))
            }
        }
    };
}

impl_decode_newtype!(Epoch, u64);
impl_decode_newtype!(Round, u64);
impl_decode_newtype!(BlockHeight, u64);
impl_decode_newtype!(ChainId, u64);
impl_decode_newtype!(Gas, u64);
impl_decode_newtype!(Amount, u128);
impl_decode_newtype!(Nonce, u64);
impl_decode_newtype!(ValidatorId, [u8; 32]);
impl_decode_newtype!(PublicKey, [u8; 48]);
impl_decode_newtype!(Signature, [u8; 96]);

// Domain-typed Hash decoding
impl<D> CanonicalDecode for amun_kernel_types::Hash<D> {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 32 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[..32]);
        Ok((amun_kernel_types::Hash::new(bytes), 32))
    }
}

impl CanonicalDecode for amun_kernel_types::PublicHash32 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 32 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[..32]);
        Ok((amun_kernel_types::PublicHash32::new(bytes), 32))
    }
}

impl CanonicalDecode for amun_kernel_types::CommitmentHash32 {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 32 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE,
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[..32]);
        Ok((amun_kernel_types::CommitmentHash32::new(bytes), 32))
    }
}
