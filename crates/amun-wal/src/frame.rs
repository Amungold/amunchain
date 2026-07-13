use amun_protocol_event::ProtocolEvent;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub enum WalEntry {
    Event(ProtocolEvent),
}

#[derive(Debug, Clone)]
pub struct WalFrame {
    pub sequence: u64,
    pub entry: WalEntry,
    pub checksum: [u8; 32],
    pub previous_frame_hash: [u8; 32],
}

impl WalFrame {
    pub fn new(sequence: u64, entry: WalEntry, prev_hash: [u8; 32]) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_WAL_FRAME_V5");
        h.update(b"SEQ"); h.update(&sequence.to_le_bytes());
        h.update(b"PREV"); h.update(&prev_hash);
        match &entry {
            WalEntry::Event(event) => {
                let encoded = event.encode();
                h.update(b"EVENT"); h.update(&encoded);
            }
        }
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&h.finalize().as_bytes()[..32]);
        Self { sequence, entry, checksum, previous_frame_hash: prev_hash }
    }

    pub fn verify(&self, prev_hash: [u8; 32]) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_WAL_FRAME_V5");
        h.update(b"SEQ"); h.update(&self.sequence.to_le_bytes());
        h.update(b"PREV"); h.update(&prev_hash);
        match &self.entry {
            WalEntry::Event(event) => {
                let encoded = event.encode();
                h.update(b"EVENT"); h.update(&encoded);
            }
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.checksum && self.previous_frame_hash == prev_hash
    }
}
