use crate::frame::{WalFrame, WalEntry};
use amun_protocol_event::ProtocolEvent;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::PathBuf;

const WAL_MAGIC: &[u8; 8] = b"AMUNWAL3";
const FRAME_HEADER_SIZE: usize = 80;
const MAX_EVENT_SIZE: usize = 16 * 1024 * 1024;

pub struct WriteAheadLog {
    file: Option<File>,
    last_frame_hash: [u8; 32],
    sequence: u64,
    frame_count: u64,
}

impl WriteAheadLog {
    pub fn create(path: PathBuf) -> std::io::Result<Self> {
        let mut file = OpenOptions::new()
            .create(true).read(true).write(true).truncate(true)
            .open(&path)?;
        file.write_all(WAL_MAGIC)?;
        file.sync_all()?;
        Ok(Self { file: Some(file), last_frame_hash: [0u8; 32], sequence: 0, frame_count: 0 })
    }

    pub fn open(path: PathBuf) -> std::io::Result<Self> {
        let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)?;
        if &magic != WAL_MAGIC {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid WAL magic"));
        }
        let mut wal = Self { file: Some(file), last_frame_hash: [0u8; 32], sequence: 0, frame_count: 0 };
        wal.replay_frames()?;
        Ok(wal)
    }

    fn validate_header(header: &[u8; FRAME_HEADER_SIZE]) -> bool {
        header[77] == 0 && header[78] == 0 && header[79] == 0
    }

    fn replay_frames(&mut self) -> std::io::Result<()> {
        let file = self.file.as_mut().unwrap();
        let file_len = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(WAL_MAGIC.len() as u64))?;
        let mut offset = WAL_MAGIC.len() as u64;
        let mut prev_hash = [0u8; 32];

        while offset + FRAME_HEADER_SIZE as u64 <= file_len {
            file.seek(SeekFrom::Start(offset))?;
            let mut header = [0u8; FRAME_HEADER_SIZE];
            if file.read_exact(&mut header).is_err() { break; }
            if !Self::validate_header(&header) { break; }
            let seq = u64::from_le_bytes(header[0..8].try_into().unwrap());
            let checksum: [u8; 32] = header[8..40].try_into().unwrap();
            let frame_prev_hash: [u8; 32] = header[40..72].try_into().unwrap();
            let event_len = u32::from_le_bytes(header[72..76].try_into().unwrap()) as usize;
            if seq != self.sequence + 1 { break; }
            if event_len > MAX_EVENT_SIZE { break; }
            let frame_size = FRAME_HEADER_SIZE + event_len;
            if offset + frame_size as u64 > file_len { break; }
            let mut event_bytes = vec![0u8; event_len];
            file.read_exact(&mut event_bytes)?;
            let event = match ProtocolEvent::decode(&event_bytes) {
                Some(e) => e,
                None => break,
            };
            let entry = WalEntry::Event(event);
            let frame = WalFrame { sequence: seq, entry, checksum, previous_frame_hash: frame_prev_hash };
            if !frame.verify(prev_hash) { break; }
            self.last_frame_hash = checksum;
            self.sequence = seq;
            self.frame_count += 1;
            prev_hash = checksum;
            offset += frame_size as u64;
        }
        Ok(())
    }

    pub fn append_event(&mut self, event: &ProtocolEvent) -> std::io::Result<()> {
        let file = self.file.as_mut().unwrap();
        let entry = WalEntry::Event(event.clone());
        let frame = WalFrame::new(self.sequence + 1, entry, self.last_frame_hash);
        let event_bytes = event.encode();
        if event_bytes.len() > MAX_EVENT_SIZE {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "event too large"));
        }
        let mut frame_bytes = Vec::with_capacity(FRAME_HEADER_SIZE + event_bytes.len());
        frame_bytes.extend_from_slice(&frame.sequence.to_le_bytes());
        frame_bytes.extend_from_slice(&frame.checksum);
        frame_bytes.extend_from_slice(&frame.previous_frame_hash);
        frame_bytes.extend_from_slice(&(event_bytes.len() as u32).to_le_bytes());
        frame_bytes.push(0u8);
        frame_bytes.extend_from_slice(&[0u8; 3]);
        frame_bytes.extend_from_slice(&event_bytes);
        file.seek(SeekFrom::End(0))?;
        file.write_all(&frame_bytes)?;
        file.sync_all()?;
        self.last_frame_hash = frame.checksum;
        self.sequence = frame.sequence;
        self.frame_count += 1;
        Ok(())
    }

    pub fn iter_events(&self) -> std::io::Result<Vec<ProtocolEvent>> {
        let frames = self.iter_frames()?;
        let mut events = Vec::new();
        for frame in frames {
            match frame.entry {
                WalEntry::Event(event) => events.push(event),
            }
        }
        Ok(events)
    }

    fn iter_frames(&self) -> std::io::Result<Vec<WalFrame>> {
        let mut file = self.file.as_ref().unwrap().try_clone()?;
        file.seek(SeekFrom::Start(WAL_MAGIC.len() as u64))?;
        let file_len = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(WAL_MAGIC.len() as u64))?;
        let mut frames = Vec::new();
        let mut offset = WAL_MAGIC.len() as u64;
        let mut prev_hash = [0u8; 32];
        let mut expected_seq: u64 = 0;

        while offset + FRAME_HEADER_SIZE as u64 <= file_len {
            file.seek(SeekFrom::Start(offset))?;
            let mut header = [0u8; FRAME_HEADER_SIZE];
            if file.read_exact(&mut header).is_err() { break; }
            if !Self::validate_header(&header) { break; }
            let seq = u64::from_le_bytes(header[0..8].try_into().unwrap());
            let checksum: [u8; 32] = header[8..40].try_into().unwrap();
            let frame_prev_hash: [u8; 32] = header[40..72].try_into().unwrap();
            let event_len = u32::from_le_bytes(header[72..76].try_into().unwrap()) as usize;
            if seq != expected_seq + 1 { break; }
            if event_len > MAX_EVENT_SIZE { break; }
            let frame_size = FRAME_HEADER_SIZE + event_len;
            if offset + frame_size as u64 > file_len { break; }
            let mut event_bytes = vec![0u8; event_len];
            file.read_exact(&mut event_bytes)?;
            let event = match ProtocolEvent::decode(&event_bytes) {
                Some(e) => e,
                None => break,
            };
            let entry = WalEntry::Event(event);
            let frame = WalFrame { sequence: seq, entry, checksum, previous_frame_hash: frame_prev_hash };
            if !frame.verify(prev_hash) { break; }
            frames.push(frame);
            prev_hash = checksum;
            expected_seq = seq;
            offset += frame_size as u64;
        }
        Ok(frames)
    }

    pub fn frame_count(&self) -> u64 { self.frame_count }
    pub fn sequence(&self) -> u64 { self.sequence }
}
