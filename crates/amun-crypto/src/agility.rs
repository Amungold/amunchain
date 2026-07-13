#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignatureScheme {
    Ed25519,
}

pub struct CryptoAgility {
    pub current: SignatureScheme,
    pub migration_epoch: Option<u64>,
}

impl CryptoAgility {
    pub fn new() -> Self {
        Self {
            current: SignatureScheme::Ed25519,
            migration_epoch: None,
        }
    }

    pub fn schedule_migration(&mut self, target: SignatureScheme, activation_epoch: u64) {
        self.migration_epoch = Some(activation_epoch);
        self.current = target;
    }

    pub fn check_migration(&mut self, current_epoch: u64) -> bool {
        if let Some(epoch) = self.migration_epoch {
            if current_epoch >= epoch {
                self.migration_epoch = None;
                return true;
            }
        }
        false
    }
}

impl Default for CryptoAgility {
    fn default() -> Self {
        Self::new()
    }
}
