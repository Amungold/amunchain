use amun_transactions::{Transaction, TransactionPayload, TransactionReceipt};
use amun_accounts::AccountStore;

/// Constitutional execution engine that processes transactions against account state.
#[derive(Debug, Clone)]
pub struct ExecutionEngine {
    pub state: AccountStore,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self { state: AccountStore::new() }
    }

    /// Execute a single transaction and return a receipt.
    pub fn execute(&mut self, tx: &Transaction) -> TransactionReceipt {
        // Step 1: Verify signature
        if !tx.verify() {
            return TransactionReceipt {
                tx_hash: tx.tx_hash(),
                success: false,
                error_code: Some(1), // Invalid signature
                sender: tx.sender,
                nonce: tx.nonce,
                gas_used: 0,
            };
        }

        // Step 2: Verify nonce
        let current_nonce = self.state.nonce_of(&tx.sender);
        if tx.nonce != current_nonce + 1 {
            return TransactionReceipt {
                tx_hash: tx.tx_hash(),
                success: false,
                error_code: Some(2), // Wrong nonce
                sender: tx.sender,
                nonce: tx.nonce,
                gas_used: 0,
            };
        }

        // Step 3: Decode and execute payload
        match &tx.payload {
            TransactionPayload::Transfer(transfer) => {
                // Step 4: Verify balance
                let balance = self.state.balance_of(&tx.sender);
                if balance < transfer.amount {
                    return TransactionReceipt {
                        tx_hash: tx.tx_hash(),
                        success: false,
                        error_code: Some(3), // Insufficient balance
                        sender: tx.sender,
                        nonce: tx.nonce,
                        gas_used: 0,
                    };
                }

                // Step 5: Apply state transition
                if self.state.debit(&tx.sender, transfer.amount).is_err() {
                    return TransactionReceipt {
                        tx_hash: tx.tx_hash(),
                        success: false,
                        error_code: Some(4), // Debit failed
                        sender: tx.sender,
                        nonce: tx.nonce,
                        gas_used: 0,
                    };
                }
                self.state.credit(&transfer.to, transfer.amount);
                self.state.increment_nonce(&tx.sender);
            }
        }

        // Step 6: Success
        TransactionReceipt {
            tx_hash: tx.tx_hash(),
            success: true,
            error_code: None,
            sender: tx.sender,
            nonce: tx.nonce,
            gas_used: 1,
        }
    }

    /// Execute multiple transactions and return all receipts.
    pub fn execute_block(&mut self, txs: &[Transaction]) -> Vec<TransactionReceipt> {
        txs.iter().map(|tx| self.execute(tx)).collect()
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_transactions::{TransferPayload, TransactionPayload};
    use ed25519_dalek::SigningKey;

    fn create_signed_transfer(sender_seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
        let seed = [sender_seed; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        let sender = signing_key.verifying_key().to_bytes();
        let mut tx = Transaction {
            version: 1, sender, nonce,
            payload: TransactionPayload::Transfer(TransferPayload { to, amount }),
            signature: vec![],
        };
        tx.sign(&signing_key);
        tx
    }

    #[test]
    fn n26_transfer_success() {
        let mut engine = ExecutionEngine::new();
        let sender_seed = [1u8; 32];
        let sender_addr = {
            let sk = SigningKey::from_bytes(&sender_seed);
            sk.verifying_key().to_bytes()
        };
        let receiver = [2u8; 32];
        engine.state.create_account(sender_addr, 1000);
        let tx = create_signed_transfer(1, 1, 300, receiver);
        let receipt = engine.execute(&tx);
        assert!(receipt.success);
        assert_eq!(engine.state.balance_of(&sender_addr), 700);
        assert_eq!(engine.state.balance_of(&receiver), 300);
    }

    #[test]
    fn n26_insufficient_balance() {
        let mut engine = ExecutionEngine::new();
        let sender_seed = [1u8; 32];
        let sender_addr = {
            let sk = SigningKey::from_bytes(&sender_seed);
            sk.verifying_key().to_bytes()
        };
        engine.state.create_account(sender_addr, 50);
        let tx = create_signed_transfer(1, 1, 100, [2u8; 32]);
        let receipt = engine.execute(&tx);
        assert!(!receipt.success);
        assert_eq!(receipt.error_code, Some(3));
        assert_eq!(engine.state.balance_of(&sender_addr), 50);
    }

    #[test]
    fn n26_wrong_nonce() {
        let mut engine = ExecutionEngine::new();
        let sender_seed = [1u8; 32];
        let sender_addr = {
            let sk = SigningKey::from_bytes(&sender_seed);
            sk.verifying_key().to_bytes()
        };
        engine.state.create_account(sender_addr, 1000);
        let tx = create_signed_transfer(1, 5, 100, [2u8; 32]);
        let receipt = engine.execute(&tx);
        assert!(!receipt.success);
        assert_eq!(receipt.error_code, Some(2));
    }

    #[test]
    fn n26_invalid_signature() {
        let mut engine = ExecutionEngine::new();
        let sender_seed = [1u8; 32];
        let sender_addr = {
            let sk = SigningKey::from_bytes(&sender_seed);
            sk.verifying_key().to_bytes()
        };
        engine.state.create_account(sender_addr, 1000);
        let mut tx = create_signed_transfer(1, 1, 100, [2u8; 32]);
        tx.signature = vec![0u8; 64];
        let receipt = engine.execute(&tx);
        assert!(!receipt.success);
        assert_eq!(receipt.error_code, Some(1));
    }

    #[test]
    fn n26_state_root_changes() {
        let mut engine = ExecutionEngine::new();
        let sender_seed = [1u8; 32];
        let sender_addr = {
            let sk = SigningKey::from_bytes(&sender_seed);
            sk.verifying_key().to_bytes()
        };
        engine.state.create_account(sender_addr, 1000);
        let root_before = engine.state.state_root();
        let tx = create_signed_transfer(1, 1, 300, [2u8; 32]);
        engine.execute(&tx);
        let root_after = engine.state.state_root();
        assert_ne!(root_before, root_after);
    }

    #[test]
    fn n26_execute_block() {
        let mut engine = ExecutionEngine::new();
        let s1 = [1u8; 32];
        let s2 = [3u8; 32];
        let a1 = { let sk = SigningKey::from_bytes(&s1); sk.verifying_key().to_bytes() };
        let a2 = { let sk = SigningKey::from_bytes(&s2); sk.verifying_key().to_bytes() };
        engine.state.create_account(a1, 1000);
        engine.state.create_account(a2, 500);
        let tx1 = create_signed_transfer(1, 1, 200, a2);
        let tx2 = create_signed_transfer(3, 1, 100, a1);
        let receipts = engine.execute_block(&[tx1, tx2]);
        assert_eq!(receipts.len(), 2);
        assert!(receipts[0].success);
        assert!(receipts[1].success);
        assert_eq!(engine.state.balance_of(&a1), 900);
        assert_eq!(engine.state.balance_of(&a2), 600);
    }
}
