use sha2::{Digest, Sha256};

const HASHER_DOMAIN: &[u8] = b"AMUN|CONSTITUTIONAL_HASHER|V1";

#[derive(Clone)]
pub struct ConstitutionalHasher {
    inner: Sha256,
}

impl ConstitutionalHasher {
    pub fn new(object_type: &[u8]) -> Self {
        let mut inner = Sha256::new();
        inner.update(HASHER_DOMAIN);
        inner.update(object_type);
        Self { inner }
    }

    pub fn update_schema(&mut self, id: u16, version: u16) -> &mut Self {
        self.inner.update(id.to_le_bytes());
        self.inner.update(version.to_le_bytes());
        self
    }
    pub fn update_revision(&mut self, constitutional: u32, replay: u32) -> &mut Self {
        self.inner.update(constitutional.to_le_bytes());
        self.inner.update(replay.to_le_bytes());
        self
    }
    pub fn update_identity(&mut self, id: u64) -> &mut Self {
        self.inner.update(id.to_le_bytes());
        self
    }
    pub fn update_u64(&mut self, v: u64) -> &mut Self {
        self.inner.update(v.to_le_bytes());
        self
    }
    pub fn update_u32(&mut self, v: u32) -> &mut Self {
        self.inner.update(v.to_le_bytes());
        self
    }
    pub fn update_u16(&mut self, v: u16) -> &mut Self {
        self.inner.update(v.to_le_bytes());
        self
    }
    pub fn update_u8(&mut self, v: u8) -> &mut Self {
        self.inner.update([v]);
        self
    }
    pub fn update_bytes(&mut self, v: &[u8]) -> &mut Self {
        self.inner.update(v);
        self
    }

    pub fn update_optional_u64(&mut self, v: Option<u64>) -> &mut Self {
        if let Some(x) = v {
            self.update_u64(x);
        }
        self
    }
    pub fn update_optional_hash(&mut self, v: Option<&[u8; 32]>) -> &mut Self {
        if let Some(x) = v {
            self.update_bytes(x);
        }
        self
    }
    pub fn update_optional_bytes(&mut self, v: Option<&[u8]>) -> &mut Self {
        if let Some(x) = v {
            self.update_bytes(x);
        }
        self
    }

    pub fn finalize(self) -> [u8; 32] {
        self.inner.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let h1 = ConstitutionalHasher::new(b"TEST")
            .update_u64(42)
            .clone()
            .finalize();
        let h2 = ConstitutionalHasher::new(b"TEST")
            .update_u64(42)
            .clone()
            .finalize();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_domain_separation() {
        let h1 = ConstitutionalHasher::new(b"TYPE_A")
            .update_u64(1)
            .clone()
            .finalize();
        let h2 = ConstitutionalHasher::new(b"TYPE_B")
            .update_u64(1)
            .clone()
            .finalize();
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_schema_affects_hash() {
        let h1 = ConstitutionalHasher::new(b"TEST")
            .update_schema(1, 1)
            .update_u64(42)
            .clone()
            .finalize();
        let mut hasher = ConstitutionalHasher::new(b"TEST");
        hasher.update_schema(1, 2);
        let h2 = hasher.update_u64(42).clone().finalize();
        assert_ne!(h1, h2);
    }
}
