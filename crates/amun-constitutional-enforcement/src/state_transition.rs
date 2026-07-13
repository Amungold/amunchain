// N126 — Constitutional State Transition Verification
// ====================================================
// Verifies that state transitions are deterministic and
// constitutionally valid. Replaces the `transition_valid: bool`
// parameter with real execution and replay verification.

use crate::{ConstitutionalLaw, ConstitutionalVerdict, ConstitutionalViolation};

/// N126: Result of verifying a state transition.
#[derive(Debug, Clone)]
pub struct StateTransitionResult {
    /// The pre-state root before the transition
    pub pre_state_root: [u8; 32],
    /// The post-state root after execution
    pub post_state_root: [u8; 32],
    /// The replay state root (independently computed)
    pub replay_root: [u8; 32],
    /// Whether supply is conserved
    pub supply_conserved: bool,
    /// Total supply before
    pub pre_supply: u64,
    /// Total supply after
    pub post_supply: u64,
}

impl StateTransitionResult {
    /// N126: Create from the raw data produced by execution and replay.
    pub fn new(
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        replay_root: [u8; 32],
        pre_supply: u64,
        post_supply: u64,
    ) -> Self {
        Self {
            pre_state_root,
            post_state_root,
            replay_root,
            supply_conserved: pre_supply == post_supply,
            pre_supply,
            post_supply,
        }
    }

    /// N126: Verify the state transition constitutionally.
    pub fn verify(&self, height: u64) -> ConstitutionalVerdict {
        let mut violations = Vec::new();

        // N126.1: State transition must change state (empty blocks are valid but rare)
        if self.pre_state_root == self.post_state_root && height > 0 {
            // Empty block is constitutional — no violation
        }

        // N126.2: Replay must produce identical state root
        if self.post_state_root != self.replay_root {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::ReplayDeterminism,
                description: format!(
                    "Replay divergence: execution={:02x?} replay={:02x?}",
                    &self.post_state_root[..4],
                    &self.replay_root[..4]
                ),
                height,
            });
        }

        // N126.3: Supply conservation (minting only via constitutional governance)
        if !self.supply_conserved {
            violations.push(ConstitutionalViolation {
                law: ConstitutionalLaw::StateTransitionValidity,
                description: format!(
                    "Supply not conserved: pre={} post={}",
                    self.pre_supply, self.post_supply
                ),
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

/// N126: Execute and verify a state transition constitutionally.
///
/// This function takes the pre-state, executes a block, replays it,
/// and produces a constitutional verdict.
///
/// `execute_fn` is the actual execution function that produces post_state_root.
/// `replay_fn` is the independent replay that must produce the same root.
/// `supply_fn` returns total supply before and after.
pub fn verify_state_transition<F, R, S>(
    height: u64,
    pre_state_root: [u8; 32],
    execute_fn: F,
    replay_fn: R,
    supply_fn: S,
) -> ConstitutionalVerdict
where
    F: FnOnce() -> [u8; 32],
    R: FnOnce() -> [u8; 32],
    S: FnOnce() -> (u64, u64),
{
    let post_state_root = execute_fn();
    let replay_root = replay_fn();
    let (pre_supply, post_supply) = supply_fn();

    let result = StateTransitionResult::new(
        pre_state_root,
        post_state_root,
        replay_root,
        pre_supply,
        post_supply,
    );

    result.verify(height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n126_identical_execution_and_replay_is_constitutional() {
        let pre_root = [0x11; 32];
        let post_root = [0x22; 32];

        let verdict = verify_state_transition(
            100,
            pre_root,
            || post_root,
            || post_root,          // replay matches
            || (100_000, 100_000), // supply conserved
        );
        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
    }

    #[test]
    fn n126_replay_divergence_is_unconstitutional() {
        let pre_root = [0x11; 32];
        let post_root = [0x22; 32];
        let wrong_replay = [0xFF; 32];

        let verdict = verify_state_transition(
            100,
            pre_root,
            || post_root,
            || wrong_replay, // replay differs!
            || (100_000, 100_000),
        );
        match verdict {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert!(violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::ReplayDeterminism));
            }
            _ => panic!("Expected Unconstitutional for replay divergence"),
        }
    }

    #[test]
    fn n126_supply_not_conserved_is_unconstitutional() {
        let root = [0x22; 32];
        let verdict = verify_state_transition(
            100,
            [0x11; 32],
            || root,
            || root,
            || (100_000, 95_000), // supply decreased without authorization
        );
        match verdict {
            ConstitutionalVerdict::Unconstitutional { violations } => {
                assert!(violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::StateTransitionValidity));
            }
            _ => panic!("Expected Unconstitutional for supply change"),
        }
    }
}
