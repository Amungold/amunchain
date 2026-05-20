pub struct KeyRotation {
    pub current_epoch: u64,
    pub rotation_epoch: u64,
    pub rotated: bool,
    pub old_public_keys: heapless::Vec<([u8; 32], u64), 4>,
    pub max_key_age_epochs: u64,
}

impl KeyRotation {
    pub fn new(rotation_epoch: u64) -> Self {
        Self {
            current_epoch: 0,
            rotation_epoch,
            rotated: false,
            old_public_keys: heapless::Vec::new(),
            max_key_age_epochs: 1000,
        }
    }

    pub fn check(&mut self, epoch: u64, current_pk: [u8; 32]) -> bool {
        self.current_epoch = epoch;
        if epoch >= self.rotation_epoch && !self.rotated {
            let _ = self.old_public_keys.push((current_pk, epoch));
            self.rotated = true;
            return true;
        }
        false
    }

    pub fn is_legacy_key(&self, pk: &[u8; 32], current_epoch: u64) -> bool {
        self.old_public_keys
            .iter()
            .any(|(old, epoch)| old == pk && current_epoch - epoch <= self.max_key_age_epochs)
    }
}
