use crate::bitmap::SignerBitmap;
use crate::phase::ConsensusPhase;
use crate::round::ConsensusRound;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::{BlockHash, Epoch};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumCertificate {
    pub qc_type: ConsensusPhase,
    pub epoch: Epoch,
    pub round: ConsensusRound,
    pub block_hash: BlockHash,
    pub signer_bitmap: SignerBitmap,
    pub aggregate_signature: [u8; 96],
}

impl QuorumCertificate {
    pub fn signer_count(&self) -> usize {
        self.signer_bitmap.count_ones()
    }
    pub const MAX_ENCODED_SIZE: usize = 177;
}

impl CanonicalEncode for QuorumCertificate {
    const MAX_ENCODED_SIZE: usize = 177;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.qc_type.encode_to_writer(writer)?;
        self.epoch.encode_to_writer(writer)?;
        self.round.encode_to_writer(writer)?;
        self.block_hash.encode_to_writer(writer)?;
        self.signer_bitmap.encode_to_writer(writer)?;
        writer.write_bytes(&self.aggregate_signature)?;
        Ok(())
    }
}

impl CanonicalDecode for QuorumCertificate {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.is_empty() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0301,
            ));
        }
        let (qc_type, _) = ConsensusPhase::decode(&input[0..1])?;
        if !qc_type.is_qc_type() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::MalformedEncoding,
                0x0006,
                0x0303,
            ));
        }
        let (epoch, len1) = Epoch::decode(&input[1..])?;
        let (round, len2) = ConsensusRound::decode(&input[1 + len1..])?;
        let pos = 1 + len1 + len2;
        let (block_hash, len3) = BlockHash::decode(&input[pos..])?;
        let pos = pos + len3;
        let (signer_bitmap, len4) = SignerBitmap::decode(&input[pos..])?;
        let pos = pos + len4;
        if input.len() < pos + 96 {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0302,
            ));
        }
        let mut aggregate_signature = [0u8; 96];
        aggregate_signature.copy_from_slice(&input[pos..pos + 96]);
        let total = pos + 96;
        Ok((
            Self {
                qc_type,
                epoch,
                round,
                block_hash,
                signer_bitmap,
                aggregate_signature,
            },
            total,
        ))
    }
}
