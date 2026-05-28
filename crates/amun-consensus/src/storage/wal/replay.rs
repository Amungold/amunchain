use std::fs::File;
use std::io::Read;
use crate::storage::wal::codec::{WALFrame, WALOp};

pub struct WALReplayIterator {
    buffer: Vec<u8>,
    pos: usize,
    expected_seq: u64,
    prev_hash: [u8; 32],
    pub corruption_detected: bool,
}

impl WALReplayIterator {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(Self { buffer, pos: 0, expected_seq: 0, prev_hash: [0u8; 32], corruption_detected: false })
    }
    
    pub fn next(&mut self) -> Option<WALFrame> {
        if self.corruption_detected { return None; }
        if self.pos + 4 > self.buffer.len() { return None; }
        let len = u32::from_le_bytes([self.buffer[self.pos], self.buffer[self.pos+1], self.buffer[self.pos+2], self.buffer[self.pos+3]]) as usize;
        self.pos += 4;
        if self.pos + len > self.buffer.len() {
            self.corruption_detected = true;
            return None;
        }
        let frame = WALFrame::from_bytes(&self.buffer[self.pos..self.pos+len])?;
        self.pos += len;
        if frame.sequence != self.expected_seq || frame.prev_hash != self.prev_hash {
            self.corruption_detected = true;
            return None;
        }
        self.expected_seq += 1;
        self.prev_hash = frame.entry_hash;
        Some(frame)
    }
}
