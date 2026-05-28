use crate::canonical::{Decoder, Encoder};

pub const WAL_DOMAIN_SEPARATOR: &[u8] = b"AMUN_WAL_FRAME_V1";
pub const CHAIN_DOMAIN_ID: &[u8] = b"AMUNCHAIN_MAINNET_V1";

#[derive(Debug, Clone)]
pub struct WalEntry {
    pub sequence: u64,
    pub op_type: u8,
    pub tx_id: u64,
    pub epoch: u64,
    pub generation: u64,
    pub key_hash: Vec<u8>,
    pub value_hash: Vec<u8>,
    pub version: u64,
    pub state_root: [u8; 32],
    pub prev_entry_hash: [u8; 32],
    pub entry_hash: [u8; 32],
}

impl WalEntry {
    pub fn new(
        sequence: u64,
        op_type: u8,
        tx_id: u64,
        epoch: u64,
        generation: u64,
        key_hash: Vec<u8>,
        value_hash: Vec<u8>,
        version: u64,
        state_root: [u8; 32],
        prev_entry_hash: [u8; 32],
    ) -> Self {
        let entry_hash = Self::compute_hash(
            sequence,
            op_type,
            tx_id,
            epoch,
            generation,
            &key_hash,
            &value_hash,
            version,
            &state_root,
            &prev_entry_hash,
        );
        Self {
            sequence,
            op_type,
            tx_id,
            epoch,
            generation,
            key_hash,
            value_hash,
            version,
            state_root,
            prev_entry_hash,
            entry_hash,
        }
    }

    pub fn verify_chain(&self, expected_prev: &[u8; 32]) -> bool {
        if &self.prev_entry_hash != expected_prev {
            return false;
        }
        let computed = Self::compute_hash(
            self.sequence,
            self.op_type,
            self.tx_id,
            self.epoch,
            self.generation,
            &self.key_hash,
            &self.value_hash,
            self.version,
            &self.state_root,
            &self.prev_entry_hash,
        );
        computed == self.entry_hash
    }

    fn compute_hash(
        sequence: u64,
        op_type: u8,
        tx_id: u64,
        epoch: u64,
        generation: u64,
        key_hash: &[u8],
        value_hash: &[u8],
        version: u64,
        state_root: &[u8; 32],
        prev_entry_hash: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(WAL_DOMAIN_SEPARATOR);
        hasher.update(CHAIN_DOMAIN_ID);
        hasher.update(&epoch.to_le_bytes());
        hasher.update(&generation.to_le_bytes());
        hasher.update(&sequence.to_le_bytes());
        hasher.update(&[op_type]);
        hasher.update(&tx_id.to_le_bytes());
        hasher.update(&(key_hash.len() as u64).to_le_bytes());
        hasher.update(key_hash);
        hasher.update(&(value_hash.len() as u64).to_le_bytes());
        hasher.update(value_hash);
        hasher.update(&version.to_le_bytes());
        hasher.update(state_root);
        hasher.update(prev_entry_hash);
        hasher.finalize().into()
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut enc = Encoder::new();
        enc.write_u64(self.sequence);
        enc.write_u8(self.op_type);
        enc.write_u64(self.tx_id);
        enc.write_u64(self.epoch);
        enc.write_u64(self.generation);
        enc.write_bytes(&self.key_hash);
        enc.write_bytes(&self.value_hash);
        enc.write_u64(self.version);
        enc.write_bytes(&self.state_root);
        enc.write_bytes(&self.prev_entry_hash);
        enc.write_bytes(&self.entry_hash);
        enc.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut dec = Decoder::new(data);
        let sequence = dec.read_u64()?;
        let op_type = dec.read_u8()?;
        let tx_id = dec.read_u64()?;
        let epoch = dec.read_u64()?;
        let generation = dec.read_u64()?;
        let key_hash = dec.read_bytes()?;
        let value_hash = dec.read_bytes()?;
        let version = dec.read_u64()?;
        let state_root: [u8; 32] = dec.read_bytes()?.try_into().ok()?;
        let prev_entry_hash: [u8; 32] = dec.read_bytes()?.try_into().ok()?;
        let entry_hash: [u8; 32] = dec.read_bytes()?.try_into().ok()?;
        if !dec.is_finished() {
            return None;
        }
        Some(Self {
            sequence,
            op_type,
            tx_id,
            epoch,
            generation,
            key_hash,
            value_hash,
            version,
            state_root,
            prev_entry_hash,
            entry_hash,
        })
    }
}
