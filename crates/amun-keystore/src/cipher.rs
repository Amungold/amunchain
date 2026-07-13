#![allow(clippy::type_complexity)]
use argon2::Argon2;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
    Key, XChaCha20Poly1305, XNonce,
};
use rand_core::RngCore;
use zeroize::Zeroize;

const SALT_SIZE: usize = 32;

pub fn encrypt_secret(
    secret: &[u8],
    password: &str,
    aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), &'static str> {
    let mut salt = vec![0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);

    let mut key_material = vec![0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key_material)
        .map_err(|_| "argon2 failed")?;

    let key = *Key::from_slice(&key_material);
    key_material.zeroize();

    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let payload = Payload { msg: secret, aad };
    let ciphertext = cipher
        .encrypt(&nonce, payload)
        .map_err(|_| "encryption failed")?;

    Ok((ciphertext, nonce.to_vec(), salt))
}

pub fn decrypt_secret(
    ciphertext: &[u8],
    nonce_bytes: &[u8],
    salt: &[u8],
    password: &str,
    aad: &[u8],
) -> Result<Vec<u8>, &'static str> {
    let mut key_material = vec![0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key_material)
        .map_err(|_| "argon2 failed")?;

    let key = *Key::from_slice(&key_material);
    key_material.zeroize();

    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XNonce::from_slice(nonce_bytes);
    let payload = Payload {
        msg: ciphertext,
        aad,
    };
    cipher
        .decrypt(nonce, payload)
        .map_err(|_| "decryption failed")
}
