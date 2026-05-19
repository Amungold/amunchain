// Quorum transition safety theorem parameters.

#[derive(Clone, Copy, Debug)]
pub struct QuorumTransitionParameters {
    pub old_set_size: u32,
    pub new_set_size: u32,
    pub overlap_size: u32,
}

impl QuorumTransitionParameters {
    const fn byzantine_threshold(n: u32) -> u32 {
        if n == 0 {
            0
        } else {
            (n - 1) / 3
        }
    }

    pub fn verify_safety(&self) -> Result<(), &'static str> {
        let f_old = Self::byzantine_threshold(self.old_set_size);
        let f_new = Self::byzantine_threshold(self.new_set_size);
        let required = f_old.max(f_new) + 1;
        if self.overlap_size >= required {
            Ok(())
        } else {
            Err("Insufficient quorum overlap for safe transition")
        }
    }
}
