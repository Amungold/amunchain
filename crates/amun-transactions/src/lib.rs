use amun_canonical_codec::CanonicalEncode;
use blake3::Hasher;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionPayload {
    Transfer(TransferPayload),
}

impl amun_canonical_codec::CanonicalEncode for TransactionPayload {
    fn canonical_encode(&self) -> Vec<u8> {
        let mut w = amun_canonical_codec::CanonicalWriter::new();
        match self {
            TransactionPayload::Transfer(t) => {
                w.write_u8(0);
                w.write_hash(&t.to);
                w.write_u64(t.amount);
            }
        }
        w.into_bytes()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPayload {
    pub to: [u8; 32],
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u16,
    pub sender: [u8; 32],
    pub nonce: u64,
    pub payload: TransactionPayload,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn tx_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_TX_V1");
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.sender);
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.payload.canonical_encode());
        hasher.finalize().into()
    }

    pub fn signable_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.version.to_le_bytes());
        data.extend_from_slice(&self.nonce.to_le_bytes());
        data.extend_from_slice(&self.payload.canonical_encode());
        data
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        let data = self.signable_data();
        let sig = signing_key.sign(&data);
        self.signature = sig.to_bytes().to_vec();
    }

    pub fn verify(&self) -> bool {
        if self.signature.len() != 64 {
            return false;
        }
        let Ok(verifying_key) = VerifyingKey::from_bytes(&self.sender) else {
            return false;
        };
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(&self.signature);
        let sig = Signature::from_bytes(&sig_bytes);
        verifying_key.verify(&self.signable_data(), &sig).is_ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub tx_hash: [u8; 32],
    pub success: bool,
    pub error_code: Option<u32>,
    pub sender: [u8; 32],
    pub nonce: u64,
    pub gas_used: u64,
}

/// ADR-027: Canonical receipt hash for protocol commitments.
impl TransactionReceipt {
    pub fn receipt_hash(&self) -> [u8; 32] {
        let encoded = self.canonical_encode();
        blake3::hash(&encoded).into()
    }
}

impl amun_canonical_codec::CanonicalEncode for TransactionReceipt {
    fn canonical_encode(&self) -> Vec<u8> {
        let mut w = amun_canonical_codec::CanonicalWriter::new();
        w.write_hash(&self.tx_hash);
        w.write_bool(self.success);
        match self.error_code {
            Some(code) => {
                w.write_bool(true);
                w.write_u32(code);
            }
            None => {
                w.write_bool(false);
            }
        }
        w.write_hash(&self.sender);
        w.write_u64(self.nonce);
        w.write_u64(self.gas_used);
        w.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn create_signed_transfer() -> Transaction {
        let mut seed = [0u8; 32];
        rand::RngCore::fill_bytes(&mut OsRng, &mut seed);
        let signing_key = SigningKey::from_bytes(&seed);
        let sender = signing_key.verifying_key().to_bytes();
        let mut tx = Transaction {
            version: 1,
            sender,
            nonce: 0,
            payload: TransactionPayload::Transfer(TransferPayload {
                to: [2u8; 32],
                amount: 100,
            }),
            signature: vec![],
        };
        tx.sign(&signing_key);
        tx
    }

    #[test]
    fn n23_tx_hash_deterministic() {
        let tx = create_signed_transfer();
        assert_eq!(tx.tx_hash(), tx.tx_hash());
    }
    #[test]
    fn n23_different_nonce_different_hash() {
        let mut seed = [0u8; 32];
        rand::RngCore::fill_bytes(&mut OsRng, &mut seed);
        let sk = SigningKey::from_bytes(&seed);
        let s = sk.verifying_key().to_bytes();
        let mut t1 = Transaction {
            version: 1,
            sender: s,
            nonce: 0,
            payload: TransactionPayload::Transfer(TransferPayload {
                to: [2u8; 32],
                amount: 100,
            }),
            signature: vec![],
        };
        t1.sign(&sk);
        let mut t2 = Transaction {
            version: 1,
            sender: s,
            nonce: 1,
            payload: TransactionPayload::Transfer(TransferPayload {
                to: [2u8; 32],
                amount: 100,
            }),
            signature: vec![],
        };
        t2.sign(&sk);
        assert_ne!(t1.tx_hash(), t2.tx_hash());
    }
    #[test]
    fn n23_sign_and_verify() {
        let tx = create_signed_transfer();
        assert!(tx.verify());
    }
    #[test]
    fn n23_tampered_rejected() {
        let mut tx = create_signed_transfer();
        let TransactionPayload::Transfer(ref mut t) = tx.payload;
        t.amount = 999;
        assert!(!tx.verify());
    }
    #[test]
    fn n23_wrong_signer_rejected() {
        let tx = create_signed_transfer();
        let mut seed = [0u8; 32];
        rand::RngCore::fill_bytes(&mut OsRng, &mut seed);
        let impostor = SigningKey::from_bytes(&seed);
        let mut fake = tx.clone();
        fake.sender = impostor.verifying_key().to_bytes();
        assert!(!fake.verify());
    }
    #[test]
    fn n23_transfer_roundtrip() {
        let t = TransferPayload {
            to: [2u8; 32],
            amount: 100,
        };
        let j = serde_json::to_string(&t).unwrap();
        let d: TransferPayload = serde_json::from_str(&j).unwrap();
        assert_eq!(d.amount, 100);
    }
    #[test]
    fn n23_receipt_error_code() {
        let tx = create_signed_transfer();
        let r = TransactionReceipt {
            tx_hash: tx.tx_hash(),
            success: false,
            error_code: Some(1),
            sender: tx.sender,
            nonce: tx.nonce,
            gas_used: 0,
        };
        assert!(!r.success);
        assert_eq!(r.error_code, Some(1));
    }
}

// ADR-027: receipt_hash protocol tests
#[cfg(test)]
#[test]
fn test_receipt_hash_deterministic() {
    let r = crate::TransactionReceipt {
        tx_hash: [1u8; 32],
        success: true,
        error_code: None,
        sender: [2u8; 32],
        nonce: 42,
        gas_used: 1000,
    };
    assert_eq!(r.receipt_hash(), r.receipt_hash());
}

#[cfg(test)]
#[test]
fn test_receipt_hash_changes_with_field() {
    let r1 = crate::TransactionReceipt {
        tx_hash: [1u8; 32],
        success: true,
        error_code: None,
        sender: [2u8; 32],
        nonce: 42,
        gas_used: 1000,
    };
    let r2 = crate::TransactionReceipt {
        tx_hash: [1u8; 32],
        success: true,
        error_code: None,
        sender: [2u8; 32],
        nonce: 42,
        gas_used: 2000,
    };
    assert_ne!(r1.receipt_hash(), r2.receipt_hash());
}
