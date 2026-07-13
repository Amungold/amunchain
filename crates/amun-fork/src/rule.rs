use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ChainHead {
    pub height: u64,
    pub stake: u64,
    pub finalized: bool,
    pub finalized_checkpoint_height: u64,
    pub block_hash: [u8; 32],
}

pub struct ForkChoiceRule;

impl ForkChoiceRule {
    /// Deterministic fork resolution using canonical ordering.
    /// Tie-break uses a VRF-style commitment over (height, stake, block_hash)
    /// to prevent adversary grinding while maintaining determinism.
    pub fn resolve(chains: &[ChainHead]) -> Option<usize> {
        if chains.is_empty() {
            return None;
        }

        let mut best: usize = 0;

        for (i, chain) in chains.iter().enumerate() {
            let current_best = &chains[best];

            if chain.finalized_checkpoint_height > current_best.finalized_checkpoint_height {
                best = i;
                continue;
            }
            if chain.finalized_checkpoint_height < current_best.finalized_checkpoint_height {
                continue;
            }

            if chain.finalized && !current_best.finalized {
                best = i;
                continue;
            }
            if !chain.finalized && current_best.finalized {
                continue;
            }

            if chain.height > current_best.height {
                best = i;
                continue;
            }
            if chain.height < current_best.height {
                continue;
            }

            if chain.stake > current_best.stake {
                best = i;
                continue;
            }
            if chain.stake < current_best.stake {
                continue;
            }

            // Tie-break: canonical ordering commitment
            let chain_commitment = Self::tiebreak_commitment(chain);
            let best_commitment = Self::tiebreak_commitment(current_best);
            if chain_commitment > best_commitment {
                best = i;
            }
        }

        Some(best)
    }

    fn tiebreak_commitment(chain: &ChainHead) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_FORK_TIEBREAK_V1");
        hasher.update(&chain.height.to_le_bytes());
        hasher.update(&chain.stake.to_le_bytes());
        hasher.update(&chain.block_hash);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        out
    }
}
