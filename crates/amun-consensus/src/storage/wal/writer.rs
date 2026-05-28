use std::fs::{File, OpenOptions};
use std::io::{Write, Read, Seek, SeekFrom};
use std::sync::Mutex;
use crate::storage::constants::MAX_WAL_ENTRY_SIZE;
use crate::storage::wal::codec::{WALFrame, WALOp};

struct WALState {
    file: File,
    next_seq: u64,
    last_hash: [u8; 32],
}

impl WALState {
    fn new(mut file: File) -> std::io::Result<Self> {
        let (next_seq, last_hash) = Self::recover(&mut file)?;
        Ok(Self { file, next_seq, last_hash })
    }
    
    fn recover(file: &mut File) -> std::io::Result<(u64, [u8; 32])> {
        file.seek(SeekFrom::Start(0))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut pos = 0;
        let mut seq = 0;
        let mut last_hash = [0u8; 32];
        while pos + 4 <= buffer.len() {
            let len = u32::from_le_bytes([buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]]) as usize;
            pos += 4;
            if pos + len > buffer.len() { break; }
            if let Some(frame) = WALFrame::from_bytes(&buffer[pos..pos+len]) {
                seq = frame.sequence + 1;
                last_hash = frame.entry_hash;
            }
            pos += len;
        }
        Ok((seq, last_hash))
    }
}

pub struct WALWriter {
    state: Mutex<WALState>,
}

impl WALWriter {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        let state = WALState::new(file)?;
        Ok(Self { state: Mutex::new(state) })
    }
    
    pub fn append(&self, op: WALOp) -> std::io::Result<[u8; 32]> {
        let mut state = self.state.lock().unwrap();
        let frame = WALFrame::new(state.next_seq, state.last_hash, op);
        let bytes = frame.to_bytes();
        if bytes.len() > MAX_WAL_ENTRY_SIZE {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Entry too large"));
        }
        let len_bytes = (bytes.len() as u32).to_le_bytes();
        state.file.write_all(&len_bytes)?;
        state.file.write_all(&bytes)?;
        state.file.sync_all()?;
        state.next_seq += 1;
        state.last_hash = frame.entry_hash;
        Ok(frame.entry_hash)
    }
}
