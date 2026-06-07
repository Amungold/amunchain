use amun_constitutional_block::ConstitutionalBlock;
use amun_proof_carrying::{ProofCarryingReceipt, ProofVerifier};
use crate::report::VerificationReport;

pub struct VerifierNode;

impl VerifierNode {
    pub fn verify_block(
        block: &ConstitutionalBlock,
        receipts: &[ProofCarryingReceipt],
        prev_block: Option<&ConstitutionalBlock>,
    ) -> VerificationReport {
        let mut report = VerificationReport::new(block.block_height, block.block_hash.clone());

        report.block_valid = block.compute_hash() == block.block_hash;

        report.lineage_valid = true;
        if let Some(prev) = prev_block {
            if block.parent_hash != prev.block_hash {
                report.lineage_valid = false;
            }
            if block.block_height != prev.block_height + 1 {
                report.lineage_valid = false;
            }
        } else {
            if block.block_height != 0 {
                report.lineage_valid = false;
            }
            if block.parent_hash != "0".repeat(64) {
                report.lineage_valid = false;
            }
        }

        report.state_proofs_valid = true;
        report.governance_proofs_valid = true;
        report.execution_proofs_valid = true;

        for receipt in receipts {
            if receipt.block_hash != block.block_hash {
                report.lineage_valid = false;
            }
            if receipt.state_root != block.state_root {
                report.state_proofs_valid = false;
            }
            if receipt.governance_root != block.governance_root {
                report.governance_proofs_valid = false;
            }
            if receipt.execution_root != block.execution_root {
                report.execution_proofs_valid = false;
            }
            if ProofVerifier::verify_receipt(receipt).is_err() {
                report.state_proofs_valid = false;
            }
        }

        report.finalize();
        report
    }

    pub fn verify_chain(
        blocks: &[ConstitutionalBlock],
        receipts_per_block: &[Vec<ProofCarryingReceipt>],
    ) -> Vec<VerificationReport> {
        let mut reports = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            let prev = if i > 0 { Some(&blocks[i - 1]) } else { None };
            let receipts = receipts_per_block.get(i).map(|v| v.as_slice()).unwrap_or(&[]);
            reports.push(Self::verify_block(block, receipts, prev));
        }
        reports
    }
}
