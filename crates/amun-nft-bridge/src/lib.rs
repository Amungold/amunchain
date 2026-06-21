use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Lock event: NFT is locked in source chain for bridging
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeLock {
    pub source_chain: u32,
    pub token_id: [u8; 32],
    pub owner: [u8; 32],
    pub destination_chain: u32,
    pub destination_owner: [u8; 32],
    pub lock_height: u64,
}

/// Unlock event: NFT is unlocked on destination chain (or released back)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeUnlock {
    pub lock_id: [u8; 32],
    pub destination_chain: u32,
    pub new_owner: [u8; 32],
    pub unlock_height: u64,
}

/// Complete bridge record for evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeRecord {
    pub lock: BridgeLock,
    pub unlock: Option<BridgeUnlock>,
}

/// Bridge ledger managing locks and proofs
#[derive(Debug, Clone, Default)]
pub struct BridgeLedger {
    pub locks: BTreeMap<[u8; 32], BridgeLock>,
    pub records: Vec<BridgeRecord>,
}

impl BridgeLedger {
    pub fn new() -> Self {
        Self { locks: BTreeMap::new(), records: Vec::new() }
    }

    /// Lock an NFT for cross-chain transfer
    pub fn lock(&mut self, lock: BridgeLock) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_BRIDGE_LOCK_V1");
        hasher.update(lock.source_chain.to_le_bytes());
        hasher.update(lock.token_id);
        hasher.update(lock.owner);
        hasher.update(lock.destination_chain.to_le_bytes());
        hasher.update(lock.destination_owner);
        hasher.update(lock.lock_height.to_le_bytes());
        let lock_id: [u8; 32] = hasher.finalize().into();
        self.locks.insert(lock_id, lock.clone());
        self.records.push(BridgeRecord { lock, unlock: None });
        lock_id
    }

    /// Complete a lock by recording the unlock
    pub fn unlock(&mut self, unlock: BridgeUnlock) -> Option<BridgeRecord> {
        if !self.locks.contains_key(&unlock.lock_id) {
            return None;
        }
        self.locks.remove(&unlock.lock_id);
        // Find the record with matching lock_id (last inserted with no unlock)
        for record in self.records.iter_mut() {
            if record.unlock.is_none() && compute_lock_id(&record.lock) == unlock.lock_id {
                record.unlock = Some(unlock.clone());
                return Some(record.clone());
            }
        }
        None
    }

    /// Check if a lock is pending
    pub fn is_locked(&self, lock_id: &[u8; 32]) -> bool {
        self.locks.contains_key(lock_id)
    }

    /// Compute bridge evidence root
    pub fn compute_bridge_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_BRIDGE_EVIDENCE_V1");
        for record in &self.records {
            let bytes = serde_json::to_vec(record).unwrap();
            hasher.update(&bytes);
        }
        hasher.finalize().into()
    }
}

fn compute_lock_id(lock: &BridgeLock) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"AMUN_BRIDGE_LOCK_V1");
    hasher.update(lock.source_chain.to_le_bytes());
    hasher.update(lock.token_id);
    hasher.update(lock.owner);
    hasher.update(lock.destination_chain.to_le_bytes());
    hasher.update(lock.destination_owner);
    hasher.update(lock.lock_height.to_le_bytes());
    hasher.finalize().into()
}
