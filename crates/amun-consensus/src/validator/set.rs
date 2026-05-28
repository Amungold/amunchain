//! Validator Set Management

use std::collections::{BTreeMap, HashSet};
use serde::{Serialize, Deserialize};
use crate::crypto::types::{AuthorityReference, AuthorityRootHash, ConstitutionalEpoch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
    Pending,
    Exited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRecord {
    pub validator_id: u64,
    pub voting_power: u64,
    pub status: ValidatorStatus,
    pub joined_epoch: ConstitutionalEpoch,
    pub last_active_epoch: ConstitutionalEpoch,
    pub slashed_epoch: Option<ConstitutionalEpoch>,
}

#[derive(Debug, Clone)]
pub struct ValidatorSet {
    validators: BTreeMap<u64, ValidatorRecord>,
    active_set: HashSet<u64>,
    total_voting_power: u64,
    current_epoch: ConstitutionalEpoch,
}

impl ValidatorSet {
    pub fn new(initial: Vec<(u64, u64)>) -> Self {
        let mut validators = BTreeMap::new();
        let mut active_set = HashSet::new();
        let mut total_power = 0;
        let epoch = ConstitutionalEpoch::new(0);

        for (id, power) in initial {
            let record = ValidatorRecord {
                validator_id: id,
                voting_power: power,
                status: ValidatorStatus::Active,
                joined_epoch: epoch,
                last_active_epoch: epoch,
                slashed_epoch: None,
            };
            validators.insert(id, record);
            active_set.insert(id);
            total_power += power;
        }

        Self {
            validators,
            active_set,
            total_voting_power: total_power,
            current_epoch: epoch,
        }
    }

    pub fn total_power(&self) -> u64 { self.total_voting_power }
    pub fn quorum_threshold(&self) -> u64 { (self.total_voting_power * 2 / 3) + 1 }
    pub fn active_validators(&self) -> Vec<u64> { self.active_set.iter().copied().collect() }
    pub fn current_epoch(&self) -> ConstitutionalEpoch { self.current_epoch }

    pub fn add_validator(&mut self, id: u64, power: u64) -> bool {
        if self.validators.contains_key(&id) {
            return false;
        }
        let record = ValidatorRecord {
            validator_id: id,
            voting_power: power,
            status: ValidatorStatus::Active,
            joined_epoch: self.current_epoch,
            last_active_epoch: self.current_epoch,
            slashed_epoch: None,
        };
        self.validators.insert(id, record);
        self.active_set.insert(id);
        self.total_voting_power += power;
        true
    }

    pub fn slash_validator(&mut self, id: u64, percentage: u64) -> bool {
        let record = match self.validators.get_mut(&id) {
            Some(r) => r,
            None => return false,
        };
        let new_power = (record.voting_power * (100 - percentage)) / 100;
        self.total_voting_power -= record.voting_power;
        self.total_voting_power += new_power;
        record.voting_power = new_power;
        record.status = ValidatorStatus::Slashed;
        record.slashed_epoch = Some(self.current_epoch);
        if new_power == 0 {
            self.active_set.remove(&id);
        }
        true
    }
}
