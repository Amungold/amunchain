use amun_transaction::tx::UnsignedTransaction;
use heapless::Vec;

const MAX_GAS_PER_TX: u64 = 10_000_000;

pub struct Mempool {
    pending: Vec<UnsignedTransaction, 4096>,
    max_size: usize,
    min_fee: u64,
}

impl Mempool {
    pub fn new(max_size: usize, min_fee: u64) -> Self {
        Self {
            pending: Vec::new(),
            max_size,
            min_fee,
        }
    }

    pub fn insert(&mut self, tx: UnsignedTransaction) -> Result<(), &'static str> {
        if self.pending.len() >= self.max_size {
            return Err("mempool full");
        }
        if tx.gas_limit.0 < self.min_fee {
            return Err("fee below minimum");
        }
        if tx.gas_limit.0 > MAX_GAS_PER_TX {
            return Err("gas exceeds maximum");
        }
        if self
            .pending
            .iter()
            .any(|e| e.sender_pubkey == tx.sender_pubkey && e.nonce == tx.nonce)
        {
            return Err("duplicate nonce");
        }
        self.pending.push(tx).map_err(|_| "push failed")
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.max_size
    }
}
