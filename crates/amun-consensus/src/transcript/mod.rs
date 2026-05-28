//! Canonical Transcript Module - LENGTH-PREFIXED
//! For deterministic replay certification

use blake3;

#[derive(Debug, Clone)]
pub struct TranscriptEntry {
    pub round: u64,
    pub event_type: TranscriptEvent,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub enum TranscriptEvent {
    Vote { validator_id: u64, vote_hash: [u8; 32] },
    QC { qc_hash: [u8; 32], height: u64 },
    Timeout { round: u64 },
    Commit { block_hash: [u8; 32], height: u64 },
}

#[derive(Debug, Clone)]
pub struct Transcript {
    entries: Vec<TranscriptEntry>,
    current_hash: [u8; 32],
}

impl Transcript {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_hash: [0u8; 32],
        }
    }
    
    fn encode_event(event_type: &TranscriptEvent) -> Vec<u8> {
        let mut bytes = Vec::new();
        match event_type {
            TranscriptEvent::Vote { validator_id, vote_hash } => {
                bytes.push(0x01);
                bytes.extend_from_slice(&validator_id.to_le_bytes());
                bytes.extend_from_slice(vote_hash);
            }
            TranscriptEvent::QC { qc_hash, height } => {
                bytes.push(0x02);
                bytes.extend_from_slice(qc_hash);
                bytes.extend_from_slice(&height.to_le_bytes());
            }
            TranscriptEvent::Timeout { round: r } => {
                bytes.push(0x03);
                bytes.extend_from_slice(&r.to_le_bytes());
            }
            TranscriptEvent::Commit { block_hash, height } => {
                bytes.push(0x04);
                bytes.extend_from_slice(block_hash);
                bytes.extend_from_slice(&height.to_le_bytes());
            }
        }
        bytes
    }
    
    pub fn push(&mut self, round: u64, event_type: TranscriptEvent) -> [u8; 32] {
        let mut bytes = Vec::new();
        // Domain separation
        bytes.extend_from_slice(b"AMUN_TRANSCRIPT_V1");
        bytes.extend_from_slice(&self.current_hash);
        bytes.extend_from_slice(&round.to_le_bytes());
        
        // Length-prefixed encoding to prevent ambiguity
        let encoded = Self::encode_event(&event_type);
        bytes.extend_from_slice(&(encoded.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&encoded);
        
        let hash = blake3::hash(&bytes).into();
        
        self.entries.push(TranscriptEntry { round, event_type, hash });
        self.current_hash = hash;
        hash
    }
    
    pub fn final_hash(&self) -> [u8; 32] {
        self.current_hash
    }
    
    pub fn entries(&self) -> &[TranscriptEntry] {
        &self.entries
    }
}

impl Default for Transcript {
    fn default() -> Self {
        Self::new()
    }
}
