use std::collections::HashSet;

pub struct RevocationList {
    pub revoked: HashSet<[u8; 32]>,
}

impl RevocationList {
    pub fn new() -> Self {
        Self {
            revoked: HashSet::new(),
        }
    }

    pub fn revoke(&mut self, fingerprint: &[u8; 32]) {
        self.revoked.insert(*fingerprint);
    }

    pub fn is_revoked(&self, fingerprint: &[u8; 32]) -> bool {
        self.revoked.contains(fingerprint)
    }
}

impl Default for RevocationList {
    fn default() -> Self {
        Self::new()
    }
}
