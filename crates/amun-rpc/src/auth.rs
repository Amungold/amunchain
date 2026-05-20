use blake3::Hasher;
use subtle::ConstantTimeEq;

pub struct AuthValidator {
    token_hash: Option<[u8; 32]>,
}

impl AuthValidator {
    pub fn new(token: Option<&str>) -> Self {
        let token_hash = token.map(|t| {
            let mut hasher = Hasher::new();
            hasher.update(t.as_bytes());
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
            hash
        });
        Self { token_hash }
    }

    pub fn requires_auth(&self) -> bool {
        self.token_hash.is_some()
    }

    pub fn validate(&self, token: &str) -> bool {
        if let Some(expected) = self.token_hash {
            let mut hasher = Hasher::new();
            hasher.update(token.as_bytes());
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

            // Constant-time comparison
            hash.ct_eq(&expected).into()
        } else {
            true
        }
    }
}
