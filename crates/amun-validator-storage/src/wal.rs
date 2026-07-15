use amun_validator_api::error::{PlatformError, PlatformResult, StorageError, StorageErrorCode};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const WAL_MAGIC: &[u8; 4] = b"AMWL";

/// A single entry in the Write-Ahead Log — binary format with frame-length prefix.
///
/// On-disk format per record:
///   frame_length(4) + magic(4) + sequence(8) + height(8) + op_len(4) + op + payload_len(4) + payload + checksum(4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalEntry {
    pub sequence: u64,
    pub height: u64,
    pub operation: String,
    pub payload: Vec<u8>,
    pub checksum: u32,
}

impl WalEntry {
    pub fn new(sequence: u64, height: u64, operation: String, payload: Vec<u8>) -> Self {
        let checksum = Self::compute_checksum(&payload);
        WalEntry {
            sequence,
            height,
            operation,
            payload,
            checksum,
        }
    }

    fn compute_checksum(data: &[u8]) -> u32 {
        let mut hash: u32 = 0x811C9DC5;
        for &byte in data {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        hash
    }

    pub fn verify_checksum(&self) -> bool {
        Self::compute_checksum(&self.payload) == self.checksum
    }

    /// Encode the entry payload (without frame_length prefix).
    fn encode_payload(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(WAL_MAGIC);
        buf.extend_from_slice(&self.sequence.to_le_bytes());
        buf.extend_from_slice(&self.height.to_le_bytes());
        let op_bytes = self.operation.as_bytes();
        buf.extend_from_slice(&(op_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(op_bytes);
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.payload);
        buf.extend_from_slice(&self.checksum.to_le_bytes());
        buf
    }

    /// Encode the full record: frame_length(4) + payload.
    pub fn encode(&self) -> Vec<u8> {
        let payload = self.encode_payload();
        let frame_len = payload.len() as u32;
        let mut frame = Vec::with_capacity(4 + payload.len());
        frame.extend_from_slice(&frame_len.to_le_bytes());
        frame.extend_from_slice(&payload);
        frame
    }

    /// Decode a frame. Returns Some(WalEntry, bytes_consumed) or None if data is incomplete/corrupt.
    pub fn decode_frame(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 4 {
            return None;
        }
        let frame_len = u32::from_le_bytes(data[0..4].try_into().ok()?) as usize;
        if data.len() < 4 + frame_len {
            return None;
        }
        let entry = Self::decode_payload(&data[4..4 + frame_len])?;
        Some((entry, 4 + frame_len))
    }

    /// Decode a single entry payload (without frame_length).
    fn decode_payload(data: &[u8]) -> Option<Self> {
        if data.len() < 32 {
            return None;
        }
        if &data[0..4] != WAL_MAGIC {
            return None;
        }
        let sequence = u64::from_le_bytes(data[4..12].try_into().ok()?);
        let height = u64::from_le_bytes(data[12..20].try_into().ok()?);
        let op_len = u32::from_le_bytes(data[20..24].try_into().ok()?) as usize;
        let op_start = 24;
        if data.len() < op_start + op_len + 4 {
            return None;
        }
        let operation = String::from_utf8(data[op_start..op_start + op_len].to_vec()).ok()?;
        let payload_len_start = op_start + op_len;
        let payload_len = u32::from_le_bytes(
            data[payload_len_start..payload_len_start + 4]
                .try_into()
                .ok()?,
        ) as usize;
        let payload_start = payload_len_start + 4;
        if data.len() < payload_start + payload_len + 4 {
            return None;
        }
        let payload = data[payload_start..payload_start + payload_len].to_vec();
        let checksum = u32::from_le_bytes(
            data[payload_start + payload_len..payload_start + payload_len + 4]
                .try_into()
                .ok()?,
        );
        Some(WalEntry {
            sequence,
            height,
            operation,
            payload,
            checksum,
        })
    }
}

/// Writer for appending entries to the WAL — binary format with frame-length prefix.
pub struct WalWriter {
    file: Mutex<BufWriter<File>>,
    path: PathBuf,
    sequence: Mutex<u64>,
}

impl WalWriter {
    pub fn open(path: &Path) -> PlatformResult<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| {
                PlatformError::Storage(StorageError::new(
                    StorageErrorCode::InitializationFailed,
                    format!("WAL open: {}", e),
                ))
            })?;
        let seq = Self::read_max_sequence(path)?;
        Ok(WalWriter {
            file: Mutex::new(BufWriter::new(file)),
            path: path.to_path_buf(),
            sequence: Mutex::new(seq),
        })
    }

    fn read_max_sequence(path: &Path) -> PlatformResult<u64> {
        if !path.exists() {
            return Ok(0);
        }
        let data = std::fs::read(path).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL read: {}", e),
            ))
        })?;
        let mut max_seq = 0u64;
        let mut offset = 0;
        while let Some((entry, consumed)) = WalEntry::decode_frame(&data[offset..]) {
            max_seq = max_seq.max(entry.sequence);
            offset += consumed;
        }
        Ok(max_seq)
    }

    pub fn append(
        &self,
        height: u64,
        operation: &str,
        payload: Vec<u8>,
    ) -> PlatformResult<WalEntry> {
        let mut seq = self.sequence.lock().unwrap_or_else(|e| e.into_inner());
        *seq += 1;
        let entry = WalEntry::new(*seq, height, operation.to_string(), payload);
        let encoded = entry.encode();
        let mut file = self.file.lock().unwrap_or_else(|e| e.into_inner());
        file.write_all(&encoded).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL write: {}", e),
            ))
        })?;
        file.flush().map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL flush: {}", e),
            ))
        })?;
        Ok(entry)
    }

    pub fn sync(&self) -> PlatformResult<()> {
        let mut file = self.file.lock().unwrap_or_else(|e| e.into_inner());
        file.flush().map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL flush: {}", e),
            ))
        })?;
        file.get_ref().sync_all().map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL sync: {}", e),
            ))
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn sequence(&self) -> u64 {
        *self.sequence.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Write-Ahead Log for crash recovery.
pub struct WriteAheadLog {
    path: PathBuf,
    writer: Option<WalWriter>,
}

impl WriteAheadLog {
    pub fn new(dir: &Path) -> PlatformResult<Self> {
        std::fs::create_dir_all(dir).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::InitializationFailed,
                format!("WAL dir: {}", e),
            ))
        })?;
        Ok(WriteAheadLog {
            path: dir.to_path_buf(),
            writer: None,
        })
    }

    pub fn writer(&self) -> PlatformResult<&WalWriter> {
        self.writer.as_ref().ok_or_else(|| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::InitializationFailed,
                "WAL not opened".into(),
            ))
        })
    }

    pub fn open(&mut self) -> PlatformResult<&WalWriter> {
        let wal_path = self.path.join("wal.log");
        let writer = WalWriter::open(&wal_path)?;
        self.writer = Some(writer);
        Ok(self.writer.as_ref().unwrap())
    }

    /// Replay all valid entries from the WAL file.
    /// Invalid/corrupt frames are skipped; only complete valid frames are returned.
    pub fn replay(&self) -> PlatformResult<Vec<WalEntry>> {
        let wal_path = self.path.join("wal.log");
        if !wal_path.exists() {
            return Ok(vec![]);
        }
        let data = std::fs::read(&wal_path).map_err(|e| {
            PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                format!("WAL read: {}", e),
            ))
        })?;
        let mut entries = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            if let Some((entry, consumed)) = WalEntry::decode_frame(&data[offset..]) {
                if !entry.verify_checksum() {
                    return Err(PlatformError::Storage(StorageError::new(
                        StorageErrorCode::WalCorrupted,
                        format!("Checksum mismatch at seq {}", entry.sequence),
                    )));
                }
                entries.push(entry);
                offset += consumed;
            } else {
                // Incomplete/corrupt frame — stop at last valid position
                break;
            }
        }
        Ok(entries)
    }

    pub fn is_healthy(&self) -> bool {
        self.path.exists()
    }

    pub fn verify(&self) -> PlatformResult<()> {
        if !self.is_healthy() {
            return Err(PlatformError::Storage(StorageError::new(
                StorageErrorCode::WalReplayFailed,
                "WAL directory missing".into(),
            )));
        }
        let wal_path = self.path.join("wal.log");
        if wal_path.exists() {
            let _ = self.replay()?;
        }
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_wal_creation() {
        let dir = test_dir();
        let wal = WriteAheadLog::new(dir.path()).unwrap();
        assert!(wal.is_healthy());
    }

    #[test]
    fn test_append_replay() {
        let dir = test_dir();
        let mut wal = WriteAheadLog::new(dir.path()).unwrap();
        wal.open().unwrap();
        wal.writer()
            .unwrap()
            .append(1, "put_block", vec![1, 2, 3])
            .unwrap();
        wal.writer()
            .unwrap()
            .append(2, "put_block", vec![4, 5, 6])
            .unwrap();
        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].height, 1);
        assert_eq!(entries[1].height, 2);
    }

    #[test]
    fn test_checksum() {
        let entry = WalEntry::new(1, 10, "test".into(), vec![1, 2, 3]);
        assert!(entry.verify_checksum());
    }

    #[test]
    fn test_checksum_failure() {
        let mut entry = WalEntry::new(1, 10, "test".into(), vec![1, 2, 3]);
        entry.payload.push(99);
        assert!(!entry.verify_checksum());
    }

    #[test]
    fn test_empty_replay() {
        let dir = test_dir();
        let wal = WriteAheadLog::new(dir.path()).unwrap();
        let entries = wal.replay().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_binary_roundtrip() {
        let entry = WalEntry::new(1, 100, "put_block".into(), vec![0xAA; 64]);
        let encoded = entry.encode();
        let (decoded, consumed) = WalEntry::decode_frame(&encoded).unwrap();
        assert_eq!(decoded.sequence, 1);
        assert_eq!(decoded.height, 100);
        assert_eq!(decoded.operation, "put_block");
        assert_eq!(decoded.payload, vec![0xAA; 64]);
        assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_sync() {
        let dir = test_dir();
        let mut wal = WriteAheadLog::new(dir.path()).unwrap();
        wal.open().unwrap();
        wal.writer().unwrap().append(1, "test", vec![1]).unwrap();
        assert!(wal.writer().unwrap().sync().is_ok());
    }

    #[test]
    fn test_corrupt_frame_skipped() {
        let dir = test_dir();
        let wal_dir = dir.path().join("wal");
        let wal_path = wal_dir.join("wal.log");
        std::fs::create_dir_all(&wal_dir).unwrap();
        let entry = WalEntry::new(1, 100, "update_height".into(), vec![1]);
        let valid_frame = entry.encode();
        let mut data = valid_frame.clone();
        data.extend_from_slice(b"GARBAGE_DATA");
        std::fs::write(&wal_path, &data).unwrap();
        let wal = WriteAheadLog::new(&wal_dir).unwrap();
        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 1, "Should recover 1 valid entry");
        assert_eq!(entries[0].height, 100);
    }

    #[test]
    fn test_truncated_frame_handled() {
        let dir = test_dir();
        let wal_path = dir.path().join("wal").join("wal.log");
        std::fs::create_dir_all(dir.path().join("wal")).unwrap();
        let entry = WalEntry::new(1, 100, "test".into(), vec![1, 2, 3]);
        let encoded = entry.encode();
        // Write partial data
        std::fs::write(&wal_path, &encoded[..encoded.len() - 10]).unwrap();
        let wal = WriteAheadLog::new(&dir.path().join("wal")).unwrap();
        let entries = wal.replay().unwrap();
        assert!(entries.is_empty());
    }
}
