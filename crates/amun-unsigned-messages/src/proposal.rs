use amun_chain_position::ChainPosition;
use blake3::Hasher;

const PROPOSAL_ENCODE_VERSION: u8 = 1;
const MIN_PROPOSAL_LEN: usize = 98;
const PROPOSAL_LEN_WITH_PARENT: usize = 130;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedProposal {
    pub proposer_id: u64,
    pub position: ChainPosition,
    pub round: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub parent_block_hash: Option<[u8; 32]>,
    pub unsigned_hash: [u8; 32],
}

impl UnsignedProposal {
    pub fn new(
        proposer_id: u64,
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        parent_block_hash: Option<[u8; 32]>,
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_UNSIGNED_PROPOSAL_V2");
        h.update(&proposer_id.to_le_bytes());
        h.update(&position.hash());
        h.update(&round.to_le_bytes());
        h.update(&block_hash);
        h.update(&state_root);
        if let Some(parent) = &parent_block_hash {
            h.update(b"PARENT");
            h.update(parent);
        }
        let mut unsigned_hash = [0u8; 32];
        unsigned_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { proposer_id, position, round, block_hash, state_root, parent_block_hash, unsigned_hash }
    }

    pub fn verify_hash(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_UNSIGNED_PROPOSAL_V2");
        h.update(&self.proposer_id.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.round.to_le_bytes());
        h.update(&self.block_hash);
        h.update(&self.state_root);
        if let Some(parent) = &self.parent_block_hash {
            h.update(b"PARENT");
            h.update(parent);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.unsigned_hash
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(130);
        buf.push(PROPOSAL_ENCODE_VERSION);
        buf.extend_from_slice(&self.proposer_id.to_le_bytes());
        buf.extend_from_slice(&self.position.epoch.to_le_bytes());
        buf.extend_from_slice(&self.position.sequence.to_le_bytes());
        buf.extend_from_slice(&self.round.to_le_bytes());
        buf.extend_from_slice(&self.block_hash);
        buf.extend_from_slice(&self.state_root);
        match &self.parent_block_hash {
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
        if data.len() < MIN_PROPOSAL_LEN { return None; }
        let version = data[0];
        if version != PROPOSAL_ENCODE_VERSION { return None; }

        let proposer_id = u64::from_le_bytes(data[1..9].try_into().ok()?);
        let epoch = u64::from_le_bytes(data[9..17].try_into().ok()?);
        let sequence = u64::from_le_bytes(data[17..25].try_into().ok()?);
        let position = ChainPosition::new(epoch, sequence);
        let round = u64::from_le_bytes(data[25..33].try_into().ok()?);
        let mut block_hash = [0u8; 32];
        block_hash.copy_from_slice(&data[33..65]);
        let mut state_root = [0u8; 32];
        state_root.copy_from_slice(&data[65..97]);

        let has_parent = data[97];
        let parent_block_hash = if has_parent == 1 {
            if data.len() < PROPOSAL_LEN_WITH_PARENT { return None; }
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&data[98..130]);
            if data.len() != PROPOSAL_LEN_WITH_PARENT { return None; }
            Some(hash)
        } else if has_parent == 0 {
            if data.len() != MIN_PROPOSAL_LEN { return None; }
            None
        } else {
            return None;
        };

        Some(Self::new(proposer_id, position, round, block_hash, state_root, parent_block_hash))
    }
}

#[derive(Debug, Clone)]
pub struct SignedProposal {
    pub unsigned: UnsignedProposal,
    pub signature: [u8; 64],
}

impl SignedProposal {
    pub fn new(unsigned: UnsignedProposal, signature: [u8; 64]) -> Self {
        Self { unsigned, signature }
    }

    pub fn verify_unsigned(&self) -> bool { self.unsigned.verify_hash() }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = self.unsigned.encode();
        buf.extend_from_slice(&self.signature);
        buf
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let total = data.len();
        if total < MIN_PROPOSAL_LEN + 64 { return None; }
        let unsigned_len = total - 64;
        let unsigned = UnsignedProposal::decode(&data[..unsigned_len])?;
        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[unsigned_len..]);
        Some(Self { unsigned, signature })
    }
}
