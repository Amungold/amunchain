use amun_crypto::Ed25519Signer;
use blake3::Hasher;

pub struct AuditJournal {
    pub entries: Vec<AuditEntry>,
}

pub struct AuditEntry {
    pub epoch: u64,
    pub event_type: String,
    pub payload_hash: [u8; 32],
    pub previous_entry_hash: [u8; 32],
    pub signature: [u8; 64],
    pub entry_hash: [u8; 32],
}

impl AuditJournal {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn record(
        &mut self,
        epoch: u64,
        event_type: &str,
        payload: &[u8],
        signer: &Ed25519Signer,
        chain_id: u64,
    ) -> Result<(), &'static str> {
        let mut hasher = Hasher::new();
        hasher.update(payload);
        let mut payload_hash = [0u8; 32];
        payload_hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

        let prev_hash = self
            .entries
            .last()
            .map(|e| e.entry_hash)
            .unwrap_or([0u8; 32]);

        let mut entry_hasher = Hasher::new();
        entry_hasher.update(b"AMUN_AUDIT_ENTRY_V4");
        entry_hasher.update(&epoch.to_le_bytes());
        entry_hasher.update(event_type.as_bytes());
        entry_hasher.update(&payload_hash);
        entry_hasher.update(&prev_hash);
        let mut entry_hash = [0u8; 32];
        entry_hash.copy_from_slice(&entry_hasher.finalize().as_bytes()[..32]);

        let signature = signer
            .sign(&entry_hash, b"AMUN_AUDIT_V4", chain_id)
            .map_err(|_| "signing failed")?;

        self.entries.push(AuditEntry {
            epoch,
            event_type: event_type.to_string(),
            payload_hash,
            previous_entry_hash: prev_hash,
            signature,
            entry_hash,
        });

        Ok(())
    }

    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = [0u8; 32];
        for entry in &self.entries {
            if entry.previous_entry_hash != prev {
                return false;
            }
            prev = entry.entry_hash;
        }
        true
    }
}

impl Default for AuditJournal {
    fn default() -> Self {
        Self::new()
    }
}
