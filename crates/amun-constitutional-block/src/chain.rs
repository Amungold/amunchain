// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::block::ConstitutionalBlock;

#[derive(Debug, Clone, Default)]
pub struct Blockchain {
    pub blocks: Vec<ConstitutionalBlock>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn append(&mut self, block: ConstitutionalBlock) -> Result<(), String> {
        if self.blocks.is_empty() {
            if block.block_height != 0 {
                return Err("Genesis block must have height 0".into());
            }
            if block.parent_hash != "0".repeat(64) {
                return Err("Genesis parent hash must be 64 zeros".into());
            }
        } else {
            let last = self.blocks.last().unwrap();
            if block.parent_hash != last.block_hash {
                return Err(format!(
                    "Parent hash mismatch: expected {} got {}",
                    last.block_hash, block.parent_hash
                ));
            }
            if block.block_height != last.block_height + 1 {
                return Err(format!(
                    "Block height non-monotonic: expected {} got {}",
                    last.block_height + 1,
                    block.block_height
                ));
            }
        }
        self.blocks.push(block);
        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        for block in &self.blocks {
            let recomputed = block.compute_hash();
            if recomputed != block.block_hash {
                return Err(format!(
                    "Block hash mismatch at height {}: expected {} got {}",
                    block.block_height, block.block_hash, recomputed
                ));
            }
        }
        Ok(())
    }
}

impl Blockchain {
    /// Verify that a block's evidence_root matches the given ActionLog.
    pub fn verify_block_evidence(&self, height: u64, log: &amun_consensus::action::ActionLog) -> Result<(), String> {
        let block = self.blocks.get(height as usize)
            .ok_or_else(|| format!("Block not found at height {}", height))?;
        let computed = hex::encode(log.evidence_root());
        if block.evidence_root != computed {
            return Err(format!(
                "Evidence root mismatch at height {}: block has {} but log produces {}",
                height, block.evidence_root, computed
            ));
        }
        Ok(())
    }

    /// Full chain evidence audit — verify every block against its ActionLog.
    pub fn verify_chain_evidence(&self, logs: &[amun_consensus::action::ActionLog]) -> Result<(), String> {
        if logs.len() != self.blocks.len() {
            return Err(format!("Log count {} != block count {}", logs.len(), self.blocks.len()));
        }
        for (i, log) in logs.iter().enumerate() {
            self.verify_block_evidence(i as u64, log)?;
        }
        Ok(())
    }
}
