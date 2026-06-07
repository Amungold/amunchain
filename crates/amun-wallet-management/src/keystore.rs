use std::fs;

use argon2::Argon2;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::types::{Keystore, WalletKeypair};

/// Encrypt a keypair with a password and save to a keystore file.
pub fn save_keystore(
    keypair: &WalletKeypair,
    password: &str,
    path: &str,
) -> Result<(), String> {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| format!("argon2 error: {}", e))?;

    // XOR encrypt the private key
    let mut encrypted = [0u8; 32];
    for i in 0..32 {
        encrypted[i] = keypair.secret_key[i] ^ key[i];
    }

    let keystore = Keystore {
        version: 1,
        public_key: keypair.public_key_hex(),
        encrypted_private_key: hex::encode(encrypted),
        salt: hex::encode(salt),
        verification_hash: keypair.verification_hash(),
    };

    let json = serde_json::to_string_pretty(&keystore)
        .map_err(|e| format!("serialization error: {}", e))?;
    fs::write(path, json)
        .map_err(|e| format!("write error: {}", e))?;

    Ok(())
}

/// Load a keystore file, decrypt with password, and return the keypair.
/// Returns an error if the password is incorrect (detected via verification hash).
pub fn load_keystore(path: &str, password: &str) -> Result<WalletKeypair, String> {
    let json = fs::read_to_string(path)
        .map_err(|e| format!("read error: {}", e))?;
    let keystore: Keystore = serde_json::from_str(&json)
        .map_err(|e| format!("invalid keystore format: {}", e))?;

    if keystore.version != 1 {
        return Err(format!("unsupported keystore version: {}", keystore.version));
    }

    let salt = hex::decode(&keystore.salt)
        .map_err(|e| format!("invalid salt: {}", e))?;
    let encrypted = hex::decode(&keystore.encrypted_private_key)
        .map_err(|e| format!("invalid encrypted key: {}", e))?;

    if encrypted.len() != 32 {
        return Err("encrypted key must be 32 bytes".into());
    }

    // Derive key from password
    let mut derived_key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut derived_key)
        .map_err(|_| "incorrect password".to_string())?;

    // XOR decrypt
    let mut secret_key = [0u8; 32];
    for i in 0..32 {
        secret_key[i] = encrypted[i] ^ derived_key[i];
    }

    let public_key = hex::decode(&keystore.public_key)
        .map_err(|e| format!("invalid public key: {}", e))?;

    let mut pub_bytes = [0u8; 32];
    pub_bytes.copy_from_slice(&public_key);

    let keypair = WalletKeypair {
        public_key: pub_bytes,
        secret_key,
    };

    // Verify the checksum — if password was wrong, this won't match
    if keypair.verification_hash() != keystore.verification_hash {
        return Err("incorrect password".into());
    }

    Ok(keypair)
}
