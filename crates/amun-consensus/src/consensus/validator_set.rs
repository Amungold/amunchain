use std::collections::BTreeMap;
use crate::crypto::types::ConstitutionalEpoch;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo { pub validator_id: u64, pub voting_power: u64, pub public_key: [u8; 32] }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet { pub validators: Vec<ValidatorInfo>, pub epoch: ConstitutionalEpoch, pub total_power: u64, pub quorum_threshold: u64, pub hash: [u8; 32] }
impl ValidatorSet {
    pub fn new(validators: Vec<ValidatorInfo>, epoch: ConstitutionalEpoch) -> Self {
        let total_power: u64 = validators.iter().map(|v| v.voting_power).sum();
        let quorum_threshold = (total_power * 2 / 3) + 1;
        let hash = Self::compute_hash(&validators, &epoch);
        Self { validators, epoch, total_power, quorum_threshold, hash }
    }
    fn compute_hash(v: &[ValidatorInfo], e: &ConstitutionalEpoch) -> [u8; 32] { use blake3; let mut bytes = Vec::new(); bytes.extend_from_slice(b"AMUN_VALIDATOR_SET_V1"); bytes.extend_from_slice(&e.as_u64().to_be_bytes()); for vv in v { bytes.extend_from_slice(&vv.validator_id.to_be_bytes()); bytes.extend_from_slice(&vv.voting_power.to_be_bytes()); bytes.extend_from_slice(&vv.public_key); } blake3::hash(&bytes).into() }
    pub fn contains(&self, vid: u64) -> bool { self.validators.iter().any(|v| v.validator_id == vid) }
    pub fn voting_power(&self, vid: u64) -> u64 { self.validators.iter().find(|v| v.validator_id == vid).map(|v| v.voting_power).unwrap_or(0) }
    pub fn verify_quorum(&self, signed: &[u64]) -> bool { let signed_power: u64 = signed.iter().map(|&id| self.voting_power(id)).sum(); signed_power >= self.quorum_threshold }
}

pub struct ValidatorSetSnapshot { snapshots: BTreeMap<u64, ValidatorSet>, current_epoch: u64 }
impl ValidatorSetSnapshot {
    pub fn new(genesis: ValidatorSet) -> Self { let mut snapshots = BTreeMap::new(); let epoch = genesis.epoch.as_u64(); snapshots.insert(epoch, genesis); Self { snapshots, current_epoch: epoch } }
    pub fn get(&self, e: u64) -> Option<&ValidatorSet> { self.snapshots.get(&e) }
    pub fn current(&self) -> &ValidatorSet { self.snapshots.get(&self.current_epoch).unwrap() }
    pub fn advance_epoch(&mut self, new_set: ValidatorSet) { self.current_epoch = new_set.epoch.as_u64(); self.snapshots.insert(self.current_epoch, new_set); }
}
