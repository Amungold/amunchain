use crate::types::id::{PeerId, PublicKey, ValidatorId};

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
    pub registered_epoch: u64,
    pub last_seen: u64,
    pub status: ValidatorStatus,
    pub stake_epoch: u64,
    pub protocol_version: u32,
    pub identity_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Suspended,
    Jailed,
    Tombstoned,
}
