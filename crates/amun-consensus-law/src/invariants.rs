use amun_invariants::kernel::IrreducibleInvariants;

pub struct ConsensusInvariants;

impl ConsensusInvariants {
    pub fn single_truth_holds(state_roots: &[[u8; 32]]) -> bool {
        if state_roots.len() <= 1 {
            return true;
        }
        let reference = state_roots[0];
        state_roots.iter().all(|r| *r == reference)
    }

    pub fn single_finality_holds(finalized: &[(u64, [u8; 32])], f: u64, n: u64) -> bool {
        if f > n / 3 {
            return false;
        }
        for i in 0..finalized.len() {
            for j in (i + 1)..finalized.len() {
                let (h1, b1) = finalized[i];
                let (h2, b2) = finalized[j];
                if h1 == h2 && b1 != b2 {
                    return false;
                }
            }
        }
        true
    }

    pub fn eventual_progress_possible(active_validators: usize, threshold: usize) -> bool {
        active_validators >= threshold
    }

    pub fn kernel_bound() -> [u8; 32] {
        IrreducibleInvariants::kernel_hash()
    }
}
