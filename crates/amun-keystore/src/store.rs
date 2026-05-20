use crate::cipher;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize)]
pub struct KeyStore {
    pub version: String,
    pub public_key: String,
    pub ciphertext: String,
    pub nonce: String,
    pub salt: String,
    pub address: String,
    pub chain_id: u64,
}

impl KeyStore {
    pub fn create(
        secret_key: &[u8],
        public_key: &[u8],
        address: &str,
        password: &str,
        chain_id: u64,
    ) -> Result<Self, &'static str> {
        let mut aad = Vec::new();
        aad.extend_from_slice(&chain_id.to_le_bytes());
        aad.extend_from_slice(b"AMUN_KEYSTORE_V4");

        let (ct, nonce, salt) = cipher::encrypt_secret(secret_key, password, &aad)?;

        Ok(Self {
            version: "4".to_string(),
            public_key: hex::encode(public_key),
            ciphertext: hex::encode(&ct),
            nonce: hex::encode(&nonce),
            salt: hex::encode(&salt),
            address: address.to_string(),
            chain_id,
        })
    }

    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>, &'static str> {
        let ct = hex::decode(&self.ciphertext).map_err(|_| "invalid ciphertext")?;
        let nonce = hex::decode(&self.nonce).map_err(|_| "invalid nonce")?;
        let salt = hex::decode(&self.salt).map_err(|_| "invalid salt")?;

        let mut aad = Vec::new();
        aad.extend_from_slice(&self.chain_id.to_le_bytes());
        aad.extend_from_slice(b"AMUN_KEYSTORE_V4");

        cipher::decrypt_secret(&ct, &nonce, &salt, password, &aad)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

impl Drop for KeyStore {
    fn drop(&mut self) {
        self.ciphertext.zeroize();
        self.nonce.zeroize();
        self.salt.zeroize();
    }
}
