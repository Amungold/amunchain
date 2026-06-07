use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};

use crate::types::WalletKeypair;

/// Sign a message using the wallet keypair.
pub fn sign_message(keypair: &WalletKeypair, message: &[u8]) -> Vec<u8> {
    let signing_key = SigningKey::from_bytes(&keypair.secret_key);
    let signature: Signature = signing_key.sign(message);
    signature.to_bytes().to_vec()
}

/// Verify a signature against a public key and message.
pub fn verify_signature(public_key: &[u8; 32], message: &[u8], signature: &[u8]) -> bool {
    let verifying_key = match VerifyingKey::from_bytes(public_key) {
        Ok(vk) => vk,
        Err(_) => return false,
    };
    let sig = match Signature::from_slice(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };
    verifying_key.verify(message, &sig).is_ok()
}

/// Sign a transaction (represented as bytes) with the wallet keypair.
pub fn sign_transaction(keypair: &WalletKeypair, transaction_bytes: &[u8]) -> Vec<u8> {
    sign_message(keypair, transaction_bytes)
}
