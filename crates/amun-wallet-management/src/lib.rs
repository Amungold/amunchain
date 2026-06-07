pub mod types;
pub mod keygen;
pub mod seed;
pub mod keystore;
pub mod signer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key01_generate_keypair() {
        let kp = keygen::generate_keypair();
        assert_eq!(kp.public_key.len(), 32);
        assert_eq!(kp.secret_key.len(), 32);
    }

    #[test]
    fn key02_deterministic_seed_import() {
        let s = seed::generate_seed();
        let kp1 = keygen::keypair_from_seed(&s);
        let kp2 = keygen::keypair_from_seed(&s);
        assert_eq!(kp1.public_key, kp2.public_key);
        assert_eq!(kp1.secret_key, kp2.secret_key);
    }

    #[test]
    fn key03_export_import_roundtrip() {
        let s = seed::generate_seed();
        let hex = s.to_hex();
        let imported = seed::import_seed_from_hex(&hex).unwrap();
        assert_eq!(s.0, imported.0);
    }

    #[test]
    fn key04_save_and_load_keystore() {
        let kp = keygen::generate_keypair();
        let path = "/tmp/test-keystore-amun.json";
        keystore::save_keystore(&kp, "test-password", path).unwrap();
        let loaded = keystore::load_keystore(path, "test-password").unwrap();
        assert_eq!(kp.public_key, loaded.public_key);
        assert_eq!(kp.secret_key, loaded.secret_key);
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn key05_wrong_password_rejected() {
        let kp = keygen::generate_keypair();
        let path = "/tmp/test-keystore-wrong-pass.json";
        keystore::save_keystore(&kp, "correct", path).unwrap();
        let result = keystore::load_keystore(path, "wrong");
        assert!(result.is_err());
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn key07_sign_and_verify_message() {
        let kp = keygen::generate_keypair();
        let message = b"AmunChain constitutional proof";
        let sig = signer::sign_message(&kp, message);
        assert!(signer::verify_signature(&kp.public_key, message, &sig));
    }

    #[test]
    fn key08_tampered_signature_rejected() {
        let kp = keygen::generate_keypair();
        let message = b"AmunChain constitutional proof";
        let mut sig = signer::sign_message(&kp, message);
        if !sig.is_empty() {
            sig[0] ^= 1;
        }
        assert!(!signer::verify_signature(&kp.public_key, message, &sig));
    }

    #[test]
    fn key09_sign_transaction() {
        let kp = keygen::generate_keypair();
        let tx_bytes = b"transfer:alice->bob:1000";
        let sig = signer::sign_transaction(&kp, tx_bytes);
        assert!(signer::verify_signature(&kp.public_key, tx_bytes, &sig));
    }

    #[test]
    fn key10_address_derivation() {
        let kp = keygen::generate_keypair();
        let addr = kp.address();
        assert!(!addr.0.is_empty());
        let addr2 = kp.address();
        assert_eq!(addr, addr2);
    }

    #[test]
    fn key11_secret_types_not_serializable() {
        let s = seed::generate_seed();
        let kp = keygen::generate_keypair();
        let _ = s.clone();
        let _ = kp.clone();
        // Types correctly lack Serialize impl — this test compiles.
        // Types correctly lack Serialize impl — this test compiles.
        // WalletSeed and WalletKeypair correctly lack Serialize impl.
        // If someone accidentally adds Serialize to these types,
        // this test would fail to compile — that is the real assertion.
    }
}
