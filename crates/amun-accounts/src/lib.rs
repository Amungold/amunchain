use blake3::Hasher;
use amun_canonical_codec::CanonicalEncode;
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

impl amun_canonical_codec::CanonicalEncode for Account {
    fn canonical_encode(&self) -> Vec<u8> {
        let mut w = amun_canonical_codec::CanonicalWriter::new();
        w.write_u64(self.balance);
        w.write_u64(self.nonce);
        w.into_bytes()
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

    /// Compute a deterministic Blake3 state root over all accounts.
    pub fn state_root(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_ACCOUNTS_V1");
        for (addr, account) in &self.accounts {
            hasher.update(addr);
            hasher.update(&account.balance.to_le_bytes());
            hasher.update(&account.nonce.to_le_bytes());
        }
        hasher.finalize().into()
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
}

#[cfg(test)]
mod canonical_tests {
    use super::*;

    #[test]
    fn test_account_canonical_deterministic() {
        let a = Account { balance: 100, nonce: 5 };
        let enc1 = a.canonical_encode();
        let enc2 = a.canonical_encode();
        assert_eq!(enc1, enc2);
    }

    #[test]
    fn test_account_canonical_changes_with_field() {
        let a1 = Account { balance: 100, nonce: 5 };
        let a2 = Account { balance: 200, nonce: 5 };
        assert_ne!(a1.canonical_encode(), a2.canonical_encode());
    }

    #[test]
    fn test_account_canonical_changes_with_nonce() {
        let a1 = Account { balance: 100, nonce: 0 };
        let a2 = Account { balance: 100, nonce: 1 };
        assert_ne!(a1.canonical_encode(), a2.canonical_encode());
    }
}
