// =============================================================================
// AmunChain Economic Constitution v1.0 — Frozen Constants
// =============================================================================
// DO NOT CHANGE these values without governance approval.
// All economic policies are derived from these constants.

use crate::BLOCKS_PER_EPOCH;

// -----------------------------------------------------------------------------
// Fee distribution (Basis Points, 100% = 10_000 BPS)
// -----------------------------------------------------------------------------
pub const TREASURY_BPS: u16 = 3500; // 35%
pub const VALIDATOR_BPS: u16 = 4000; // 40%
pub const ECOSYSTEM_BPS: u16 = 1500; // 15%
pub const BURN_BPS: u16 = 1000; // 10%

// -----------------------------------------------------------------------------
// Fee policy
// -----------------------------------------------------------------------------
pub const MINIMUM_FEE_NTR: u64 = 1; // Minimum fee per transaction
pub const MAXIMUM_FEE_NTR: u64 = 1_000_000; // Maximum fee (governance ceiling)

// -----------------------------------------------------------------------------
// Block reward (Minting)
// -----------------------------------------------------------------------------
pub const BLOCK_REWARD_NTR: u64 = 100; // Minted with every block

// -----------------------------------------------------------------------------
// Epoch policy
// -----------------------------------------------------------------------------
// Epoch does NOT mint new tokens; it only settles accumulated rewards.
// Epoch reward cap is derived from BLOCK_REWARD_NTR * BLOCKS_PER_EPOCH.
pub fn epoch_reward_cap() -> u64 {
    BLOCK_REWARD_NTR * BLOCKS_PER_EPOCH
}
