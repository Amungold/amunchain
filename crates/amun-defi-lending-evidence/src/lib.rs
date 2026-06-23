use sha2::{Digest, Sha256};

pub struct LendingEvidence;

impl LendingEvidence {
    pub fn generate_loan_creation_evidence(
        loan_id: [u8; 32],
        borrower: [u8; 32],
        principal: u64,
        interest_rate_bps: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_LENDING_CREATE_V1");
        hasher.update(loan_id);
        hasher.update(borrower);
        hasher.update(principal.to_le_bytes());
        hasher.update(interest_rate_bps.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn generate_repayment_evidence(
        loan_id: [u8; 32],
        borrower: [u8; 32],
        amount: u64,
        outstanding_after: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_LENDING_REPAY_V1");
        hasher.update(loan_id);
        hasher.update(borrower);
        hasher.update(amount.to_le_bytes());
        hasher.update(outstanding_after.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn generate_liquidation_evidence(
        loan_id: [u8; 32],
        liquidator: [u8; 32],
        collateral: u64,
        debt: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_LENDING_LIQUIDATE_V1");
        hasher.update(loan_id);
        hasher.update(liquidator);
        hasher.update(collateral.to_le_bytes());
        hasher.update(debt.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn generate_interest_accrual_evidence(
        loan_id: [u8; 32],
        interest_amount: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_LENDING_INTEREST_V1");
        hasher.update(loan_id);
        hasher.update(interest_amount.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }
}
