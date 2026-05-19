use crate::receipt::ExecutionReceipt;
use amun_chain_position::ChainPosition;
use blake3::Hasher;

const MAX_JOURNAL: usize = 100_000;

#[derive(Debug, Clone)]
pub struct StorageJournal { 
    pub entries: Vec<JournalEntry>,
    current_epoch: u64,
}

#[derive(Debug, Clone)]
pub struct JournalEntry { 
    pub receipt: ExecutionReceipt, 
    pub journal_hash: [u8; 32], 
    pub previous_journal_hash: [u8; 32],
    pub epoch: u64,
}

#[derive(Debug)]
pub enum JournalError { 
    ReceiptChainBroken { position: ChainPosition, expected_root: [u8; 32], actual_root: [u8; 32] }, 
    JournalFull,
}

impl StorageJournal {
    pub fn new(_: [u8; 32]) -> Self { 
        Self { entries: Vec::with_capacity(MAX_JOURNAL), current_epoch: 0 } 
    }

    pub fn set_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
    }

    pub fn append(&mut self, receipt: ExecutionReceipt) -> Result<(), JournalError> {
        if self.entries.len() >= MAX_JOURNAL { return Err(JournalError::JournalFull); }
        
        let prev_hash = self.entries.last().map(|e| e.journal_hash).unwrap_or([0u8; 32]);

        // Check receipt continuity: only within the same epoch.
        // Epoch boundaries (seals) reset the receipt chain.
        if let Some(last) = self.entries.last() {
            if last.epoch == self.current_epoch {
                if receipt.from_root != last.receipt.to_root {
                    return Err(JournalError::ReceiptChainBroken {
                        position: receipt.position,
                        expected_root: last.receipt.to_root,
                        actual_root: receipt.from_root,
                    });
                }
            }
            // Cross-epoch: seal changed state, receipt chain is reset - this is valid
        }

        let mut h = Hasher::new();
        h.update(b"AMUN_JOURNAL_V3");
        h.update(&receipt.hash());
        h.update(&prev_hash);
        h.update(&self.current_epoch.to_le_bytes());
        let mut jh = [0u8; 32];
        jh.copy_from_slice(&h.finalize().as_bytes()[..32]);

        self.entries.push(JournalEntry {
            receipt, 
            journal_hash: jh,
            previous_journal_hash: prev_hash,
            epoch: self.current_epoch,
        });
        Ok(())
    }

    pub fn verify_continuity(&self) -> bool {
        if self.entries.is_empty() { return true; }
        let mut prev_hash = [0u8; 32];
        let mut prev_to = self.entries[0].receipt.from_root;
        let mut prev_epoch = self.entries[0].epoch;

        for e in &self.entries {
            if e.previous_journal_hash != prev_hash { return false; }
            // Receipt chain only checked within same epoch
            if e.epoch == prev_epoch {
                if e.receipt.from_root != prev_to { return false; }
            }
            prev_hash = e.journal_hash;
            prev_to = e.receipt.to_root;
            prev_epoch = e.epoch;
        }
        true
    }

    pub fn chain_hash(&self) -> [u8; 32] { 
        self.entries.last().map(|e| e.journal_hash).unwrap_or([0u8; 32]) 
    }
}
