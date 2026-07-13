use amun_failure::{ConstitutionalFault, FailureContext};
use amun_kernel_types::{Amount, Nonce, PublicHash32};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountState {
    pub nonce: Nonce,
    pub balance: Amount,
    pub storage_root: PublicHash32,
    pub code_hash: PublicHash32,
    pub flags: u8,
}

impl AccountState {
    pub fn new(nonce: Nonce, balance: Amount) -> Self {
        Self {
            nonce,
            balance,
            storage_root: PublicHash32::default(),
            code_hash: PublicHash32::default(),
            flags: 0,
        }
    }

    pub fn nonce_key(account: &[u8]) -> Result<heapless::Vec<u8, 32>, FailureContext> {
        let mut key = heapless::Vec::new();
        key.extend_from_slice(b"NONCE:").map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0050)
        })?;
        key.extend_from_slice(account).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0051)
        })?;
        Ok(key)
    }

    pub fn balance_key(account: &[u8]) -> Result<heapless::Vec<u8, 32>, FailureContext> {
        let mut key = heapless::Vec::new();
        key.extend_from_slice(b"BALANCE:").map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0052)
        })?;
        key.extend_from_slice(account).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0053)
        })?;
        Ok(key)
    }
}
