//! AmunChain History Root Computation
//!
//! ADR-024: Chain Commitment (history_root)
//!
//! This crate is the single source of truth for computing the
//! cumulative chain commitment across all finalized blocks.
//!
//! ## Specification
//!
//! ```text
//! HistoryRoot₀ = BLAKE3("AMUN_HISTORY_ROOT_V1" || [0x00; 32] || GenesisBlockHash)
//! HistoryRootₙ = BLAKE3("AMUN_HISTORY_ROOT_V1" || HistoryRootₙ₋₁ || BlockHashₙ)
//! ```

/// Domain separator for history root computation (ADR-024 §4.1).
pub const HISTORY_ROOT_DOMAIN: &[u8] = b"AMUN_HISTORY_ROOT_V1";

/// The predecessor hash for the genesis block (ADR-024 §4.3).
pub const GENESIS_PREDECESSOR: [u8; 32] = [0u8; 32];

/// Compute the history root for a new block.
///
/// This is the canonical reference implementation of ADR-024 §4.4.
///
/// # Arguments
/// * `prev_history_root` — The history root of the previous block.
/// * `block_hash` — The block hash of the current block.
///
/// # Returns
/// The new cumulative history root as a 32-byte array.
///
/// # Examples
/// ```
/// use amun_history::compute_history_root;
/// let prev = [1u8; 32];
/// let hash = [2u8; 32];
/// let root = compute_history_root(prev, hash);
/// assert_ne!(root, [0u8; 32]);
/// ```
#[inline]
pub fn compute_history_root(prev_history_root: [u8; 32], block_hash: [u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(HISTORY_ROOT_DOMAIN);
    hasher.update(&prev_history_root);
    hasher.update(&block_hash);
    hasher.finalize().into()
}

/// Compute the genesis history root.
///
/// Uses `[0x00; 32]` as the predecessor hash (ADR-024 §4.3).
///
/// # Arguments
/// * `genesis_block_hash` — The block hash of the genesis block.
///
/// # Returns
/// The genesis history root as a 32-byte array.
#[inline]
pub fn genesis_history_root(genesis_block_hash: [u8; 32]) -> [u8; 32] {
    compute_history_root(GENESIS_PREDECESSOR, genesis_block_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: compute a chain of N history roots given block hashes.
    fn compute_chain(block_hashes: &[[u8; 32]]) -> Vec<[u8; 32]> {
        let mut roots = Vec::with_capacity(block_hashes.len());
        let mut prev = GENESIS_PREDECESSOR;
        for &hash in block_hashes {
            let root = compute_history_root(prev, hash);
            roots.push(root);
            prev = root;
        }
        roots
    }

    #[test]
    fn test_history_root_determinism() {
        let prev = [1u8; 32];
        let hash = [2u8; 32];
        let r1 = compute_history_root(prev, hash);
        let r2 = compute_history_root(prev, hash);
        assert_eq!(r1, r2, "Same inputs must produce same output");
    }

    #[test]
    fn test_history_root_changes_with_input() {
        let prev = [1u8; 32];
        let h1 = compute_history_root(prev, [2u8; 32]);
        let h2 = compute_history_root(prev, [3u8; 32]);
        assert_ne!(h1, h2, "Different block hash must produce different root");
    }

    #[test]
    fn test_history_root_genesis() {
        let genesis_hash = [42u8; 32];
        let root = genesis_history_root(genesis_hash);
        // Same inputs must produce same genesis root
        let root2 = genesis_history_root(genesis_hash);
        assert_eq!(root, root2);
        // Different genesis hash must produce different root
        let root3 = genesis_history_root([43u8; 32]);
        assert_ne!(root, root3);
    }

    #[test]
    fn test_history_root_replay() {
        let hashes: Vec<[u8; 32]> = (0..10).map(|i| [i as u8; 32]).collect();
        let chain1 = compute_chain(&hashes);
        let chain2 = compute_chain(&hashes);
        assert_eq!(chain1, chain2, "Replay must produce identical chain");
    }

    #[test]
    fn test_history_root_chain_of_100() {
        let hashes: Vec<[u8; 32]> = (0..100).map(|i| [i as u8; 32]).collect();
        let chain = compute_chain(&hashes);
        assert_eq!(chain.len(), 100);
        // Every root must be different from its predecessor
        for i in 1..chain.len() {
            assert_ne!(
                chain[i],
                chain[i - 1],
                "Root at index {} must differ from previous",
                i
            );
        }
        // Every root must be non-zero
        for (i, root) in chain.iter().enumerate() {
            assert_ne!(*root, [0u8; 32], "Root at index {} must be non-zero", i);
        }
    }

    #[test]
    fn test_history_root_fork_divergence() {
        // Chain A: genesis -> A1 -> A2
        let genesis = [0u8; 32];
        let a1 = [1u8; 32];
        let a2 = [2u8; 32];
        let mut prev = GENESIS_PREDECESSOR;
        prev = compute_history_root(prev, genesis);
        prev = compute_history_root(prev, a1);
        let root_a = compute_history_root(prev, a2);

        // Chain B: genesis -> B1 -> B2 (different blocks)
        let b1 = [10u8; 32];
        let b2 = [20u8; 32];
        let mut prev = GENESIS_PREDECESSOR;
        prev = compute_history_root(prev, genesis);
        prev = compute_history_root(prev, b1);
        let root_b = compute_history_root(prev, b2);

        assert_ne!(
            root_a, root_b,
            "Forked chains must produce different final roots"
        );
    }

    #[test]
    fn test_history_root_domain_separation() {
        // Verify that the domain separator affects the output
        let prev = [1u8; 32];
        let hash = [2u8; 32];

        let mut hasher = blake3::Hasher::new();
        hasher.update(HISTORY_ROOT_DOMAIN);
        hasher.update(&prev);
        hasher.update(&hash);
        let with_domain: [u8; 32] = hasher.finalize().into();

        let mut hasher2 = blake3::Hasher::new();
        hasher2.update(&prev);
        hasher2.update(&hash);
        let without_domain: [u8; 32] = hasher2.finalize().into();

        assert_ne!(
            with_domain, without_domain,
            "Domain separator must change the hash output"
        );
    }

    #[test]
    fn test_history_root_throughput() {
        // 100,000 iterations must complete quickly without allocations
        let mut prev = GENESIS_PREDECESSOR;
        let hash = [42u8; 32];
        let start = std::time::Instant::now();
        for _ in 0..100_000 {
            prev = compute_history_root(prev, hash);
        }
        let elapsed = start.elapsed();
        // 100k BLAKE3 hashes should complete in well under 1 second
        assert!(
            elapsed.as_millis() < 1000,
            "100k history roots took {}ms, expected <1000ms",
            elapsed.as_millis()
        );
        assert_ne!(prev, [0u8; 32], "Final root must be non-zero");
    }
}
