use std::fs::{File, OpenOptions};
use std::io::{Write, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crc32fast::Hasher;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum WALEntry {
    QcJustified { seq: u64, epoch: u64, qc_hash: [u8; 32], height: u64, round: u64 },
    QcLocked { seq: u64, epoch: u64, qc_hash: [u8; 32], height: u64, round: u64 },
    QcFinalized { seq: u64, epoch: u64, qc_hash: [u8; 32], height: u64, round: u64 },
    Vote { seq: u64, epoch: u64, vid: u64, round: u64, vote_hash: [u8; 32] },
    RoundAdvance { seq: u64, epoch: u64, round: u64 },
    Snapshot { seq: u64, epoch: u64, state_hash: [u8; 32], height: u64 },
}
impl WALEntry { pub fn seq(&self) -> u64 { match self { Self::QcJustified { seq, .. } => *seq, Self::QcLocked { seq, .. } => *seq, Self::QcFinalized { seq, .. } => *seq, Self::Vote { seq, .. } => *seq, Self::RoundAdvance { seq, .. } => *seq, Self::Snapshot { seq, .. } => *seq } } }

#[derive(Debug)]
pub struct AppendOnlyWAL { path: PathBuf, file: File, next_seq: u64 }
impl AppendOnlyWAL {
    pub fn new(data_dir: &str) -> std::io::Result<Self> {
        let path = PathBuf::from(data_dir).join("consensus.wal");
        let file = OpenOptions::new().create(true).append(true).read(true).open(&path)?;
        let mut buf = Vec::new(); let mut f_clone = file.try_clone()?; f_clone.seek(SeekFrom::Start(0))?; f_clone.read_to_end(&mut buf)?;
        let mut max_seq = 0; let mut pos = 0;
        while pos + 12 <= buf.len() {
            let len = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize; pos += 4; pos += 4;
            if pos + len > buf.len() { break; }
            if let Ok(entry) = bincode::deserialize::<WALEntry>(&buf[pos..pos+len]) { max_seq = max_seq.max(entry.seq()); }
            pos += len;
        }
        Ok(Self { path, file, next_seq: max_seq + 1 })
    }
    pub fn append(&mut self, mut entry: WALEntry) -> std::io::Result<()> {
        match &mut entry {
            WALEntry::QcJustified { seq, .. } => *seq = self.next_seq,
            WALEntry::QcLocked { seq, .. } => *seq = self.next_seq,
            WALEntry::QcFinalized { seq, .. } => *seq = self.next_seq,
            WALEntry::Vote { seq, .. } => *seq = self.next_seq,
            WALEntry::RoundAdvance { seq, .. } => *seq = self.next_seq,
            WALEntry::Snapshot { seq, .. } => *seq = self.next_seq,
        }
        let bytes = bincode::serialize(&entry).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut hasher = Hasher::new(); hasher.update(&bytes);
        let len_bytes = (bytes.len() as u32).to_be_bytes();
        self.file.write_all(&len_bytes)?; self.file.write_all(&hasher.finalize().to_be_bytes())?; self.file.write_all(&bytes)?; self.file.sync_all()?;
        self.next_seq += 1; Ok(())
    }
    pub fn replay(&mut self) -> std::io::Result<Vec<WALEntry>> {
        let mut entries = Vec::new(); let mut buf = Vec::new(); self.file.seek(SeekFrom::Start(0))?; self.file.read_to_end(&mut buf)?;
        let mut pos = 0; let mut expected_seq = 0; let mut first_seq_seen = false;
        while pos + 12 <= buf.len() {
            let len = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize; pos += 4;
            let stored_crc = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]); pos += 4;
            if pos + len > buf.len() { break; }
            let mut hasher = Hasher::new(); hasher.update(&buf[pos..pos+len]);
            if hasher.finalize() != stored_crc { break; }
            if let Ok(entry) = bincode::deserialize::<WALEntry>(&buf[pos..pos+len]) {
                if !first_seq_seen { expected_seq = entry.seq(); first_seq_seen = true; }
                if entry.seq() == expected_seq { entries.push(entry); expected_seq += 1; } else { break; }
            }
            pos += len;
        }
        Ok(entries)
    }
}
