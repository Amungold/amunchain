use amun_accounts::AccountStore;
use amun_consensus_network::SlashingCertificate;
use amun_execution::ExecutionEngine;
use amun_mempool::Mempool;
use amun_merkle::{receipts_root, transactions_root};
use amun_transactions::{Transaction, TransactionReceipt};
use blake3::Hasher;

/// A constitutional block containing transactions and their execution results.
#[derive(Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub parent_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<TransactionReceipt>,
    pub state_root: [u8; 32],
    /// ADR-026: Merkle root of transactions in execution order.
    pub transactions_root: [u8; 32],
    /// ADR-027: Merkle root of execution receipts.
    pub receipts_root: [u8; 32],
    pub proposer: [u8; 32],
    pub timestamp: u64,
    /// N110.4: Slashing certificates included in this block
    pub slashing_certificates: Vec<amun_consensus_network::SlashingCertificate>,
    /// N120.2: Merkle root of the slashing ledger after this block
    pub slashing_root: [u8; 32],
    /// P2: Merkle root of constitutional evidence.
    pub evidence_root: [u8; 32],
}

impl Block {
    /// N110.4b: Verify all slashing certificates in this block.
    pub fn verify_slashing_certificates(&self) -> Result<(), String> {
        const MAX_CERTS_PER_BLOCK: usize = 10;

        if self.slashing_certificates.len() > MAX_CERTS_PER_BLOCK {
            return Err(format!(
                "Too many slashing certificates ({} > {})",
                self.slashing_certificates.len(),
                MAX_CERTS_PER_BLOCK
            ));
        }

        for (i, cert) in self.slashing_certificates.iter().enumerate() {
            cert.verify()
                .map_err(|e| format!("Certificate {} invalid: {}", i, e))?;

            let recomputed = cert.compute_hash();
            if recomputed != cert.certificate_hash {
                return Err(format!("Certificate {} hash mismatch", i));
            }

            if cert.evidence_ids.is_empty() {
                return Err(format!("Certificate {} has no evidence IDs", i));
            }
        }

        Ok(())
    }

    /// N120.3: Verify that the slashing_root matches the given ledger root.
    /// Returns Err if the block's slashing_root doesn't match.
    pub fn verify_slashing_root(&self, expected_root: &[u8; 32]) -> Result<(), String> {
        if self.slashing_root != *expected_root {
            return Err(format!(
                "N120.3: slashing_root mismatch: block={:02x?} expected={:02x?}",
                &self.slashing_root[..4],
                &expected_root[..4]
            ));
        }
        Ok(())
    }

    /// Compute the Blake3 block hash with domain separation.
    pub fn block_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_BLOCK_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.parent_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.transactions_root);
        hasher.update(&self.receipts_root);
        hasher.update(&self.proposer);
        hasher.update(&self.timestamp.to_le_bytes());
        for tx in &self.transactions {
            hasher.update(&tx.tx_hash());
        }
        // N110.4: Include slashing certificates in block hash
        for cert in &self.slashing_certificates {
            hasher.update(&cert.certificate_hash);
        }
        // N120.2: Include slashing root in block hash
        hasher.update(&self.slashing_root);
        hasher.update(&self.evidence_root);
        hasher.finalize().into()
    }
}

/// Builds constitutional blocks from mempool transactions.
pub struct BlockBuilder {
    pub engine: ExecutionEngine,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            engine: ExecutionEngine::new(),
        }
    }

    /// Build a block from the mempool, executing all transactions.
    /// N110.4a: Build a block with slashing certificates included.
    /// The certificates MUST have been verified before being passed here.
    pub fn build_block(
        &mut self,
        height: u64,
        parent_hash: [u8; 32],
        mempool: &mut Mempool,
        max_txs: usize,
        proposer: [u8; 32],
        timestamp: u64,
    ) -> Block {
        self.build_block_with_certificates(
            height,
            parent_hash,
            mempool,
            max_txs,
            proposer,
            timestamp,
            vec![],
            [0u8; 32],
        )
    }

    /// N110.4: Build a block with pending slashing certificates.
    /// N110.4a: Build a block with slashing certificates.
    /// Note: arguments kept as-is for clarity in consensus integration.
    #[allow(clippy::too_many_arguments)]
    pub fn build_block_with_certificates(
        &mut self,
        height: u64,
        parent_hash: [u8; 32],
        mempool: &mut Mempool,
        max_txs: usize,
        proposer: [u8; 32],
        timestamp: u64,
        slashing_certificates: Vec<SlashingCertificate>,
        evidence_root: [u8; 32],
    ) -> Block {
        let transactions = mempool.take_for_block(max_txs);
        let receipts = self.engine.execute_block(&transactions);
        let state_root = self.engine.state.state_root();

        // ADR-026 & ADR-027: Compute Merkle roots before moving data into Block
        let tx_hashes: Vec<[u8; 32]> = transactions.iter().map(|tx| tx.tx_hash()).collect();
        let tx_root = transactions_root(&tx_hashes);
        let receipt_hashes: Vec<[u8; 32]> = receipts.iter().map(|r| r.receipt_hash()).collect();
        let rec_root = receipts_root(&receipt_hashes);

        Block {
            height,
            parent_hash,
            transactions,
            receipts,
            state_root,
            transactions_root: tx_root,
            receipts_root: rec_root,
            proposer,
            timestamp,
            slashing_certificates,
            slashing_root: [0u8; 32],
            evidence_root, // N120.2: computed after ledger update
        }
    }

    /// Get a reference to the account store.
    pub fn account_store(&self) -> &AccountStore {
        &self.engine.state
    }
}

impl Default for BlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_transactions::{TransactionPayload, TransferPayload};
    use ed25519_dalek::SigningKey;

    fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
        let s = [seed; 32];
        let sk = SigningKey::from_bytes(&s);
        let sender = sk.verifying_key().to_bytes();
        let mut tx = Transaction {
            version: 1,
            sender,
            nonce,
            payload: TransactionPayload::Transfer(TransferPayload { to, amount }),
            signature: vec![],
        };
        tx.sign(&sk);
        tx
    }

    #[test]
    fn n27_build_block_with_transactions() {
        let mut builder = BlockBuilder::new();
        let mut mempool = Mempool::new();
        let s1 = [1u8; 32];
        let s2 = [3u8; 32];
        let a1 = {
            let sk = SigningKey::from_bytes(&s1);
            sk.verifying_key().to_bytes()
        };
        let a2 = {
            let sk = SigningKey::from_bytes(&s2);
            sk.verifying_key().to_bytes()
        };
        builder.engine.state.create_account(a1, 1000);
        builder.engine.state.create_account(a2, 500);
        mempool
            .add_transaction(create_signed_transfer(1, 1, 200, a2))
            .unwrap();
        mempool
            .add_transaction(create_signed_transfer(3, 1, 100, a1))
            .unwrap();
        let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
        assert_eq!(block.height, 1);
        assert_eq!(block.transactions.len(), 2);
        assert_eq!(block.receipts.len(), 2);
        assert!(block.receipts[0].success);
        assert!(block.receipts[1].success);
        assert_eq!(builder.engine.state.balance_of(&a1), 900);
        assert_eq!(builder.engine.state.balance_of(&a2), 600);
    }

    #[test]
    fn n27_block_hash_deterministic() {
        let mut builder = BlockBuilder::new();
        let mut mempool = Mempool::new();
        let sk = SigningKey::from_bytes(&[1u8; 32]);
        let a1 = sk.verifying_key().to_bytes();
        builder.engine.state.create_account(a1, 1000);
        mempool
            .add_transaction(create_signed_transfer(1, 1, 100, [2u8; 32]))
            .unwrap();
        let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
        let h1 = block.block_hash();
        let h2 = block.block_hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn n27_different_state_different_block_hash() {
        let mut b1 = BlockBuilder::new();
        let mut b2 = BlockBuilder::new();
        let a = {
            let sk = SigningKey::from_bytes(&[1u8; 32]);
            sk.verifying_key().to_bytes()
        };
        b1.engine.state.create_account(a, 1000);
        b2.engine.state.create_account(a, 999);
        let block1 = b1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
        let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
        assert_ne!(block1.block_hash(), block2.block_hash());
    }
}
