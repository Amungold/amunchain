use crate::phase::ConsensusPhase;
use crate::round::ConsensusRound;
use crate::validator::ValidatorIndex;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::{BlockHash, Epoch, ValidatorId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vote {
    pub phase: ConsensusPhase,
    pub epoch: Epoch,
    pub round: ConsensusRound,
    pub block_hash: BlockHash,
    pub validator_index: ValidatorIndex,
    pub validator_id: ValidatorId,
}

impl Vote {
    pub const MAX_ENCODED_SIZE: usize = 83;

    pub fn new(
        phase: ConsensusPhase,
        epoch: Epoch,
        round: ConsensusRound,
        block_hash: BlockHash,
        validator_index: ValidatorIndex,
        validator_id: ValidatorId,
    ) -> Result<Self, &'static str> {
        if !phase.is_vote() {
            return Err("Vote phase must be a vote type");
        }
        Ok(Self {
            phase,
            epoch,
            round,
            block_hash,
            validator_index,
            validator_id,
        })
    }
}

impl CanonicalEncode for Vote {
    const MAX_ENCODED_SIZE: usize = 83;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.phase.encode_to_writer(writer)?;
        self.epoch.encode_to_writer(writer)?;
        self.round.encode_to_writer(writer)?;
        self.block_hash.encode_to_writer(writer)?;
        self.validator_index.encode_to_writer(writer)?;
        self.validator_id.encode_to_writer(writer)?;
        Ok(())
    }
}

impl CanonicalDecode for Vote {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.is_empty() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0006,
                0x0201,
            ));
        }
        let (phase, _) = ConsensusPhase::decode(&input[0..1])?;
        if !phase.is_vote() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::MalformedEncoding,
                0x0006,
                0x0203,
            ));
        }
        let (epoch, len1) = Epoch::decode(&input[1..])?;
        let (round, len2) = ConsensusRound::decode(&input[1 + len1..])?;
        let pos = 1 + len1 + len2;
        let (block_hash, len3) = BlockHash::decode(&input[pos..])?;
        let pos = pos + len3;
        let (validator_index, len4) = ValidatorIndex::decode(&input[pos..])?;
        let pos = pos + len4;
        let (validator_id, len5) = ValidatorId::decode(&input[pos..])?;
        let total = 1 + len1 + len2 + len3 + len4 + len5;
        Ok((
            Self {
                phase,
                epoch,
                round,
                block_hash,
                validator_index,
                validator_id,
            },
            total,
        ))
    }
}
