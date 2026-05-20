use crate::state::StateStore;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};

pub struct NonceStore;

impl NonceStore {
    pub fn get_nonce<S: StateStore>(store: &S, account: &[u8]) -> AmunResult<u64> {
        match store.get(account)? {
            Some(data) if data.len() >= 8 => {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&data[..8]);
                Ok(u64::from_le_bytes(bytes))
            }
            _ => Ok(0),
        }
    }

    pub fn increment_nonce<S: StateStore>(store: &mut S, account: &[u8]) -> AmunResult<u64> {
        let current = Self::get_nonce(store, account)?;
        let next = current.checked_add(1).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0008, 0x0200)
        })?;
        store.set(account, &next.to_le_bytes())?;
        Ok(next)
    }

    pub fn validate_nonce<S: StateStore>(
        store: &S,
        account: &[u8],
        tx_nonce: u64,
    ) -> AmunResult<()> {
        let expected = Self::get_nonce(store, account)?
            .checked_add(1)
            .ok_or_else(|| {
                FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0008, 0x0201)
            })?;
        if tx_nonce != expected {
            return Err(FailureContext::new(
                ConstitutionalFault::ReplayViolation,
                0x0008,
                0x0202,
            ));
        }
        Ok(())
    }
}
