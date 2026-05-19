use crate::domain::HashDomain;
use crate::writer::{CanonicalWriter, WriteResult};
use amun_failure::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::{ChainId, Epoch, PublicHash32};

pub const CURRENT_ENCODING_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug)]
pub struct ProtocolHeader {
    pub encoding_version: u8,
    pub domain: HashDomain,
    pub chain_id: ChainId,
    pub protocol_epoch: Epoch,
    pub constitution_hash: PublicHash32,
}

impl ProtocolHeader {
    pub const HEADER_SIZE: usize = 50;

    pub fn new(
        domain: HashDomain,
        chain_id: ChainId,
        protocol_epoch: Epoch,
        constitution_hash: PublicHash32,
    ) -> Self {
        Self {
            encoding_version: CURRENT_ENCODING_VERSION,
            domain,
            chain_id,
            protocol_epoch,
            constitution_hash,
        }
    }

    pub fn write(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&[self.encoding_version])?;
        writer.write_bytes(&[self.domain as u8])?;
        writer.write_bytes(&self.chain_id.0.to_le_bytes())?;
        writer.write_bytes(&self.protocol_epoch.0.to_le_bytes())?;
        writer.write_bytes(self.constitution_hash.as_bytes())?;
        Ok(())
    }

    pub fn read(
        input: &[u8],
        _expected_domain: HashDomain,
        expected_chain: ChainId,
    ) -> AmunResult<(Self, usize)> {
        if input.len() < Self::HEADER_SIZE {
            return Err(FailureContext::new(
                ConstitutionalFault::MalformedEncoding,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE_HEADER,
            ));
        }

        let version = input[0];
        if version != CURRENT_ENCODING_VERSION {
            return Err(FailureContext::new(
                ConstitutionalFault::EncodingVersionMismatch,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE_HEADER,
            ));
        }

        let domain = HashDomain::from_byte(input[1]).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::MalformedEncoding,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE_HEADER,
            )
        })?;

        let mut chain_bytes = [0u8; 8];
        chain_bytes.copy_from_slice(&input[2..10]);
        let chain_id = ChainId(u64::from_le_bytes(chain_bytes));
        if chain_id != expected_chain {
            return Err(FailureContext::new(
                ConstitutionalFault::ReplayViolation,
                module_ids::AMUN_CODEC,
                operation_ids::DECODE_HEADER,
            ));
        }

        let mut epoch_bytes = [0u8; 8];
        epoch_bytes.copy_from_slice(&input[10..18]);
        let protocol_epoch = Epoch(u64::from_le_bytes(epoch_bytes));

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&input[18..50]);
        let constitution_hash = PublicHash32::new(hash_bytes);

        Ok((
            Self {
                encoding_version: version,
                domain,
                chain_id,
                protocol_epoch,
                constitution_hash,
            },
            Self::HEADER_SIZE,
        ))
    }
}
