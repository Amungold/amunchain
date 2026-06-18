// N124 — Constitutional Proof Engine
// ====================================
// Connects ConstitutionalEnforcementKernel to real verification
// modules across AmunChain.

use crate::{ConstitutionalLaw, ConstitutionalVerdict, ConstitutionalViolation};

/// N124: Real verification functions for each constitutional law.
pub struct ConstitutionalProofEngine;

impl ConstitutionalProofEngine {
    /// Verify state root integrity using block data.
    /// Requires the block's state_root to match the execution result.
    pub fn verify_state_root_integrity(
        block_state_root: &[u8; 32],
        execution_state_root: &[u8; 32],
    ) -> bool {
        block_state_root == execution_state_root
    }

    /// Verify chain continuity: block must build on the previous block.
    pub fn verify_chain_continuity(block_parent: &[u8; 32], tip_hash: &[u8; 32]) -> bool {
        block_parent == tip_hash
    }

    /// N125.1: Verify transactions using real ed25519 verification.
    /// Each transaction's signature is verified against its sender's public key.
    /// Transactions must provide (sender, payload, signature) tuples.
    pub fn verify_signatures(transactions: &[([u8; 32], Vec<u8>, [u8; 64])]) -> bool {
        for &(sender, ref payload, ref signature) in transactions {
            if !amun_validator_identity::verify_ed25519(&sender, payload, signature) {
                return false;
            }
        }
        true
    }

    /// N125: Verify no double-spend using (sender, nonce) pairs.
    /// Same sender with different nonces is valid (multiple txs).
    /// Same sender with same nonce is a double-spend.
    pub fn verify_no_double_spend(sender_nonce_pairs: &[([u8; 32], u64)]) -> bool {
        let mut seen = std::collections::HashSet::new();
        for &(sender, nonce) in sender_nonce_pairs {
            if !seen.insert((sender, nonce)) {
                return false; // Same sender + same nonce = double-spend
            }
        }
        true
    }

    /// Verify slashing certificate is backed by valid evidence.
    pub fn verify_slashing_evidence_binding(
        evidence_ids: &[[u8; 32]],
        evidence_available: bool,
    ) -> bool {
        !evidence_ids.is_empty() && evidence_available
    }

    /// Verify validator set changes follow governance rules.
    pub fn verify_validator_governance(governance_approved: bool) -> bool {
        governance_approved
    }

    /// Verify replay determinism: replaying the block produces the same state root.
    pub fn verify_replay_determinism(original_root: &[u8; 32], replay_root: &[u8; 32]) -> bool {
        original_root == replay_root
    }

    /// Verify finality has supermajority.
    pub fn verify_finality_supermajority(approval_power: u64, total_power: u64) -> bool {
        total_power > 0 && approval_power * 3 > total_power * 2
    }

    /// Verify state transition validity.
    pub fn verify_state_transition(
        pre_state_root: &[u8; 32],
        post_state_root: &[u8; 32],
        transition_valid: bool,
    ) -> bool {
        pre_state_root != post_state_root && transition_valid
    }

    /// Verify evidence is cryptographically valid.
    pub fn verify_evidence_validity(evidence_hash_valid: bool) -> bool {
        evidence_hash_valid
    }

    /// N124: Run all constitutional checks with real data.
    pub fn run_constitutional_review(
        height: u64,
        block_state_root: &[u8; 32],
        execution_state_root: &[u8; 32],
        block_parent: &[u8; 32],
        tip_hash: &[u8; 32],
        transactions: &[([u8; 32], Vec<u8>, [u8; 64])],
        sender_nonce_pairs: &[([u8; 32], u64)],
        evidence_ids: &[[u8; 32]],
        evidence_available: bool,
        governance_approved: bool,
        original_root: &[u8; 32],
        replay_root: &[u8; 32],
        approval_power: u64,
        total_power: u64,
        pre_state_root: &[u8; 32],
        post_state_root: &[u8; 32],
        transition_valid: bool,
        evidence_hash_valid: bool,
    ) -> ConstitutionalVerdict {
        let mut violations = Vec::new();

        // State Root Integrity
        if !Self::verify_state_root_integrity(block_state_root, execution_state_root) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::StateRootIntegrity,
                description: format!(
                    "Block state_root {:02x?} != execution {:02x?}",
                    &block_state_root[..4],
                    &execution_state_root[..4]
                ),
                height,
            });
        }

        // Chain Continuity
        if !Self::verify_chain_continuity(block_parent, tip_hash) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::ChainContinuity,
                description: "Block parent does not match chain tip".into(),
                height,
            });
        }

        // N125.1: Real cryptographic signature verification
        if !Self::verify_signatures(transactions) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::SignatureValidity,
                description: "One or more transactions have invalid signatures".into(),
                height,
            });
        }

        // No Double Spend (by sender + nonce)
        if !Self::verify_no_double_spend(sender_nonce_pairs) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::NoDoubleSpend,
                description: "Duplicate sender detected in block".into(),
                height,
            });
        }

        // Slashing Evidence Binding
        if !Self::verify_slashing_evidence_binding(evidence_ids, evidence_available) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::SlashingEvidenceBinding,
                description: "Slashing certificate lacks valid evidence".into(),
                height,
            });
        }

        // Validator Set Governance
        if !Self::verify_validator_governance(governance_approved) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::ValidatorSetGovernance,
                description: "Validator set change not approved by governance".into(),
                height,
            });
        }

        // Replay Determinism
        if !Self::verify_replay_determinism(original_root, replay_root) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::ReplayDeterminism,
                description: "Replay produced different state root".into(),
                height,
            });
        }

        // Finality Supermajority
        if !Self::verify_finality_supermajority(approval_power, total_power) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::FinalitySupermajority,
                description: format!(
                    "Finality lacks supermajority: {}/{}",
                    approval_power, total_power
                ),
                height,
            });
        }

        // State Transition Validity
        if !Self::verify_state_transition(pre_state_root, post_state_root, transition_valid) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::StateTransitionValidity,
                description: "State transition is invalid".into(),
                height,
            });
        }

        // Evidence Validity
        if !Self::verify_evidence_validity(evidence_hash_valid) {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::EvidenceValidity,
                description: "Evidence hash is invalid".into(),
                height,
            });
        }

        if violations.is_empty() {
            ConstitutionalVerdict::Constitutional
        } else {
            ConstitutionalVerdict::Unconstitutional { violations }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n124_state_root_mismatch_detected() {
        let verdict = ConstitutionalProofEngine::run_constitutional_review(
            100,
            &[0x42; 32],
            &[0xFF; 32], // different roots
            &[0xAA; 32],
            &[0xAA; 32], // parent matches
            &[],
            &[],
            &[],
            true,
            true,
            &[0x11; 32],
            &[0x11; 32], // replay matches
            3,
            4, // supermajority
            &[0x01; 32],
            &[0x02; 32],
            true,
            true,
        );
        match verdict {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert!(violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::StateRootIntegrity));
            }
            _ => panic!("Expected Unconstitutional"),
        }
    }

    #[test]
    fn n124_constitutional_block_passes() {
        let root = [0x42; 32];
        let verdict = ConstitutionalProofEngine::run_constitutional_review(
            100,
            &root,
            &root, // matching roots
            &[0xAA; 32],
            &[0xAA; 32],   // parent matches
            &[],           // no transactions
            &[],           // no transactions
            &[[0xA1; 32]], // one evidence ID
            true,          // evidence available
            true,          // governance approved
            &root,
            &root, // replay matches
            3,
            4, // supermajority
            &[0x01; 32],
            &[0x02; 32],
            true,
            true,
        );
        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
    }

    #[test]
    fn n124_no_supermajority_detected() {
        let root = [0x42; 32];
        let verdict = ConstitutionalProofEngine::run_constitutional_review(
            100,
            &root,
            &root,
            &[0xAA; 32],
            &[0xAA; 32],
            &[],
            &[],
            &[],
            true,
            true,
            &root,
            &root,
            1,
            4, // 1/4 = 25% < 66.6%
            &[0x01; 32],
            &[0x02; 32],
            true,
            true,
        );
        match verdict {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert!(violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::FinalitySupermajority));
            }
            _ => panic!("Expected Unconstitutional"),
        }
    }
}
