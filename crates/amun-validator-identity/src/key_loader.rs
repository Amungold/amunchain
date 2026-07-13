use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use ed25519_dalek::SigningKey;
use rand_core::OsRng;

use crate::derive_validator_id;

pub fn load_or_create_signing_key<P: AsRef<Path>>(path: P) -> Result<SigningKey, Error> {
    let path = path.as_ref();

    if path.exists() {
        let bytes = fs::read(path)?;
        let seed: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                "validator key must be exactly 32 bytes",
            )
        })?;

        return Ok(SigningKey::from_bytes(&seed));
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let sk = SigningKey::generate(&mut OsRng);

    fs::write(path, sk.to_bytes())?;

    Ok(sk)
}

pub fn load_signing_key<P: AsRef<Path>>(path: P) -> Result<SigningKey, Error> {
    load_or_create_signing_key(path)
}

pub fn load_public_key<P: AsRef<Path>>(path: P) -> Result<[u8; 32], Error> {
    Ok(load_signing_key(path)?.verifying_key().to_bytes())
}

pub fn derive_validator_id_from_key(key: &SigningKey) -> [u8; 32] {
    derive_validator_id(&key.verifying_key().to_bytes())
}
