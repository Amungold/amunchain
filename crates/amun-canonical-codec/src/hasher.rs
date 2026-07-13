pub struct CanonicalHasher {
    hasher: blake3::Hasher,
}

impl Default for CanonicalHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalHasher {
    pub fn new() -> Self {
        Self {
            hasher: blake3::Hasher::new(),
        }
    }
    pub fn with_domain(domain: &[u8]) -> Self {
        let mut h = Self::new();
        h.update(domain);
        h
    }
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
    pub fn update_u64(&mut self, v: u64) {
        self.hasher.update(&v.to_le_bytes());
    }
    pub fn update_bool(&mut self, v: bool) {
        self.hasher.update(&[if v { 1 } else { 0 }]);
    }
    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}
