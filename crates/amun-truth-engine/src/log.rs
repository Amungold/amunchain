use amun_chain_position::ChainPosition;
use amun_deterministic_allocator::DeterministicArena;
use blake3::Hasher;

const MAX_ENTRIES: usize = 100_000;
const MAX_PAYLOAD: usize = 65535;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptEntry {
    Transaction {
        position: ChainPosition,
        payload_hash: [u8; 32],
    },
    EpochSeal {
        epoch: u64,
        seal_hash: [u8; 32],
    },
}

#[derive(Debug, Clone)]
pub struct MessageEntry {
    pub position: ChainPosition,
    pub transcript: TranscriptEntry,
    pub arena_offset: usize,
    pub payload_len: u32,
    pub hash: [u8; 32],
    pub chain_hash: [u8; 32],
}
impl MessageEntry {
    pub fn position(&self) -> ChainPosition {
        self.position
    }
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

pub struct MessageLog {
    entries: Vec<MessageEntry>,
    arena: DeterministicArena,
    max_entries: usize,
    last_position: Option<ChainPosition>,
}

impl MessageLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_ENTRIES),
            arena: DeterministicArena::new(),
            max_entries: MAX_ENTRIES,
            last_position: None,
        }
    }

    pub fn append(
        &mut self,
        position: ChainPosition,
        payload: &[u8],
    ) -> Result<MessageEntry, &'static str> {
        if let Some(lp) = self.last_position {
            if position <= lp {
                return Err("non-monotonic");
            }
        }
        if self.entries.len() >= self.max_entries {
            return Err("log full");
        }
        let (off, plen) = if payload.is_empty() {
            (0, 0u32)
        } else {
            if payload.len() > MAX_PAYLOAD {
                return Err("payload too large");
            }
            let (s, o) = self
                .arena
                .allocate(payload.len())
                .ok_or("arena exhausted")?;
            s.copy_from_slice(payload);
            (o, payload.len() as u32)
        };
        let mut h = Hasher::new();
        h.update(&position.hash());
        h.update(payload);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        let prev = self
            .entries
            .last()
            .map(|e| e.chain_hash)
            .unwrap_or([0u8; 32]);
        let mut ch = Hasher::new();
        ch.update(&prev);
        ch.update(&hash);
        let mut chain = [0u8; 32];
        chain.copy_from_slice(&ch.finalize().as_bytes()[..32]);
        let entry = MessageEntry {
            position,
            transcript: TranscriptEntry::Transaction {
                position,
                payload_hash: hash,
            },
            arena_offset: off,
            payload_len: plen,
            hash,
            chain_hash: chain,
        };
        self.entries.push(entry.clone());
        self.last_position = Some(position);
        Ok(entry)
    }

    pub fn append_seal(
        &mut self,
        position: ChainPosition,
        seal_hash: [u8; 32],
    ) -> Result<MessageEntry, &'static str> {
        if let Some(lp) = self.last_position {
            if position <= lp {
                return Err("non-monotonic");
            }
        }
        if self.entries.len() >= self.max_entries {
            return Err("log full");
        }
        let mut h = Hasher::new();
        h.update(&position.hash());
        h.update(&seal_hash);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        let prev = self
            .entries
            .last()
            .map(|e| e.chain_hash)
            .unwrap_or([0u8; 32]);
        let mut ch = Hasher::new();
        ch.update(&prev);
        ch.update(&hash);
        let mut chain = [0u8; 32];
        chain.copy_from_slice(&ch.finalize().as_bytes()[..32]);
        let entry = MessageEntry {
            position,
            transcript: TranscriptEntry::EpochSeal {
                epoch: position.epoch,
                seal_hash,
            },
            arena_offset: 0,
            payload_len: 0,
            hash,
            chain_hash: chain,
        };
        self.entries.push(entry.clone());
        self.last_position = Some(position);
        Ok(entry)
    }

    pub fn payload<'a>(&'a self, entry: &'a MessageEntry) -> Option<&'a [u8]> {
        if entry.payload_len == 0 {
            Some(&[])
        } else {
            self.arena
                .get_slice(entry.arena_offset, entry.payload_len as usize)
        }
    }
    pub fn entries(&self) -> &[MessageEntry] {
        &self.entries
    }
    pub fn chain_hash(&self) -> [u8; 32] {
        self.entries
            .last()
            .map(|e| e.chain_hash)
            .unwrap_or([0u8; 32])
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for MessageLog {
    fn default() -> Self {
        Self::new()
    }
}
