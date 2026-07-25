use amun_transactions::Transaction;
use std::collections::{HashMap, HashSet};

/// A constitutional mempool that holds pending transactions before block inclusion.
#[derive(Debug, Clone, Default)]
pub struct Mempool {
    /// Transactions ordered by insertion time.
    pending: Vec<Transaction>,
    /// Tracks the highest nonce seen per sender to prevent replay.
    /// This is a mempool-level index only: it resets on restart
    /// and is cleaned up when transactions are committed.
    /// Chain-level nonce validation should happen in block execution.
    nonce_index: HashMap<[u8; 32], u64>,
    /// Prevents duplicate transaction hashes.
    hash_index: HashSet<[u8; 32]>,
}

/// Maximum number of transactions allowed in the mempool.
const MAX_MEMPOOL_TXS: usize = 10_000;

impl Mempool {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            nonce_index: HashMap::new(),
            hash_index: HashSet::new(),
        }
    }

    /// Add a transaction to the mempool. Rejects if nonce is not greater than the last seen.
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), &'static str> {
        let tx_hash = tx.tx_hash();

        // Check for duplicate transaction hash
        if self.hash_index.contains(&tx_hash) {
            return Err("Transaction already in mempool");
        }

        // Check mempool size limit
        if self.pending.len() >= MAX_MEMPOOL_TXS {
            return Err("Mempool is full");
        }

        let current_nonce = self.nonce_index.get(&tx.sender).copied().unwrap_or(0);
        if tx.nonce <= current_nonce {
            return Err("Transaction nonce too low");
        }
        self.nonce_index.insert(tx.sender, tx.nonce);
        self.hash_index.insert(tx_hash);
        self.pending.push(tx);
        Ok(())
    }

    /// Number of pending transactions.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Take up to `max` transactions for inclusion in a block.
    pub fn take_for_block(&mut self, max: usize) -> Vec<Transaction> {
        let count = self.pending.len().min(max);
        self.pending.drain(0..count).collect()
    }

    /// Remove committed transactions from the mempool.
    pub fn remove_committed(&mut self, tx_hashes: &[[u8; 32]]) {
        let hash_set: std::collections::HashSet<[u8; 32]> = tx_hashes.iter().copied().collect();
        self.pending.retain(|tx| !hash_set.contains(&tx.tx_hash()));
        // Clean up hash index for removed transactions
        self.hash_index.retain(|h| !hash_set.contains(h));
        // Rebuild nonce_index from remaining transactions.
        // Committed nonces are intentionally removed from the mempool index.
        // Chain-level nonce enforcement during block execution prevents replay
        // of committed transactions across restarts.
        self.nonce_index.clear();
        for tx in &self.pending {
            self.nonce_index.insert(tx.sender, tx.nonce);
        }
    }

    /// Preview transactions without removing them (for block building)
    pub fn peek_for_block(&self, max: usize) -> Vec<Transaction> {
        self.pending.iter().take(max).cloned().collect()
    }

    /// Check if the mempool is empty.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_transactions::{TransactionPayload, TransferPayload};
    use ed25519_dalek::SigningKey;

    fn create_test_tx(sender_seed: u8, nonce: u64, amount: u64) -> Transaction {
        let seed = [sender_seed; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        let sender = signing_key.verifying_key().to_bytes();
        let mut tx = Transaction {
            version: 1,
            sender,
            nonce,
            payload: TransactionPayload::Transfer(TransferPayload {
                to: [2u8; 32],
                amount,
            }),
            signature: vec![],
        };
        tx.sign(&signing_key);
        tx
    }

    #[test]
    fn n24_add_transaction() {
        let mut mempool = Mempool::new();
        let tx = create_test_tx(1, 1, 100);
        assert!(mempool.add_transaction(tx).is_ok());
        assert_eq!(mempool.pending_count(), 1);
    }

    #[test]
    fn n24_reject_duplicate_nonce() {
        let mut mempool = Mempool::new();
        let tx1 = create_test_tx(1, 1, 100);
        let tx2 = create_test_tx(1, 1, 200);
        assert!(mempool.add_transaction(tx1).is_ok());
        assert!(mempool.add_transaction(tx2).is_err());
    }

    #[test]
    fn n24_take_for_block() {
        let mut mempool = Mempool::new();
        mempool.add_transaction(create_test_tx(1, 1, 100)).unwrap();
        mempool.add_transaction(create_test_tx(2, 1, 200)).unwrap();
        mempool.add_transaction(create_test_tx(3, 1, 300)).unwrap();
        let block_txs = mempool.take_for_block(2);
        assert_eq!(block_txs.len(), 2);
        assert_eq!(mempool.pending_count(), 1);
    }

    #[test]
    fn n24_remove_committed() {
        let mut mempool = Mempool::new();
        let tx1 = create_test_tx(1, 1, 100);
        let tx2 = create_test_tx(2, 1, 200);
        let h1 = tx1.tx_hash();
        mempool.add_transaction(tx1).unwrap();
        mempool.add_transaction(tx2).unwrap();
        mempool.remove_committed(&[h1]);
        assert_eq!(mempool.pending_count(), 1);
    }

    #[test]
    fn n24_order_preserved() {
        let mut mempool = Mempool::new();
        let tx1 = create_test_tx(1, 1, 100);
        let tx2 = create_test_tx(2, 1, 200);
        mempool.add_transaction(tx1.clone()).unwrap();
        mempool.add_transaction(tx2.clone()).unwrap();
        let block = mempool.take_for_block(2);
        assert_eq!(block[0].tx_hash(), tx1.tx_hash());
        assert_eq!(block[1].tx_hash(), tx2.tx_hash());
    }

    #[test]
    fn n24_empty_mempool() {
        let mut mempool = Mempool::new();
        assert!(mempool.is_empty());
        assert_eq!(mempool.pending_count(), 0);
        assert!(mempool.take_for_block(10).is_empty());
    }
}
