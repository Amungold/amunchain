use amun_chain_position::ChainPosition;
use amun_constitution_core::phase::ConstitutionalPhase;
use blake3::Hasher;

const VOTE_ENCODE_VERSION: u8 = 1;
const MIN_VOTE_LEN_NIL: usize = 35;
const MIN_VOTE_LEN_WITH_HASH: usize = 67;
const SIGNED_NIL_VOTE_LEN: usize = 99;
const SIGNED_HASH_VOTE_LEN: usize = 131;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedVote {
    pub validator_id: u64,
    pub position: ChainPosition,
    pub round: u64,
    pub phase: ConstitutionalPhase,
    pub block_hash: Option<[u8; 32]>,
    pub unsigned_hash: [u8; 32],
}

impl UnsignedVote {
    pub fn new(
        validator_id: u64,
        position: ChainPosition,
        round: u64,
        phase: ConstitutionalPhase,
        block_hash: Option<[u8; 32]>,
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_UNSIGNED_VOTE_V1");
        h.update(&validator_id.to_le_bytes());
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        h.update(&[phase.tag()]);
        if let Some(bh) = &block_hash {
            h.update(bh);
        }
        let mut unsigned_hash = [0u8; 32];
        unsigned_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { validator_id, position, round, phase, block_hash, unsigned_hash }
    }

    pub fn verify_hash(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_UNSIGNED_VOTE_V1");
        h.update(&self.validator_id.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        h.update(&[self.phase.tag()]);
        if let Some(bh) = &self.block_hash {
            h.update(bh);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.unsigned_hash
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(128);
        buf.push(VOTE_ENCODE_VERSION);
        buf.extend_from_slice(&self.validator_id.to_le_bytes());
        buf.extend_from_slice(&self.position.epoch.to_le_bytes());
        buf.extend_from_slice(&self.position.sequence.to_le_bytes());
        buf.extend_from_slice(&self.round.to_le_bytes());
        buf.push(self.phase.tag());
        match &self.block_hash {
            Some(hash) => {
                buf.push(1u8);
                buf.extend_from_slice(hash);
            }
            None => {
                buf.push(0u8);
            }
        }
        buf
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < MIN_VOTE_LEN_NIL { return None; }
        
        let version = data[0];
        if version != VOTE_ENCODE_VERSION { return None; }
        
        let validator_id = u64::from_le_bytes(data[1..9].try_into().ok()?);
        let epoch = u64::from_le_bytes(data[9..17].try_into().ok()?);
        let sequence = u64::from_le_bytes(data[17..25].try_into().ok()?);
        let position = ChainPosition::new(epoch, sequence);
        let round = u64::from_le_bytes(data[25..33].try_into().ok()?);
        
        let phase_tag = data[33];
        let phase = match phase_tag {
            0 => ConstitutionalPhase::Propose,
            1 => ConstitutionalPhase::Prevote,
            2 => ConstitutionalPhase::Precommit,
            _ => return None,
        };
        
        let has_block = data[34];
        let block_hash = if has_block == 1 {
            if data.len() < MIN_VOTE_LEN_WITH_HASH { return None; }
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&data[35..67]);
            Some(hash)
        } else if has_block == 0 {
            if data.len() != MIN_VOTE_LEN_NIL { return None; }
            None
        } else {
            return None;
        };

        if block_hash.is_some() && data.len() != MIN_VOTE_LEN_WITH_HASH {
            return None;
        }

        Some(Self::new(validator_id, position, round, phase, block_hash))
    }
}

#[derive(Debug, Clone)]
pub struct SignedVote {
    pub unsigned: UnsignedVote,
    pub signature: [u8; 64],
}

impl SignedVote {
    pub fn new(unsigned: UnsignedVote, signature: [u8; 64]) -> Self {
        Self { unsigned, signature }
    }

    pub fn verify_unsigned(&self) -> bool { self.unsigned.verify_hash() }
    pub fn validator_id(&self) -> u64 { self.unsigned.validator_id }
    pub fn position(&self) -> ChainPosition { self.unsigned.position }
    pub fn round(&self) -> u64 { self.unsigned.round }
    pub fn phase(&self) -> ConstitutionalPhase { self.unsigned.phase }
    pub fn block_hash(&self) -> Option<[u8; 32]> { self.unsigned.block_hash }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = self.unsigned.encode();
        buf.extend_from_slice(&self.signature);
        buf
    }

    /// Strict decode: only two valid lengths.
    /// - Nil vote: 35 unsigned + 64 signature = 99 bytes
    /// - Hash vote: 67 unsigned + 64 signature = 131 bytes
    pub fn decode(data: &[u8]) -> Option<Self> {
        match data.len() {
            SIGNED_NIL_VOTE_LEN => {
                let unsigned = UnsignedVote::decode(&data[..MIN_VOTE_LEN_NIL])?;
                let mut signature = [0u8; 64];
                signature.copy_from_slice(&data[MIN_VOTE_LEN_NIL..]);
                Some(Self { unsigned, signature })
            }
            SIGNED_HASH_VOTE_LEN => {
                let unsigned = UnsignedVote::decode(&data[..MIN_VOTE_LEN_WITH_HASH])?;
                let mut signature = [0u8; 64];
                signature.copy_from_slice(&data[MIN_VOTE_LEN_WITH_HASH..]);
                Some(Self { unsigned, signature })
            }
            _ => None,
        }
    }
}
