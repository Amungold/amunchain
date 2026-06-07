use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

// ─── Magic Constants ───────────────────────────────────────────
const FRAME_MAGIC: u32 = 0x414D554E; // "AMUN" in little‑endian
const FOOTER_MAGIC: u64 = 0x414D554E5345414C; // "AMUNSEAL"
const SEGMENT_SUFFIX: &str = ".wal";

// ─── Core State ────────────────────────────────────────────────
#[derive(Debug)]
pub struct WriteAheadLog {
    file: File,
    base_path: String,
    next_sequence: u64,
    segment: u64,
    max_segment_size: u64,
    segment_bytes: u64,
    chain_hash: [u8; 32],
    epoch_id: [u8; 32],
    segment_entry_count: u64,
}

// ─── On‑Disk Structures ────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentFooter {
    segment: u64,
    entry_count: u64,
    last_sequence: u64,
    chain_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WALManifest {
    pub base_path: String,
    pub epoch_id: String,
    pub sealed_segments: Vec<SealedSegmentRecord>,
    pub active_segment: u64,
    pub last_sequence: u64,
    pub chain_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSegmentRecord {
    pub segment: u64,
    pub entry_count: u64,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub chain_hash: String,
    pub file_size: u64,
}

// ─── Entry ─────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WALEntry {
    pub sequence: u64,
    pub entry_type: String,
    pub payload_json: String,
    pub payload_hash: String,
    pub chain_hash: String,
    pub epoch_id: String,
}

// ─── Diagnostics ───────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct WALIntegrity {
    pub total_entries: usize,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub gaps: Vec<u64>,
    pub duplicates: Vec<u64>,
    pub payload_mismatches: Vec<(u64, String)>,
    pub chain_breaks: Vec<u64>,
    pub framing_errors: Vec<u64>,
    pub crc_errors: Vec<u64>,
    pub parse_errors: Vec<u64>,
    pub magic_errors: Vec<u64>,
    pub unsealed_segments: Vec<u64>,
    pub epoch_mismatches: Vec<u64>,
    pub is_clean: bool,
}

#[derive(Debug)]
pub struct PartialRecovery {
    pub entries: Vec<WALEntry>,
    pub recovered_count: usize,
    pub first_error_at_byte: Option<u64>,
    pub error_type: Option<String>,
    pub error_message: Option<String>,
}

// ─── Enums ─────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMode {
    Strict,
    Lenient,
    Forensic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorruptionAction {
    None,
    Abort,
    Truncate,
    Quarantine,
    Rebuild,
}

#[derive(Debug)]
pub struct CorruptionReport {
    pub action: CorruptionAction,
    pub affected_segments: Vec<u64>,
    pub affected_sequences: Vec<u64>,
    pub error_types: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug)]
pub struct AuthorityValidation {
    pub is_valid: bool,
    pub total_entries: usize,
    pub chain_continuous: bool,
    pub no_gaps: bool,
    pub no_duplicates: bool,
    pub epoch_consistent: bool,
    pub all_sealed_segments_valid: bool,
    pub active_segment_healthy: bool,
    pub violations: Vec<String>,
}

// ─── Implementation ────────────────────────────────────────────
impl WriteAheadLog {
    // ── Openers ────────────────────────────────────────────
    pub fn open(base_path: &str) -> Result<Self, String> {
        Self::open_internal(
            base_path,
            64 * 1024 * 1024,
            [0u8; 32],
            RecoveryMode::Lenient,
        )
    }

    pub fn open_strict(base_path: &str) -> Result<Self, String> {
        let wal =
            Self::open_internal(base_path, 64 * 1024 * 1024, [0u8; 32], RecoveryMode::Strict)?;
        let _auth = wal.validate_authority_chain()?;
        Ok(wal)
    }

    pub fn open_with_epoch(base_path: &str, epoch_id: [u8; 32]) -> Result<Self, String> {
        Self::open_internal(base_path, 64 * 1024 * 1024, epoch_id, RecoveryMode::Lenient)
    }

    pub fn open_with_mode(base_path: &str, mode: RecoveryMode) -> Result<Self, String> {
        Self::open_internal(base_path, 64 * 1024 * 1024, [0u8; 32], mode)
    }

    fn open_internal(
        base_path: &str,
        max_segment_size: u64,
        epoch_id: [u8; 32],
        mode: RecoveryMode,
    ) -> Result<Self, String> {
        let segments = Self::list_segments_strict(base_path);
        if mode == RecoveryMode::Strict {
            for seg_num in &segments {
                let seg_path = Self::segment_path(base_path, *seg_num);
                let scan = Self::scan_entries(&seg_path)?;
                if scan.error_type.is_some() {
                    return Err(format!(
                        "Strict mode: corruption in segment {}: {:?}",
                        seg_num, scan.error_type
                    ));
                }
            }
        }
        let (file, segment, segment_bytes) = Self::find_active_segment(base_path)?;
        let mut next_sequence = 1u64;
        let mut chain_hash = [0u8; 32];
        let mut segment_entry_count = 0u64;
        for seg_num in &segments {
            let seg_path = Self::segment_path(base_path, *seg_num);
            let scan = Self::scan_entries(&seg_path)?;
            for entry in &scan.entries {
                next_sequence = next_sequence.max(entry.sequence + 1);
                if let Ok(h) = Self::decode_hash(&entry.chain_hash) {
                    chain_hash = h;
                }
            }
            if *seg_num == segment {
                segment_entry_count = scan.entries.len() as u64;
            }
        }
        Ok(Self {
            file,
            base_path: base_path.to_string(),
            next_sequence,
            segment,
            max_segment_size,
            segment_bytes,
            chain_hash,
            epoch_id,
            segment_entry_count,
        })
    }

    // ── Path Helpers ───────────────────────────────────────
    fn segment_path(base: &str, seg_num: u64) -> String {
        format!("{}.{:08X}{}", base, seg_num, SEGMENT_SUFFIX)
    }

    fn parse_segment_number(filename: &str, base_stem: &str) -> Option<u64> {
        let stripped = filename.strip_prefix(&format!("{}.", base_stem))?;
        let hex_part = stripped.strip_suffix(SEGMENT_SUFFIX)?;
        if hex_part.len() != 8 {
            return None;
        }
        u64::from_str_radix(hex_part, 16).ok()
    }

    fn list_segments_strict(base_path: &str) -> Vec<u64> {
        let dir = Path::new(base_path).parent().unwrap_or(Path::new("."));
        let stem = Path::new(base_path)
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("wal"))
            .to_string_lossy()
            .to_string();
        let mut segments = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                if let Some(seg_num) = Self::parse_segment_number(&fname, &stem) {
                    segments.push(seg_num);
                }
            }
        }
        segments.sort();
        segments
    }

    fn find_active_segment(base_path: &str) -> Result<(File, u64, u64), String> {
        let segments = Self::list_segments_strict(base_path);
        if let Some(&last_seg) = segments.last() {
            let seg_path = Self::segment_path(base_path, last_seg);
            if Self::is_segment_sealed(&seg_path) {
                let new_seg = last_seg + 1;
                let new_path = Self::segment_path(base_path, new_seg);
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .read(true)
                    .open(&new_path)
                    .map_err(|e| format!("Failed to create segment {}: {}", new_seg, e))?;
                return Ok((file, new_seg, 0));
            }
            let file = OpenOptions::new()
                .append(true)
                .read(true)
                .open(&seg_path)
                .map_err(|e| format!("Failed to open segment: {}", e))?;
            let size = file
                .metadata()
                .map(|m| m.len())
                .map_err(|e| format!("Metadata: {}", e))?;
            return Ok((file, last_seg, size));
        }
        let seg_path = Self::segment_path(base_path, 1);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&seg_path)
            .map_err(|e| format!("Failed to create segment 1: {}", e))?;
        Ok((file, 1, 0))
    }

    // ── Segment Sealing ────────────────────────────────────
    pub fn is_segment_sealed(path: &str) -> bool {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let file_size = match file.metadata() {
            Ok(m) => m.len(),
            Err(_) => return false,
        };
        if file_size < 16 {
            return false;
        }
        if file.seek(SeekFrom::End(-8)).is_err() {
            return false;
        }
        let mut magic_buf = [0u8; 8];
        if file.read_exact(&mut magic_buf).is_err() {
            return false;
        }
        if u64::from_le_bytes(magic_buf) != FOOTER_MAGIC {
            return false;
        }
        if file.seek(SeekFrom::End(-12)).is_err() {
            return false;
        }
        let mut crc_buf = [0u8; 4];
        if file.read_exact(&mut crc_buf).is_err() {
            return false;
        }
        let stored_crc = u32::from_le_bytes(crc_buf);
        if file.seek(SeekFrom::End(-16)).is_err() {
            return false;
        }
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            return false;
        }
        let footer_len = u32::from_le_bytes(len_buf) as usize;
        if footer_len == 0 || footer_len > file_size as usize - 16 {
            return false;
        }
        let footer_start = file_size as usize - 16 - footer_len;
        if file.seek(SeekFrom::Start(footer_start as u64)).is_err() {
            return false;
        }
        let mut footer_bytes = vec![0u8; footer_len];
        if file.read_exact(&mut footer_bytes).is_err() {
            return false;
        }
        crc32fast::hash(&footer_bytes) == stored_crc
    }

    pub fn read_segment_footer(path: &str) -> Option<SegmentFooter> {
        let mut file = File::open(path).ok()?;
        let file_size = file.metadata().ok()?.len();
        if file.seek(SeekFrom::End(-16)).is_err() {
            return None;
        }
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            return None;
        }
        let footer_len = u32::from_le_bytes(len_buf) as usize;
        let footer_start = file_size as usize - 16 - footer_len;
        if file.seek(SeekFrom::Start(footer_start as u64)).is_err() {
            return None;
        }
        let mut footer_bytes = vec![0u8; footer_len];
        if file.read_exact(&mut footer_bytes).is_err() {
            return None;
        }
        serde_json::from_slice(&footer_bytes).ok()
    }

    fn rotate(&mut self) -> Result<(), String> {
        self.seal_current_segment()?;
        let new_segment = self.segment + 1;
        let seg_path = Self::segment_path(&self.base_path, new_segment);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&seg_path)
            .map_err(|e| format!("Failed to create segment: {}", e))?;
        self.file = file;
        self.segment = new_segment;
        self.segment_bytes = 0;
        self.segment_entry_count = 0;
        if let Some(parent) = Path::new(&self.base_path).parent() {
            if let Ok(dir_file) = File::open(parent) {
                let _ = dir_file.sync_all();
            }
        }
        Ok(())
    }

    fn seal_current_segment(&mut self) -> Result<(), String> {
        let seg_path = Self::segment_path(&self.base_path, self.segment);
        if Self::is_segment_sealed(&seg_path) {
            return Ok(());
        }
        let footer = SegmentFooter {
            segment: self.segment,
            entry_count: self.segment_entry_count,
            last_sequence: self.next_sequence.saturating_sub(1),
            chain_hash: Self::encode_hash(self.chain_hash),
        };
        let footer_json =
            serde_json::to_vec(&footer).map_err(|e| format!("Footer serialization: {}", e))?;
        let footer_len = footer_json.len() as u32;
        let crc = crc32fast::hash(&footer_json);
        self.file
            .write_all(&footer_json)
            .map_err(|e| format!("Write footer: {}", e))?;
        self.file
            .write_all(&footer_len.to_le_bytes())
            .map_err(|e| format!("Write footer len: {}", e))?;
        self.file
            .write_all(&crc.to_le_bytes())
            .map_err(|e| format!("Write footer CRC: {}", e))?;
        self.file
            .write_all(&FOOTER_MAGIC.to_le_bytes())
            .map_err(|e| format!("Write footer magic: {}", e))?;
        self.file
            .sync_all()
            .map_err(|e| format!("fsync segment: {}", e))?;
        Ok(())
    }

    fn maybe_rotate(&mut self) -> Result<(), String> {
        if self.segment_bytes >= self.max_segment_size {
            self.rotate()?;
        }
        Ok(())
    }

    // ── Write Path ─────────────────────────────────────────
    pub fn append_and_return_entry(
        &mut self,
        entry_type: &str,
        payload_json: &str,
    ) -> Result<WALEntry, String> {
        self.maybe_rotate()?;
        let seq = self.next_sequence;
        let mut hasher = Sha256::new();
        hasher.update(payload_json.as_bytes());
        let payload_hash = Self::encode_hash(hasher.finalize().into());
        let mut chain_hasher = Sha256::new();
        chain_hasher.update(self.chain_hash.as_ref());
        chain_hasher.update(seq.to_le_bytes().as_ref());
        chain_hasher.update(entry_type.as_bytes());
        chain_hasher.update(payload_json.as_bytes());
        let new_chain: [u8; 32] = chain_hasher.finalize().into();
        let entry = WALEntry {
            sequence: seq,
            entry_type: entry_type.to_string(),
            payload_json: payload_json.to_string(),
            payload_hash,
            chain_hash: Self::encode_hash(new_chain),
            epoch_id: Self::encode_hash(self.epoch_id),
        };
        let json_bytes =
            serde_json::to_vec(&entry).map_err(|e| format!("Serialize entry: {}", e))?;
        let crc = crc32fast::hash(&json_bytes);
        let length = json_bytes.len() as u32;
        let mut framed = Vec::with_capacity(4 + 4 + json_bytes.len() + 4);
        framed.extend_from_slice(&FRAME_MAGIC.to_le_bytes());
        framed.extend_from_slice(&length.to_le_bytes());
        framed.extend_from_slice(&json_bytes);
        framed.extend_from_slice(&crc.to_le_bytes());
        self.file
            .write_all(&framed)
            .map_err(|e| format!("Write frame: {}", e))?;
        self.file
            .sync_all()
            .map_err(|e| format!("fsync after frame: {}", e))?;
        self.chain_hash = new_chain;
        self.next_sequence = seq + 1;
        self.segment_bytes += framed.len() as u64;
        self.segment_entry_count += 1;
        Ok(entry)
    }

    pub fn append(&mut self, entry_type: &str, payload_json: &str) -> Result<u64, String> {
        self.append_and_return_entry(entry_type, payload_json)
            .map(|e| e.sequence)
    }

    // ── Read Path ──────────────────────────────────────────
    pub fn read_all(&self) -> Result<Vec<WALEntry>, String> {
        let mut all = Vec::new();
        for seg_num in Self::list_segments_strict(&self.base_path) {
            let scan = Self::scan_entries(&Self::segment_path(&self.base_path, seg_num))?;
            all.extend(scan.entries);
        }
        all.sort_by_key(|e| e.sequence);
        Ok(all)
    }

    pub fn read_from_sequence(&self, from_seq: u64) -> Result<Vec<WALEntry>, String> {
        Ok(self
            .read_all()?
            .into_iter()
            .filter(|e| e.sequence >= from_seq)
            .collect())
    }

    fn scan_entries(path: &str) -> Result<PartialRecovery, String> {
        let mut file = File::open(path).map_err(|e| format!("Open '{}': {}", path, e))?;
        let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        let data_end = Self::find_footer_start(path).unwrap_or(file_size);
        let mut entries = Vec::new();
        let mut error_at = None;
        let mut error_type = None;
        let mut error_msg = None;
        let mut byte_offset = 0u64;
        loop {
            if byte_offset >= data_end {
                break;
            }
            let mut magic_buf = [0u8; 4];
            match file.read_exact(&mut magic_buf) {
                Ok(()) => {
                    byte_offset += 4;
                    if u32::from_le_bytes(magic_buf) != FRAME_MAGIC {
                        error_at = Some(byte_offset - 4);
                        error_type = Some("magic".to_string());
                        break;
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    error_at = Some(byte_offset);
                    error_type = Some("io".to_string());
                    break;
                }
            }
            let mut len_buf = [0u8; 4];
            if file.read_exact(&mut len_buf).is_err() {
                error_at = Some(byte_offset);
                error_type = Some("framing".to_string());
                break;
            }
            byte_offset += 4;
            let length = u32::from_le_bytes(len_buf) as usize;
            if length > 10_000_000 || length == 0 {
                error_at = Some(byte_offset);
                error_type = Some("framing".to_string());
                break;
            }
            let mut json_bytes = vec![0u8; length];
            if let Err(e) = file.read_exact(&mut json_bytes) {
                error_at = Some(byte_offset);
                error_type = Some("framing".to_string());
                error_msg = Some(format!("{}", e));
                break;
            }
            byte_offset += length as u64;
            let mut crc_buf = [0u8; 4];
            if file.read_exact(&mut crc_buf).is_err() {
                error_at = Some(byte_offset);
                error_type = Some("framing".to_string());
                break;
            }
            byte_offset += 4;
            if u32::from_le_bytes(crc_buf) != crc32fast::hash(&json_bytes) {
                error_at = Some(byte_offset - 4);
                error_type = Some("crc".to_string());
                break;
            }
            match serde_json::from_slice::<WALEntry>(&json_bytes) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    error_at = Some(byte_offset);
                    error_type = Some("parse".to_string());
                    error_msg = Some(format!("{}", e));
                    break;
                }
            }
        }
        Ok(PartialRecovery {
            recovered_count: entries.len(),
            entries,
            first_error_at_byte: error_at,
            error_type,
            error_message: error_msg,
        })
    }

    fn find_footer_start(path: &str) -> Option<u64> {
        let mut file = File::open(path).ok()?;
        let file_size = file.metadata().ok()?.len();
        if file_size < 16 {
            return Some(file_size);
        }
        if file.seek(SeekFrom::End(-8)).is_err() {
            return Some(file_size);
        }
        let mut magic_buf = [0u8; 8];
        if file.read_exact(&mut magic_buf).is_err() {
            return Some(file_size);
        }
        if u64::from_le_bytes(magic_buf) != FOOTER_MAGIC {
            return Some(file_size);
        }
        if file.seek(SeekFrom::End(-16)).is_err() {
            return Some(file_size);
        }
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            return Some(file_size);
        }
        let footer_len = u32::from_le_bytes(len_buf) as u64;
        if footer_len == 0 || footer_len > file_size - 16 {
            return Some(file_size);
        }
        Some(file_size - 16 - footer_len)
    }

    #[allow(clippy::needless_borrows_for_generic_args)]
    // ── Integrity / Authority ──────────────────────────────
    pub fn verify_chain_continuity(entries: &[WALEntry]) -> Result<(), String> {
        if entries.is_empty() {
            return Ok(());
        }
        let mut expected_chain = [0u8; 32];
        let mut prev_seq = 0u64;
        for entry in entries {
            if prev_seq != 0 && entry.sequence != prev_seq + 1 {
                return Err(format!(
                    "Sequence gap at {} expected {}",
                    entry.sequence,
                    prev_seq + 1
                ));
            }
            let mut hasher = Sha256::new();
            hasher.update(expected_chain.as_ref());
            hasher.update(entry.sequence.to_le_bytes().as_ref());
            hasher.update(entry.entry_type.as_bytes());
            hasher.update(entry.payload_json.as_bytes());
            let computed: [u8; 32] = hasher.finalize().into();
            if Self::encode_hash(computed) != entry.chain_hash {
                return Err(format!(
                    "Chain hash mismatch at sequence {}",
                    entry.sequence
                ));
            }
            expected_chain = computed;
            prev_seq = entry.sequence;
        }
        Ok(())
    }

    pub fn check_integrity(&self) -> Result<WALIntegrity, String> {
        let mut seq_set = BTreeSet::new();
        let mut duplicates = Vec::new();
        let mut payload_mismatches = Vec::new();
        let mut chain_breaks = Vec::new();
        let mut framing_errs = Vec::new();
        let mut crc_errs = Vec::new();
        let mut parse_errs = Vec::new();
        let mut magic_errs = Vec::new();
        let mut unsealed = Vec::new();
        let mut epoch_mismatches = Vec::new();
        let mut first = u64::MAX;
        let mut last = 0u64;
        let mut seq_payload_map: std::collections::BTreeMap<u64, String> =
            std::collections::BTreeMap::new();
        let mut expected_chain = [0u8; 32];
        let mut prev_seq = 0u64;
        let expected_epoch = Self::encode_hash(self.epoch_id);
        for seg_num in Self::list_segments_strict(&self.base_path) {
            let seg_path = Self::segment_path(&self.base_path, seg_num);
            let recovery = Self::scan_entries(&seg_path)?;
            for entry in &recovery.entries {
                if !seq_set.insert(entry.sequence) {
                    duplicates.push(entry.sequence);
                }
                if let Some(existing) = seq_payload_map.get(&entry.sequence) {
                    if *existing != entry.payload_hash {
                        payload_mismatches.push((entry.sequence, "mismatch".into()));
                    }
                } else {
                    seq_payload_map.insert(entry.sequence, entry.payload_hash.clone());
                }
                if entry.epoch_id != expected_epoch
                    && expected_epoch != Self::encode_hash([0u8; 32])
                {
                    epoch_mismatches.push(entry.sequence);
                }
                if entry.sequence == prev_seq + 1 || prev_seq == 0 {
                    let mut hasher = Sha256::new();
                    hasher.update(expected_chain.as_ref());
                    hasher.update(entry.sequence.to_le_bytes().as_ref());
                    hasher.update(entry.entry_type.as_bytes());
                    hasher.update(entry.payload_json.as_bytes());
                    let computed: [u8; 32] = hasher.finalize().into();
                    if Self::encode_hash(computed) != entry.chain_hash {
                        chain_breaks.push(entry.sequence);
                    }
                    if let Ok(h) = Self::decode_hash(&entry.chain_hash) {
                        expected_chain = h;
                    }
                } else {
                    chain_breaks.push(entry.sequence);
                    if let Ok(h) = Self::decode_hash(&entry.chain_hash) {
                        expected_chain = h;
                    }
                }
                prev_seq = entry.sequence;
                first = first.min(entry.sequence);
                last = last.max(entry.sequence);
            }
            if let Some(ref err_type) = recovery.error_type {
                match err_type.as_str() {
                    "magic" => magic_errs.push(recovery.first_error_at_byte.unwrap_or(0)),
                    "framing" => framing_errs.push(recovery.first_error_at_byte.unwrap_or(0)),
                    "crc" => crc_errs.push(recovery.first_error_at_byte.unwrap_or(0)),
                    "parse" => parse_errs.push(recovery.first_error_at_byte.unwrap_or(0)),
                    _ => {}
                }
            }
            if !Self::is_segment_sealed(&seg_path) && seg_num != self.segment {
                unsealed.push(seg_num);
            }
        }
        let mut gaps = Vec::new();
        if !seq_set.is_empty() {
            let expected_first = *seq_set.first().unwrap();
            let actual_last = *seq_set.last().unwrap();
            for seq in expected_first..=actual_last {
                if !seq_set.contains(&seq) {
                    gaps.push(seq);
                }
            }
        }
        let total = seq_set.len();
        let is_clean = gaps.is_empty()
            && duplicates.is_empty()
            && payload_mismatches.is_empty()
            && chain_breaks.is_empty()
            && framing_errs.is_empty()
            && crc_errs.is_empty()
            && parse_errs.is_empty()
            && magic_errs.is_empty()
            && epoch_mismatches.is_empty()
            && total > 0;
        Ok(WALIntegrity {
            total_entries: total,
            first_sequence: if total > 0 { first } else { 0 },
            last_sequence: if total > 0 { last } else { 0 },
            gaps,
            duplicates,
            payload_mismatches,
            chain_breaks,
            framing_errors: framing_errs,
            crc_errors: crc_errs,
            parse_errors: parse_errs,
            magic_errors: magic_errs,
            unsealed_segments: unsealed,
            epoch_mismatches,
            is_clean,
        })
    }

    pub fn validate_authority_chain(&self) -> Result<AuthorityValidation, String> {
        let integrity = self.check_integrity()?;
        let mut violations = Vec::new();
        let all_segs = Self::list_segments_strict(&self.base_path);
        let unsealed_count = integrity.unsealed_segments.len();
        if unsealed_count > 1 {
            violations.push(format!("Multiple unsealed segments: {}", unsealed_count));
        }
        if unsealed_count == 1 && all_segs.last() != integrity.unsealed_segments.first() {
            violations.push("Unsealed segment is not the latest segment".to_string());
        }
        let mut all_sealed_valid = true;
        for seg_num in &all_segs {
            let seg_path = Self::segment_path(&self.base_path, *seg_num);
            if Self::is_segment_sealed(&seg_path) && Self::read_segment_footer(&seg_path).is_none()
            {
                violations.push(format!("Segment {} footer unreadable", seg_num));
                all_sealed_valid = false;
            }
        }
        let active_path = Self::segment_path(&self.base_path, self.segment);
        let active_healthy = Self::scan_entries(&active_path)
            .map(|r| r.error_type.is_none())
            .unwrap_or(false);
        Ok(AuthorityValidation {
            is_valid: violations.is_empty(),
            total_entries: integrity.total_entries,
            chain_continuous: integrity.chain_breaks.is_empty(),
            no_gaps: integrity.gaps.is_empty(),
            no_duplicates: integrity.duplicates.is_empty(),
            epoch_consistent: integrity.epoch_mismatches.is_empty(),
            all_sealed_segments_valid: all_sealed_valid,
            active_segment_healthy: active_healthy,
            violations,
        })
    }

    pub fn determine_corruption_action(&self) -> Result<CorruptionReport, String> {
        let integrity = self.check_integrity()?;
        let mut affected_sequences = Vec::new();
        let mut error_types = Vec::new();
        if !integrity.magic_errors.is_empty() {
            error_types.push("magic".to_string());
            affected_sequences.extend(&integrity.magic_errors);
        }
        if !integrity.framing_errors.is_empty() {
            error_types.push("framing".to_string());
            affected_sequences.extend(&integrity.framing_errors);
        }
        if !integrity.crc_errors.is_empty() {
            error_types.push("crc".to_string());
            affected_sequences.extend(&integrity.crc_errors);
        }
        if !integrity.parse_errors.is_empty() {
            error_types.push("parse".to_string());
            affected_sequences.extend(&integrity.parse_errors);
        }
        if !integrity.chain_breaks.is_empty() {
            error_types.push("chain_break".to_string());
            affected_sequences.extend(&integrity.chain_breaks);
        }
        if !integrity.gaps.is_empty() {
            error_types.push("gap".to_string());
            affected_sequences.extend(&integrity.gaps);
        }
        let (action, recommendation) = if integrity.is_clean {
            (CorruptionAction::None, "WAL is clean".to_string())
        } else if !integrity.chain_breaks.is_empty() {
            (
                CorruptionAction::Abort,
                "Chain integrity broken — manual intervention required".to_string(),
            )
        } else if !integrity.framing_errors.is_empty()
            || !integrity.crc_errors.is_empty()
            || !integrity.magic_errors.is_empty()
        {
            (
                CorruptionAction::Truncate,
                "Physical corruption — truncate at last valid entry".to_string(),
            )
        } else if !integrity.gaps.is_empty() {
            (
                CorruptionAction::Quarantine,
                "Sequence gaps detected — quarantine affected range".to_string(),
            )
        } else {
            (
                CorruptionAction::Rebuild,
                "Rebuild from sealed segments".to_string(),
            )
        };
        Ok(CorruptionReport {
            action,
            affected_segments: Vec::new(),
            affected_sequences,
            error_types,
            recommendation,
        })
    }

    pub fn build_manifest(&self) -> WALManifest {
        let mut sealed = Vec::new();
        for seg_num in Self::list_segments_strict(&self.base_path) {
            let seg_path = Self::segment_path(&self.base_path, seg_num);
            if Self::is_segment_sealed(&seg_path) {
                if let Some(footer) = Self::read_segment_footer(&seg_path) {
                    let entries = Self::scan_entries(&seg_path).ok();
                    let first_seq = entries
                        .as_ref()
                        .and_then(|e| e.entries.first().map(|e| e.sequence))
                        .unwrap_or(0);
                    sealed.push(SealedSegmentRecord {
                        segment: seg_num,
                        entry_count: footer.entry_count,
                        first_sequence: first_seq,
                        last_sequence: footer.last_sequence,
                        chain_hash: footer.chain_hash,
                        file_size: std::fs::metadata(&seg_path).map(|m| m.len()).unwrap_or(0),
                    });
                }
            }
        }
        WALManifest {
            base_path: self.base_path.clone(),
            epoch_id: Self::encode_hash(self.epoch_id),
            sealed_segments: sealed,
            active_segment: self.segment,
            last_sequence: self.next_sequence.saturating_sub(1),
            chain_hash: Self::encode_hash(self.chain_hash),
        }
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.seal_current_segment()
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
    pub fn segment(&self) -> u64 {
        self.segment
    }
    pub fn chain_hash(&self) -> [u8; 32] {
        self.chain_hash
    }
    pub fn epoch_id(&self) -> [u8; 32] {
        self.epoch_id
    }
    pub fn list_segments_public(base_path: &str) -> Vec<u64> {
        Self::list_segments_strict(base_path)
    }
    pub fn encode_hash(hash: [u8; 32]) -> String {
        hash.iter().map(|b| format!("{:02x}", b)).collect()
    }
    pub fn decode_hash(hex: &str) -> Result<[u8; 32], String> {
        if hex.len() != 64 {
            return Err("Invalid hex length".into());
        }
        let mut arr = [0u8; 32];
        for i in 0..32 {
            arr[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("Hex decode: {}", e))?;
        }
        Ok(arr)
    }

    pub fn reset_for_testing(base_path: &str) -> Result<(), String> {
        for seg in Self::list_segments_strict(base_path) {
            let path = Self::segment_path(base_path, seg);
            let _ = std::fs::remove_file(path);
        }
        Ok(())
    }
}

impl Drop for WriteAheadLog {
    fn drop(&mut self) {
        let _ = self.seal_current_segment();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_roundtrip() {
        let path = "/tmp/amun_test_ft";
        let _ = WriteAheadLog::reset_for_testing(path);
        {
            let mut wal = WriteAheadLog::open(path).unwrap();
            wal.append("QC", r#"{"b":"1"}"#).unwrap();
            wal.append("C", r#"{"b":"1"}"#).unwrap();
        }
        assert!(WriteAheadLog::is_segment_sealed(
            &WriteAheadLog::segment_path(path, 1)
        ));
        assert_eq!(
            WriteAheadLog::read_segment_footer(&WriteAheadLog::segment_path(path, 1))
                .unwrap()
                .entry_count,
            2
        );
        let _ = WriteAheadLog::reset_for_testing(path);
    }

    #[test]
    fn test_clean() {
        let path = "/tmp/amun_test_cl";
        let _ = WriteAheadLog::reset_for_testing(path);
        let mut wal = WriteAheadLog::open(path).unwrap();
        wal.append("Q", r#"{"b":"1"}"#).unwrap();
        wal.append("C", r#"{"b":"1"}"#).unwrap();
        assert!(wal.check_integrity().unwrap().is_clean);
        let _ = WriteAheadLog::reset_for_testing(path);
    }

    #[test]
    fn test_entry() {
        let path = "/tmp/amun_test_ent";
        let _ = WriteAheadLog::reset_for_testing(path);
        let mut wal = WriteAheadLog::open(path).unwrap();
        let e = wal.append_and_return_entry("QC", r#"{"b":"1"}"#).unwrap();
        assert_eq!(e.sequence, 1);
        assert_eq!(e.entry_type, "QC");
        let _ = WriteAheadLog::reset_for_testing(path);
    }

    #[test]
    fn test_strict_mode_rejects_corrupt_segment() {
        let path = "/tmp/amun_test_str";
        let _ = WriteAheadLog::reset_for_testing(path);
        std::fs::write(WriteAheadLog::segment_path(path, 1), b"garbage").unwrap();
        assert!(WriteAheadLog::open_strict(path).is_err());
        let _ = WriteAheadLog::reset_for_testing(path);
    }

    #[test]
    fn test_policy_clean() {
        let path = "/tmp/amun_test_pol";
        let _ = WriteAheadLog::reset_for_testing(path);
        let mut wal = WriteAheadLog::open(path).unwrap();
        wal.append("Q", r#"{"b":"1"}"#).unwrap();
        assert_eq!(
            wal.determine_corruption_action().unwrap().action,
            CorruptionAction::None
        );
        let _ = WriteAheadLog::reset_for_testing(path);
    }

    #[test]
    fn test_magic_error_detected() {
        let path = "/tmp/amun_test_magic";
        let _ = WriteAheadLog::reset_for_testing(path);
        {
            let mut wal = WriteAheadLog::open(path).unwrap();
            wal.append("X", r#"{"k":"v"}"#).unwrap();
        }
        let seg1 = WriteAheadLog::segment_path(path, 1);
        let mut data = match std::fs::read(&seg1) { Ok(d) => d, Err(e) => { eprintln!("WAL: cannot read segment: {}", e); return; } };
        data[0] = 0xFF;
        std::fs::write(&seg1, &data).unwrap();
        let wal = WriteAheadLog::open(path).unwrap();
        let integrity = wal.check_integrity().unwrap();
        assert!(!integrity.is_clean);
        assert!(!integrity.magic_errors.is_empty());
        let _ = WriteAheadLog::reset_for_testing(path);
    }
}
