use crate::types::Hash256;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: Hash256,
    pub voting_power: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub validators: Vec<Validator>,
    pub total_power: u64,
    /// Internal index for O(1) power lookups.
    /// Must be rebuilt after deserialization via rebuild_index().
    #[serde(skip, default = "HashMap::new")]
    power_index: HashMap<Hash256, u64>,
}

impl ValidatorSet {
    pub fn new(validators: Vec<Validator>) -> Result<Self, String> {
        let mut ids = HashSet::new();
        let mut power_index = HashMap::new();
        for v in &validators {
            if !ids.insert(v.id) {
                return Err(format!("duplicate validator id: {:?}", v.id));
            }
            power_index.insert(v.id, v.voting_power);
        }
        let total_power = validators.iter().map(|v| v.voting_power).sum();
        Ok(Self {
            validators,
            total_power,
            power_index,
        })
    }

    /// Rebuild the internal power index from the validators list.
    /// Must be called after deserialization or any manual modification.
    pub fn rebuild_index(&mut self) {
        self.power_index.clear();
        for v in &self.validators {
            self.power_index.insert(v.id, v.voting_power);
        }
    }

    pub fn total_power(&self) -> u64 {
        self.total_power
    }

    /// O(1) lookup of voting power by validator id.
    pub fn power_of(&self, id: &Hash256) -> u64 {
        self.power_index.get(id).copied().unwrap_or(0)
    }

    pub fn has_quorum(&self, voting_power: u64) -> bool {
        (voting_power as u128) * 3 > (self.total_power as u128) * 2
    }

    pub fn has_majority(&self, voting_power: u64) -> bool {
        (voting_power as u128) * 2 > (self.total_power as u128)
    }
}
