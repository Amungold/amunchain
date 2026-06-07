use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    #[serde(with = "serde_bytes")]
    pub tx_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub sender: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub recipient: [u8; 32],
    pub amount: u64,
    pub nonce: u64,
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    TransactionAnnounce {
        #[serde(with = "serde_bytes")]
        tx_hash: [u8; 32],
        #[serde(with = "serde_bytes")]
        sender_id: [u8; 32],
    },
    TransactionRequest {
        #[serde(with = "serde_bytes")]
        tx_hash: [u8; 32],
        #[serde(with = "serde_bytes")]
        requester_id: [u8; 32],
    },
    TransactionResponse {
        transaction: Transaction,
    },
}

impl Transaction {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_TX_V1");
        hasher.update(&self.sender);
        hasher.update(&self.recipient);
        hasher.update(&self.amount.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify_hash(&self) -> bool {
        self.tx_hash == self.compute_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tx(sender: u8, nonce: u64) -> Transaction {
        let mut tx = Transaction {
            tx_hash: [0u8; 32],
            sender: [sender; 32],
            recipient: [sender + 1; 32],
            amount: 100,
            nonce,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        tx.tx_hash = tx.compute_hash();
        tx
    }

    #[test]
    fn n74_tx_hash_deterministic() {
        let tx1 = make_tx(1, 0);
        let tx2 = make_tx(1, 0);
        assert_eq!(tx1.tx_hash, tx2.tx_hash);
    }

    #[test]
    fn n74_tx_hash_changes_with_nonce() {
        let tx1 = make_tx(1, 0);
        let tx2 = make_tx(1, 1);
        assert_ne!(tx1.tx_hash, tx2.tx_hash);
    }

    #[test]
    fn n74_tx_hash_verification() {
        let tx = make_tx(1, 0);
        assert!(tx.verify_hash());
    }

    #[test]
    fn n74_tx_tampered_rejected() {
        let mut tx = make_tx(1, 0);
        tx.amount = 999;
        assert!(!tx.verify_hash());
    }

    #[test]
    fn n74_gossip_message_serialization() {
        let msg = GossipMessage::TransactionAnnounce {
            tx_hash: [0xAA; 32],
            sender_id: [1u8; 32],
        };
        let encoded = postcard::to_stdvec(&msg).unwrap();
        let decoded: GossipMessage = postcard::from_bytes(&encoded).unwrap();
        match decoded {
            GossipMessage::TransactionAnnounce { tx_hash, sender_id } => {
                assert_eq!(tx_hash, [0xAA; 32]);
                assert_eq!(sender_id, [1u8; 32]);
            }
            _ => panic!("Wrong variant"),
        }
    }
}
