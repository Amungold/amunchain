use sha2::Digest;
use amun_resource_core::{ResourceId, ResourceRegistry, RegistryError};
use amun_defi_lending_core::{LoanPosition, CollateralPosition, InterestModel};
use std::collections::BTreeMap;

pub struct LendingEngine {
    pub loans: BTreeMap<[u8; 32], LoanPosition>,
    pub collaterals: BTreeMap<[u8; 32], CollateralPosition>,
    pub next_loan_id: u64,
}

impl Default for LendingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LendingEngine {
    pub fn new() -> Self {
        Self { loans: BTreeMap::new(), collaterals: BTreeMap::new(), next_loan_id: 0 }
    }

    fn generate_loan_id(&mut self) -> ResourceId {
        self.next_loan_id += 1;
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"AMUN_LOAN_ID_V1");
        hasher.update(self.next_loan_id.to_le_bytes());
        ResourceId(hasher.finalize().into())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_loan(
        &mut self,
        _registry: &mut ResourceRegistry,
        borrower: [u8; 32],
        principal: u64,
        interest_rate_bps: u64,
        collateral_amount: u64,
        collateral_token: [u8; 32],
        current_height: u64,
    ) -> Result<(ResourceId, ResourceId), RegistryError> {
        let loan_id = self.generate_loan_id();
        let collateral_id_bytes = collateral_id_from_loan(&loan_id);
        let collateral_id = ResourceId(collateral_id_bytes);

        let loan = LoanPosition {
            loan_id,
            borrower,
            principal,
            outstanding: principal,
            interest_rate_bps,
            start_height: current_height,
            last_interest_height: current_height,
            collateral_locked: collateral_amount,
            collateral_token: ResourceId(collateral_token),
            active: true,
        };

        let collateral = CollateralPosition {
            collateral_id,
            loan_id,
            owner: borrower,
            amount: collateral_amount,
            token: ResourceId(collateral_token),
            locked: true,
        };

        self.loans.insert(loan_id.0, loan);
        self.collaterals.insert(collateral_id.0, collateral);

        Ok((loan_id, collateral_id))
    }

    pub fn accrue_interest(&mut self, loan_id: &[u8; 32], current_height: u64) -> Option<u64> {
        if let Some(loan) = self.loans.get_mut(loan_id) {
            if !loan.active { return None; }
            let blocks_elapsed = current_height - loan.last_interest_height;
            if blocks_elapsed == 0 { return Some(0); }
            let interest = InterestModel::compute_interest(loan.outstanding, loan.interest_rate_bps, blocks_elapsed);
            loan.outstanding += interest;
            loan.last_interest_height = current_height;
            Some(interest)
        } else { None }
    }

    pub fn repay(&mut self, loan_id: &[u8; 32], amount: u64) -> Result<u64, &'static str> {
        if let Some(loan) = self.loans.get_mut(loan_id) {
            if !loan.active { return Err("Loan not active"); }
            let repay_amount = std::cmp::min(amount, loan.outstanding);
            loan.outstanding -= repay_amount;
            if loan.outstanding == 0 { loan.active = false; }
            Ok(repay_amount)
        } else { Err("Loan not found") }
    }

    pub fn liquidate(&mut self, loan_id: &[u8; 32], _liquidator: [u8; 32], current_height: u64) -> Result<(u64, u64), &'static str> {
        let health_factor = self.get_health_factor(loan_id, current_height);
        if !InterestModel::is_liquidatable(health_factor) { return Err("Health factor too high"); }
        if let Some(loan) = self.loans.get_mut(loan_id) {
            if !loan.active { return Err("Loan not active"); }
            let collateral = loan.collateral_locked;
            loan.collateral_locked = 0;
            loan.active = false;
            Ok((collateral, loan.outstanding))
        } else { Err("Loan not found") }
    }

    pub fn get_health_factor(&self, loan_id: &[u8; 32], current_height: u64) -> u64 {
        if let Some(loan) = self.loans.get(loan_id) {
            let blocks_elapsed = current_height - loan.last_interest_height;
            let pending_interest = InterestModel::compute_interest(loan.outstanding, loan.interest_rate_bps, blocks_elapsed);
            let total_debt = loan.outstanding + pending_interest;
            InterestModel::compute_health_factor(loan.collateral_locked, total_debt, 8000)
        } else { 0 }
    }

    pub fn compute_lending_root(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"AMUN_LENDING_EVIDENCE_V1");
        for (id, loan) in &self.loans {
            hasher.update(id);
            hasher.update(loan.borrower);
            hasher.update(loan.outstanding.to_le_bytes());
            hasher.update(loan.collateral_locked.to_le_bytes());
            hasher.update([loan.active as u8]);
        }
        hasher.finalize().into()
    }
}

fn collateral_id_from_loan(loan_id: &ResourceId) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"COLLATERAL_");
    hasher.update(loan_id.0);
    hasher.finalize().into()
}
