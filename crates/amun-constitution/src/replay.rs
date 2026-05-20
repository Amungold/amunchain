use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::{ChainId, Epoch, PublicHash32};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplayContext {
    pub chain_id: ChainId,
    pub network_id: u32,
    pub genesis_hash: PublicHash32,
    pub fork_id: u32,
    pub epoch: Epoch,
    pub protocol_version: u16,
    pub crypto_suite_version: u16,
}

impl ReplayContext {
    pub const ENCODED_SIZE: usize = 64;

    pub const fn new(
        chain_id: ChainId,
        network_id: u32,
        genesis_hash: PublicHash32,
        fork_id: u32,
        epoch: Epoch,
        protocol_version: u16,
        crypto_suite_version: u16,
    ) -> Self {
        Self {
            chain_id,
            network_id,
            genesis_hash,
            fork_id,
            epoch,
            protocol_version,
            crypto_suite_version,
        }
    }

    pub fn encode_for_signing(&self) -> [u8; Self::ENCODED_SIZE] {
        let mut buf = [0u8; Self::ENCODED_SIZE];
        buf[..8].copy_from_slice(&self.chain_id.0.to_le_bytes());
        buf[8..12].copy_from_slice(&self.network_id.to_le_bytes());
        buf[12..44].copy_from_slice(self.genesis_hash.as_bytes());
        buf[44..48].copy_from_slice(&self.fork_id.to_le_bytes());
        buf[48..56].copy_from_slice(&self.epoch.0.to_le_bytes());
        buf[56..58].copy_from_slice(&self.protocol_version.to_le_bytes());
        buf[58..60].copy_from_slice(&self.crypto_suite_version.to_le_bytes());
        buf[60..64].copy_from_slice(&[0u8; 4]);
        buf
    }
}

impl CanonicalEncode for ReplayContext {
    const MAX_ENCODED_SIZE: usize = 64;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        writer.write_bytes(&self.encode_for_signing())?;
        Ok(())
    }
}

impl CanonicalDecode for ReplayContext {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 64 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0B00,
            ));
        }
        let chain_bytes: [u8; 8] = input[..8].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B01)
        })?;
        let net_bytes: [u8; 4] = input[8..12].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B02)
        })?;
        let mut gen_bytes = [0u8; 32];
        gen_bytes.copy_from_slice(&input[12..44]);
        let fork_bytes: [u8; 4] = input[44..48].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B03)
        })?;
        let epoch_bytes: [u8; 8] = input[48..56].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B04)
        })?;
        let proto_bytes: [u8; 2] = input[56..58].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B05)
        })?;
        let crypto_bytes: [u8; 2] = input[58..60].try_into().map_err(|_| {
            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B06)
        })?;
        Ok((
            Self {
                chain_id: ChainId(u64::from_le_bytes(chain_bytes)),
                network_id: u32::from_le_bytes(net_bytes),
                genesis_hash: PublicHash32::new(gen_bytes),
                fork_id: u32::from_le_bytes(fork_bytes),
                epoch: Epoch(u64::from_le_bytes(epoch_bytes)),
                protocol_version: u16::from_le_bytes(proto_bytes),
                crypto_suite_version: u16::from_le_bytes(crypto_bytes),
            },
            64,
        ))
    }
}
