use crate::phase::ConsensusPhase;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::{ChainId, Epoch, PublicHash32};

pub const MAX_PAYLOAD_SIZE: usize = 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsensusMessage {
    pub protocol_version: u16,
    pub chain_id: ChainId,
    pub epoch: Epoch,
    pub phase: ConsensusPhase,
    pub constitution_hash: PublicHash32,
    pub payload: heapless::Vec<u8, MAX_PAYLOAD_SIZE>,
}

impl ConsensusMessage {
    pub fn new(
        protocol_version: u16,
        chain_id: ChainId,
        epoch: Epoch,
        phase: ConsensusPhase,
        constitution_hash: PublicHash32,
        payload: &[u8],
    ) -> Result<Self, &'static str> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err("Payload too large");
        }
        let mut vec = heapless::Vec::new();
        vec.extend_from_slice(payload)
            .map_err(|_| "Payload extend failed")?;
        Ok(Self {
            protocol_version,
            chain_id,
            epoch,
            phase,
            constitution_hash,
            payload: vec,
        })
    }

    pub const MAX_ENCODED_SIZE: usize = 2 +     // protocol_version: u16
        8 +     // chain_id: ChainId
        8 +     // epoch: Epoch
        1 +     // phase: ConsensusPhase
        32 +    // constitution_hash: PublicHash32
        2 +     // payload_len: u16
        MAX_PAYLOAD_SIZE; // payload bytes
}

impl CanonicalEncode for ConsensusMessage {
    const MAX_ENCODED_SIZE: usize = ConsensusMessage::MAX_ENCODED_SIZE;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.protocol_version.encode_to_writer(writer)?;
        self.chain_id.encode_to_writer(writer)?;
        self.epoch.encode_to_writer(writer)?;
        self.phase.encode_to_writer(writer)?;
        self.constitution_hash.encode_to_writer(writer)?;
        (self.payload.len() as u16).encode_to_writer(writer)?;
        writer.write_bytes(&self.payload)?;
        Ok(())
    }
}

impl CanonicalDecode for ConsensusMessage {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        let (protocol_version, len0) = u16::decode(input)?;
        let (chain_id, len1) = ChainId::decode(&input[len0..])?;
        let (epoch, len2) = Epoch::decode(&input[len0 + len1..])?;
        let pos = len0 + len1 + len2;
        let (phase, _) = ConsensusPhase::decode(&input[pos..pos + 1])?;
        let pos = pos + 1;
        let (constitution_hash, len3) = PublicHash32::decode(&input[pos..])?;
        let pos = pos + len3;
        let (payload_len, len4) = u16::decode(&input[pos..pos + 2])?;
        let pos = pos + len4;
        let payload_len = payload_len as usize;
        if payload_len > MAX_PAYLOAD_SIZE {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::CapacityExceeded,
                0x0006,
                0x0502,
            ));
        }
        if input.len() < pos + payload_len {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0503,
            ));
        }
        let mut payload = heapless::Vec::new();
        payload
            .extend_from_slice(&input[pos..pos + payload_len])
            .map_err(|_| {
                amun_failure::FailureContext::new(
                    amun_failure::ConstitutionalFault::CapacityExceeded,
                    0x0006,
                    0x0504,
                )
            })?;
        let total = pos + payload_len;
        Ok((
            Self {
                protocol_version,
                chain_id,
                epoch,
                phase,
                constitution_hash,
                payload,
            },
            total,
        ))
    }
}
