use amun_defi_lending_engine::LendingEngine;
use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct CollateralLock {
    pub token_id: ResourceId,
    pub owner: [u8; 32],
    pub locked: bool,
}

pub struct NftCollateralEngine {
    pub locks: BTreeMap<[u8; 32], CollateralLock>,
    pub lending: LendingEngine,
}

impl Default for NftCollateralEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NftCollateralEngine {
    pub fn new() -> Self {
        Self {
            locks: BTreeMap::new(),
            lending: LendingEngine::new(),
        }
    }

    pub fn lock_nft(
        &mut self,
        registry: &ResourceRegistry,
        token_id: ResourceId,
        owner: &[u8; 32],
    ) -> Result<(), RegistryError> {
        let token = registry
            .get(&token_id)
            .ok_or(RegistryError::NotFound(token_id))?;
        if token.owner != *owner {
            return Err(RegistryError::NotActive(token_id));
        }
        if self.locks.contains_key(&token_id.0) {
            return Err(RegistryError::DuplicateId(token_id));
        }
        self.locks.insert(
            token_id.0,
            CollateralLock {
                token_id,
                owner: *owner,
                locked: true,
            },
        );
        Ok(())
    }

    pub fn is_locked(&self, token_id: &ResourceId) -> bool {
        self.locks
            .get(&token_id.0)
            .map(|l| l.locked)
            .unwrap_or(false)
    }

    pub fn borrow_against_nft(
        &mut self,
        registry: &mut ResourceRegistry,
        token_id: ResourceId,
        borrower: [u8; 32],
        loan_amount: u64,
        interest_rate_bps: u64,
        current_height: u64,
    ) -> Result<ResourceId, RegistryError> {
        if !self.is_locked(&token_id) {
            return Err(RegistryError::NotActive(token_id));
        }
        let lock = self.locks.get(&token_id.0).unwrap();
        if lock.owner != borrower {
            return Err(RegistryError::NotActive(token_id));
        }
        let (loan_id, _) = self.lending.create_loan(
            registry,
            borrower,
            loan_amount,
            interest_rate_bps,
            loan_amount * 2,
            token_id.0,
            current_height,
        )?;
        Ok(loan_id)
    }

    pub fn repay_and_unlock(
        &mut self,
        _registry: &mut ResourceRegistry,
        loan_id: &ResourceId,
        token_id: ResourceId,
        amount: u64,
    ) -> Result<(), &'static str> {
        self.lending.repay(&loan_id.0, amount)?;
        let loan = self.lending.loans.get(&loan_id.0).ok_or("Loan not found")?;
        if !loan.active {
            self.locks.remove(&token_id.0);
            Ok(())
        } else {
            Err("Loan still active")
        }
    }

    pub fn liquidate(
        &mut self,
        loan_id: &ResourceId,
        token_id: ResourceId,
        liquidator: [u8; 32],
        current_height: u64,
    ) -> Result<(u64, u64), &'static str> {
        let result = self
            .lending
            .liquidate(&loan_id.0, liquidator, current_height)?;
        self.locks.remove(&token_id.0);
        Ok(result)
    }

    pub fn compute_evidence_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_NFT_COLLATERAL_V1");
        for (id, lock) in &self.locks {
            hasher.update(id);
            hasher.update(lock.owner);
            hasher.update([lock.locked as u8]);
        }
        hasher.update(self.lending.compute_lending_root());
        hasher.finalize().into()
    }
}
