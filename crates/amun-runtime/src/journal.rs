use amun_failure::{ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicHash32;
use heapless::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JournalEntry {
    Set {
        key: Vec<u8, 32>,
        value: Vec<u8, 64>,
    },
    Delete {
        key: Vec<u8, 32>,
    },
    Receipt {
        tx_hash: PublicHash32,
        gas_used: u64,
        return_code: u8,
    },
}

pub struct ExecutionJournal {
    entries: Vec<JournalEntry, 512>,
}

impl ExecutionJournal {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn record_set(&mut self, key: &[u8], value: &[u8]) -> Result<(), FailureContext> {
        let mut k = Vec::new();
        k.extend_from_slice(key).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0020)
        })?;
        let mut v = Vec::new();
        v.extend_from_slice(value).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0021)
        })?;
        if self.entries.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000A,
                0x0022,
            ));
        }
        self.entries
            .push(JournalEntry::Set { key: k, value: v })
            .map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0023)
            })?;
        Ok(())
    }
    pub fn record_receipt(
        &mut self,
        tx_hash: PublicHash32,
        gas_used: u64,
        return_code: u8,
    ) -> Result<(), FailureContext> {
        if self.entries.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000A,
                0x0027,
            ));
        }
        self.entries
            .push(JournalEntry::Receipt {
                tx_hash,
                gas_used,
                return_code,
            })
            .map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000A, 0x0028)
            })?;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for ExecutionJournal {
    fn default() -> Self {
        Self::new()
    }
}
