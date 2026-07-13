use crate::ids::{PeerId, PublicKey, ValidatorId};

#[derive(Debug, Clone)]
pub struct ValidatorRecord {
    pub validator_id: ValidatorId,
    pub peer_id: PeerId,
    pub public_key: PublicKey,
    pub certificate_hash: [u8; 32],
    pub stake: u64,
    pub voting_power: u64,
    pub active: bool,
    pub slash_count: u32,
    pub registered_at: u64,
}
