use amun_constitutional_commitment::{
    ConstitutionalRoots, EconomicSnapshot, EndBlockPipeline, Hash32,
};
use amun_tokenomics_ledger::EconomicLedger;
use blake3::Hasher;
use std::collections::BTreeMap;

/// A constitutional account holding balance and nonce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

impl Account {
    pub fn new(balance: u64) -> Self {
        Self { balance, nonce: 0 }
    }
}

/// Deterministic account store backed by BTreeMap for canonical state root.
#[derive(Debug, Clone, Default)]
pub struct AccountStore {
    accounts: BTreeMap<[u8; 32], Account>,
}

impl AccountStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create or overwrite an account with an initial balance.
    pub fn create_account(&mut self, address: [u8; 32], balance: u64) {
        self.accounts.insert(address, Account::new(balance));
    }

    /// Get the balance of an account (0 if nonexistent).
    pub fn balance_of(&self, address: &[u8; 32]) -> u64 {
        self.accounts.get(address).map(|a| a.balance).unwrap_or(0)
    }

    /// Get the nonce of an account (0 if nonexistent).
    pub fn nonce_of(&self, address: &[u8; 32]) -> u64 {
        self.accounts.get(address).map(|a| a.nonce).unwrap_or(0)
    }

    /// Debit an account. Fails if insufficient balance.
    pub fn debit(&mut self, address: &[u8; 32], amount: u64) -> Result<(), &'static str> {
        let account = self.accounts.get_mut(address).ok_or("Account not found")?;
        if account.balance < amount {
            return Err("Insufficient balance");
        }
        account.balance -= amount;
        Ok(())
    }

    /// Credit an account. Creates the account if it does not exist.
    pub fn credit(&mut self, address: &[u8; 32], amount: u64) {
        let account = self.accounts.entry(*address).or_insert(Account::new(0));
        account.balance += amount;
    }

    /// Increment the nonce of an account.
    pub fn increment_nonce(&mut self, address: &[u8; 32]) {
        let account = self.accounts.entry(*address).or_insert(Account::new(0));
        account.nonce += 1;
    }

    /// Compute the total supply from all account balances.
    pub fn total_supply(&self) -> u64 {
        self.accounts.values().map(|a| a.balance).sum()
    }

    // =======================================================================
    // N132.2 — EconomicSnapshot builders
    // =======================================================================

    /// (Compatibility layer) Build snapshot from local state (all zeros).
    pub fn build_economic_snapshot(&self) -> EconomicSnapshot {
        let ledger = EconomicLedger::new();
        self.build_economic_snapshot_with_ledger(&ledger)
    }

    /// Build snapshot from the real EconomicLedger (Single Source of Truth).
    pub fn build_economic_snapshot_with_ledger(
        &self,
        ledger: &EconomicLedger,
    ) -> EconomicSnapshot {
        let total_supply = self.total_supply();
        EconomicSnapshot {
            total_supply,
            treasury_balance: ledger.treasury(),
            validator_reward_pool: ledger.validator_pool(),
            ecosystem_pool: ledger.ecosystem_pool(),
            burned_supply: ledger.burned_supply(),
            issued_supply: ledger.issued_supply(),
            staked_supply: ledger.staked_supply(),
            circulating_supply: total_supply
                .saturating_sub(ledger.burned_supply())
                .saturating_sub(ledger.staked_supply())
                .saturating_sub(ledger.treasury()),
        }
    }

    /// Compute the raw account state root (without constitutional commitment).
    pub fn raw_state_root(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_ACCOUNTS_V1");
        for (addr, account) in &self.accounts {
            hasher.update(addr);
            hasher.update(&account.balance.to_le_bytes());
            hasher.update(&account.nonce.to_le_bytes());
        }
        hasher.finalize().into()
    }

    /// (Compatibility layer) Compute CCA-aware state root (uses empty ledger).
    pub fn state_root(&self) -> [u8; 32] {
        let ledger = EconomicLedger::new();
        self.state_root_with_ledger(&ledger)
    }

    /// Compute CCA-aware state root from a real EconomicLedger.
    pub fn state_root_with_ledger(&self, ledger: &EconomicLedger) -> [u8; 32] {
        self.constitutional_roots_with_ledger(ledger).state_root
    }

    /// (Compatibility layer) Compute constitutional roots (uses empty ledger).
    pub fn constitutional_roots(&self) -> ConstitutionalRoots {
        let ledger = EconomicLedger::new();
        self.constitutional_roots_with_ledger(&ledger)
    }

    /// Compute constitutional roots from a real EconomicLedger.
    pub fn constitutional_roots_with_ledger(
        &self,
        ledger: &EconomicLedger,
    ) -> ConstitutionalRoots {
        let raw_root = self.raw_state_root();
        let snapshot = self.build_economic_snapshot_with_ledger(ledger);

        let identity_root: Hash32 = [0u8; 32];
        let evidence_root: Hash32 = [0u8; 32];
        let governance_root: Hash32 = [0u8; 32];

        if let Some(commitment) =
            EndBlockPipeline::execute(identity_root, evidence_root, governance_root, &snapshot)
        {
            ConstitutionalRoots::from_commitment(
                raw_root,
                commitment.economic_root,
                identity_root,
                governance_root,
                &commitment,
            )
        } else {
            ConstitutionalRoots {
                state_root: raw_root,
                commitment_root: [0u8; 32],
                economic_root: [0u8; 32],
                identity_root: [0u8; 32],
                governance_root: [0u8; 32],
                constitutional_root: [0u8; 32],
            }
        }
    }

    /// Number of accounts.
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn len(&self) -> usize {
        self.accounts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n25_create_account() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 1000);
        assert_eq!(store.balance_of(&[1u8; 32]), 1000);
    }

    #[test]
    fn n25_balance_lookup() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 500);
        assert_eq!(store.balance_of(&[1u8; 32]), 500);
        assert_eq!(store.balance_of(&[2u8; 32]), 0);
    }

    #[test]
    fn n25_debit_success() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 1000);
        assert!(store.debit(&[1u8; 32], 400).is_ok());
        assert_eq!(store.balance_of(&[1u8; 32]), 600);
    }

    #[test]
    fn n25_insufficient_balance() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 100);
        assert!(store.debit(&[1u8; 32], 500).is_err());
        assert_eq!(store.balance_of(&[1u8; 32]), 100);
    }

    #[test]
    fn n25_credit_creates_account() {
        let mut store = AccountStore::new();
        store.credit(&[3u8; 32], 1000);
        assert_eq!(store.balance_of(&[3u8; 32]), 1000);
    }

    #[test]
    fn n25_nonce_increment() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 100);
        store.increment_nonce(&[1u8; 32]);
        assert_eq!(store.nonce_of(&[1u8; 32]), 1);
    }

    #[test]
    fn n25_state_determinism() {
        let mut s1 = AccountStore::new();
        let mut s2 = AccountStore::new();
        for store in [&mut s1, &mut s2] {
            store.create_account([1u8; 32], 1000);
            store.create_account([2u8; 32], 500);
            store.debit(&[1u8; 32], 300).unwrap();
            store.credit(&[2u8; 32], 300);
        }
        assert_eq!(s1.state_root(), s2.state_root());
    }

    #[test]
    fn n25_different_state_different_root() {
        let mut s1 = AccountStore::new();
        let mut s2 = AccountStore::new();
        s1.create_account([1u8; 32], 1000);
        s2.create_account([1u8; 32], 999);
        assert_ne!(s1.state_root(), s2.state_root());
    }

    #[test]
    fn cca_state_root_changes_when_balance_changes() {
        let mut s1 = AccountStore::new();
        let mut s2 = AccountStore::new();
        s1.create_account([1u8; 32], 1000);
        s2.create_account([1u8; 32], 999);
        assert_ne!(s1.state_root(), s2.state_root());
    }

    #[test]
    fn cca_same_accounts_produce_same_state_root() {
        let mut s1 = AccountStore::new();
        let mut s2 = AccountStore::new();
        s1.create_account([1u8; 32], 1000);
        s2.create_account([1u8; 32], 1000);
        assert_eq!(s1.state_root(), s2.state_root());
    }

    #[test]
    fn cca_state_root_includes_commitment_root() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 1000);
        let root = store.state_root();
        let raw_root = store.raw_state_root();
        assert_ne!(root, raw_root, "CCA state root must differ from raw root");
    }

    #[test]
    fn cca_raw_state_root_matches_original_behavior() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 1000);
        let raw = store.raw_state_root();

        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_ACCOUNTS_V1");
        hasher.update(&[1u8; 32]);
        hasher.update(&1000u64.to_le_bytes());
        hasher.update(&0u64.to_le_bytes());
        let expected: [u8; 32] = hasher.finalize().into();

        assert_eq!(raw, expected);
    }

    #[test]
    fn cca_constitutional_roots_consistent() {
        let mut store = AccountStore::new();
        store.create_account([1u8; 32], 1000);
        let roots = store.constitutional_roots();
        assert_eq!(roots.state_root, store.state_root());
        assert_ne!(roots.commitment_root, [0u8; 32]);
        assert_ne!(roots.economic_root, [0u8; 32]);
        assert_ne!(roots.constitutional_root, [0u8; 32]);
    }
}
