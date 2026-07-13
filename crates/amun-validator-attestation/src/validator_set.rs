use std::collections::BTreeMap;
use amun_kernel_types::epoch::Epoch;

#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub id: u64,
    pub public_key: [u8; 32],
    pub stake: u64,
}

#[derive(Debug, Clone)]
pub struct ValidatorSet {
    pub epoch: Epoch,
    validators: BTreeMap<u64, ValidatorInfo>,
    total_stake: u64,
    quorum_threshold: u64,
}

impl ValidatorSet {
    pub fn new(epoch: Epoch, validators: Vec<ValidatorInfo>) -> Result<Self, ValidatorSetError> {
        if validators.is_empty() {
            return Err(ValidatorSetError::EmptySet);
        }

        let total_stake: u64 = validators.iter().map(|v| v.stake).sum();
        let quorum_threshold = (total_stake * 2) / 3 + 1;

        let validators_map: BTreeMap<u64, ValidatorInfo> =
            validators.into_iter().map(|v| (v.id, v)).collect();

        Ok(Self {
            epoch,
            validators: validators_map,
            total_stake,
            quorum_threshold,
        })
    }

    pub fn get_validator(&self, id: u64) -> Option<&ValidatorInfo> {
        self.validators.get(&id)
    }

    pub fn has_quorum(&self, weight: u64) -> bool {
        weight >= self.quorum_threshold
    }

    pub fn total_stake(&self) -> u64 {
        self.total_stake
    }

    pub fn quorum_threshold(&self) -> u64 {
        self.quorum_threshold
    }

    pub fn validator_ids(&self) -> Vec<u64> {
        self.validators.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.validators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }
}

#[derive(Debug)]
pub enum ValidatorSetError {
    EmptySet,
}
