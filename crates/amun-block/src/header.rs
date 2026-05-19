use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_consensus_types::{ConsensusRound, ValidatorIndex};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::{BlockHeight, ChainId, Epoch, PublicHash32, StateCommitment};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub height: BlockHeight,
    pub epoch: Epoch,
    pub round: ConsensusRound,
    pub parent_hash: PublicHash32,
    pub state_root: StateCommitment,
    pub tx_root: PublicHash32,
    pub proposer: ValidatorIndex,
    pub chain_id: ChainId,
    pub constitution_hash: PublicHash32,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
impl BlockHeader {
    pub const MAX_ENCODED_SIZE: usize = 170;

    pub fn new(
        #[allow(clippy::too_many_arguments)] height: BlockHeight,
        epoch: Epoch,
        round: ConsensusRound,
        parent_hash: PublicHash32,
        state_root: StateCommitment,
        tx_root: PublicHash32,
        proposer: ValidatorIndex,
        chain_id: ChainId,
        constitution_hash: PublicHash32,
        timestamp: u64,
    ) -> Self {
        Self {
            height,
            epoch,
            round,
            parent_hash,
            state_root,
            tx_root,
            proposer,
            chain_id,
            constitution_hash,
            timestamp,
        }
    }
}

impl CanonicalEncode for BlockHeader {
    const MAX_ENCODED_SIZE: usize = 170;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.height.encode_to_writer(writer)?;
        self.epoch.encode_to_writer(writer)?;
        self.round.encode_to_writer(writer)?;
        self.parent_hash.encode_to_writer(writer)?;
        self.state_root.encode_to_writer(writer)?;
        self.tx_root.encode_to_writer(writer)?;
        self.proposer.encode_to_writer(writer)?;
        self.chain_id.encode_to_writer(writer)?;
        self.constitution_hash.encode_to_writer(writer)?;
        self.timestamp.encode_to_writer(writer)?;
        Ok(())
    }
}

impl CanonicalDecode for BlockHeader {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < Self::MAX_ENCODED_SIZE {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                0x0007,
                0x0200,
            ));
        }
        let (height, len1) = BlockHeight::decode(input)?;
        let (epoch, len2) = Epoch::decode(&input[len1..])?;
        let pos = len1 + len2;
        let (round, len3) = ConsensusRound::decode(&input[pos..])?;
        let pos = pos + len3;
        let (parent_hash, len4) = PublicHash32::decode(&input[pos..])?;
        let pos = pos + len4;
        let (state_root, len5) = StateCommitment::decode(&input[pos..])?;
        let pos = pos + len5;
        let (tx_root, len6) = PublicHash32::decode(&input[pos..])?;
        let pos = pos + len6;
        let (proposer, len7) = ValidatorIndex::decode(&input[pos..])?;
        let pos = pos + len7;
        let (chain_id, len8) = ChainId::decode(&input[pos..])?;
        let pos = pos + len8;
        let (constitution_hash, len9) = PublicHash32::decode(&input[pos..])?;
        let pos = pos + len9;
        let (timestamp, len10) = u64::decode(&input[pos..])?;
        let total = pos + len10;
        Ok((
            Self {
                height,
                epoch,
                round,
                parent_hash,
                state_root,
                tx_root,
                proposer,
                chain_id,
                constitution_hash,
                timestamp,
            },
            total,
        ))
    }
}
