pub struct AtomicCheckpoint {
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub committed: bool,
}

impl AtomicCheckpoint {
    pub fn new(epoch: u64, root: [u8; 32]) -> Self {
        Self {
            epoch,
            state_root: root,
            committed: false,
        }
    }

    pub fn commit(&mut self) {
        self.committed = true;
    }

    pub fn is_committed(&self) -> bool {
        self.committed
    }
}
