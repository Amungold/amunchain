use amun_kernel_types::PublicHash32;
use amun_failure::AmunResult;
use heapless::Vec;

pub struct StfState {
    pub state_root: PublicHash32,
    pub block_height: u64,
    pub dirty: bool,
    pending_keys: Vec<(Vec<u8,32>, Option<Vec<u8,32>>), 64>,
}

impl StfState {
    pub fn new(root: PublicHash32, height: u64) -> Self {
        Self { state_root: root, block_height: height, dirty: false, pending_keys: Vec::new() }
    }

    pub fn apply_set(&mut self, key: Vec<u8,32>, value: Vec<u8,32>) -> AmunResult<()> {
        self.pending_keys.push((key, Some(value))).map_err(|_| amun_failure::FailureContext::new(amun_failure::ConstitutionalFault::CapacityExceeded, 0x000F, 0x0001))?;
        self.dirty = true;
        Ok(())
    }

    pub fn apply_delete(&mut self, key: Vec<u8,32>) -> AmunResult<()> {
        self.pending_keys.push((key, None)).map_err(|_| amun_failure::FailureContext::new(amun_failure::ConstitutionalFault::CapacityExceeded, 0x000F, 0x0002))?;
        self.dirty = true;
        Ok(())
    }

    pub fn commit(&mut self) -> AmunResult<PublicHash32> {
        if !self.dirty { return Ok(self.state_root); }
        self.state_root = self.compute_root();
        self.block_height = self.block_height.saturating_add(1);
        self.pending_keys.clear();
        self.dirty = false;
        Ok(self.state_root)
    }

    pub fn rollback(&mut self) {
        self.pending_keys.clear();
        self.dirty = false;
    }

    fn compute_root(&self) -> PublicHash32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.state_root.0);
        hasher.update(&self.block_height.to_le_bytes());
        for (k, v) in &self.pending_keys {
            hasher.update(k);
            if let Some(val) = v { hasher.update(val); } else { hasher.update(b"DEL"); }
        }
        let h = hasher.finalize();
        let mut r = PublicHash32::default();
        r.0.copy_from_slice(&h.as_bytes()[..32]);
        r
    }
}
