#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use sha2::{Sha256, Digest};
use amun_consensus_math::Fixed;

/// Constitutional account state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub balance: Fixed,
    pub delegated_to: Option<u64>,
    pub delegation_amount: Fixed,
    pub nonce: u64,
}

impl Account {
    pub fn new(balance: Fixed) -> Self {
        Self {
            balance,
            delegated_to: None,
            delegation_amount: Fixed::ZERO,
            nonce: 0,
        }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.balance.raw().to_be_bytes());
        bytes.push(self.delegated_to.map_or(0, |_| 1));
        if let Some(to) = self.delegated_to {
            bytes.extend_from_slice(&to.to_be_bytes());
        }
        bytes.extend_from_slice(&self.delegation_amount.raw().to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes
    }
}

/// Constitutional state machine state
#[derive(Debug, Clone)]
pub struct ConstitutionalState {
    pub accounts: BTreeMap<u64, Account>,
    pub total_supply: Fixed,
    pub height: u64,
    pub state_hash: [u8; 32],
}

impl ConstitutionalState {
    pub fn new() -> Self {
        let mut state = Self {
            accounts: BTreeMap::new(),
            total_supply: Fixed::ZERO,
            height: 0,
            state_hash: [0; 32],
        };
        state.recompute_hash();
        state
    }
    
    pub fn add_account(&mut self, id: u64, balance: Fixed) {
        self.accounts.insert(id, Account::new(balance));
        self.total_supply = self.total_supply + balance;
        self.recompute_hash();
    }
    
    pub fn get_account(&self, id: u64) -> Option<&Account> {
        self.accounts.get(&id)
    }
    
    pub fn get_account_mut(&mut self, id: u64) -> Option<&mut Account> {
        self.accounts.get_mut(&id)
    }
    
    /// Canonical state hash (deterministic)
    pub fn recompute_hash(&mut self) {
        let mut hasher = Sha256::new();
        
        // Hash height
        hasher.update(self.height.to_be_bytes());
        
        // Hash total supply
        hasher.update(self.total_supply.raw().to_be_bytes());
        
        // Hash accounts in deterministic order (BTreeMap is ordered)
        for (id, account) in self.accounts.iter() {
            hasher.update(id.to_be_bytes());
            hasher.update(&account.canonical_bytes());
        }
        
        self.state_hash = hasher.finalize().into();
    }
    
    pub fn hash(&self) -> [u8; 32] {
        self.state_hash
    }
}

impl Default for ConstitutionalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_hash_deterministic() {
        let mut state1 = ConstitutionalState::new();
        state1.add_account(1, Fixed::ONE);
        state1.add_account(2, Fixed::ONE);
        let hash1 = state1.hash();
        
        let mut state2 = ConstitutionalState::new();
        state2.add_account(1, Fixed::ONE);
        state2.add_account(2, Fixed::ONE);
        let hash2 = state2.hash();
        
        assert_eq!(hash1, hash2);
    }
}
