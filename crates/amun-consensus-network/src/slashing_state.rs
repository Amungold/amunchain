// N121.1 — Slashing State Integration
// ====================================
// Elevates the slashing ledger from a sidecar structure to a
// first-class consensus state component with its own Merkle root.
// This root is committed alongside state_root and history_root.

use crate::slashing_ledger::{ExecutedSlash, SlashingLedger};
use crate::slashing_merkle::merkle_root;

/// N121.1: Consensus-level slashing state.
/// Wraps the deterministic ledger and exposes a root for consensus commitment.
#[derive(Debug, Clone)]
pub struct SlashingState {
    pub ledger: SlashingLedger,
    pub root: [u8; 32],
}

impl SlashingState {
    /// Create a new slashing state with an empty ledger.
    pub fn new() -> Self {
        let ledger = SlashingLedger::new();
        let root = merkle_root(&ledger.history);
        Self { ledger, root }
    }

    /// Execute a slash and update the root atomically.
    pub fn execute<F, T>(
        &mut self,
        cert: &crate::slashing_certificate::SlashingCertificate,
        execute_fn: F,
    ) -> Result<T, String>
    where
        F: FnOnce() -> Result<T, String>,
    {
        let result = self.ledger.execute(cert, execute_fn)?;
        // N121.1: Update the root after each execution
        self.root = merkle_root(&self.ledger.history);
        Ok(result)
    }

    /// Verify that the internal root matches the ledger.
    pub fn verify_consistency(&self) -> Result<(), String> {
        let recomputed = merkle_root(&self.ledger.history);
        if recomputed != self.root {
            return Err("N121.1: slashing state root mismatch".into());
        }
        Ok(())
    }

    /// Get the number of executed slashes.
    pub fn executed_count(&self) -> usize {
        self.ledger.executed_count()
    }

    /// Get the audit trail.
    pub fn history(&self) -> &[ExecutedSlash] {
        &self.ledger.history
    }
}

impl Default for SlashingState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
    use crate::ValidatorStatus;

    fn make_cert() -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            [0x42; 32],
            30,
            vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
            vec![EvidenceCount {
                evidence_type: EvidenceType::DoubleVote,
                count: 3,
                weight: 30,
            }],
            1500,
            15000,
            85000,
            3,
            ValidatorStatus::SlashEligible,
            100,
        )
    }

    #[test]
    fn n121_1_initial_state_has_zero_root() {
        let state = SlashingState::new();
        assert_eq!(state.root, [0u8; 32]);
        assert_eq!(state.executed_count(), 0);
        assert!(state.verify_consistency().is_ok());
    }

    #[test]
    fn n121_1_root_updates_after_execution() {
        let mut state = SlashingState::new();
        let cert = make_cert();
        let root_before = state.root;

        state.execute(&cert, || Ok(())).unwrap();

        assert_ne!(
            state.root, root_before,
            "N121.1 FAIL: root must change after execution"
        );
        assert_ne!(
            state.root, [0u8; 32],
            "N121.1 FAIL: root must be non-zero after execution"
        );
        assert!(state.verify_consistency().is_ok());
        assert_eq!(state.executed_count(), 1);
    }

    #[test]
    fn n121_1_multiple_executions_update_root() {
        let mut state = SlashingState::new();
        let cert1 = make_cert();
        let cert2 = SlashingCertificate::from_slash_result(
            [0x99; 32],
            20,
            vec![[0xB1; 32]],
            vec![],
            500,
            5000,
            95000,
            1,
            ValidatorStatus::Warned,
            200,
        );

        state.execute(&cert1, || Ok(())).unwrap();
        let root_after_1 = state.root;
        assert_eq!(state.executed_count(), 1);

        state.execute(&cert2, || Ok(())).unwrap();
        let root_after_2 = state.root;
        assert_eq!(state.executed_count(), 2);

        assert_ne!(
            root_after_1, root_after_2,
            "N121.1 FAIL: root must change with each execution"
        );
        assert!(state.verify_consistency().is_ok());
    }

    #[test]
    fn n121_1_replay_protection_preserved() {
        let mut state = SlashingState::new();
        let cert = make_cert();

        state.execute(&cert, || Ok(())).unwrap();
        let root_after_first = state.root;

        // Second execution of same certificate must fail
        let result = state.execute(&cert, || Ok(()));
        assert!(result.is_err(), "N121.1 FAIL: replay must be rejected");
        assert!(result.unwrap_err().contains("already executed"));

        // Root must not change after failed execution
        assert_eq!(
            state.root, root_after_first,
            "N121.1 FAIL: root must not change on replay"
        );
        assert_eq!(state.executed_count(), 1);
    }
}
