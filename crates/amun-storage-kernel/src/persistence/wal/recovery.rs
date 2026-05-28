use super::entry::WalEntry;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn recover_sequence(file: &File) -> Result<u64, &'static str> {
    let mut reader = file.try_clone().map_err(|_| "clone failed")?;
    reader.seek(SeekFrom::Start(0)).map_err(|_| "seek failed")?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|_| "read failed")?;

    let mut pos = 0;
    let mut last_valid_seq: Option<u64> = None;

    while pos + 4 <= buffer.len() {
        let len = u32::from_le_bytes([
            buffer[pos],
            buffer[pos + 1],
            buffer[pos + 2],
            buffer[pos + 3],
        ]) as usize;
        pos += 4;
        if pos + len > buffer.len() {
            break;
        }

        if let Some(entry) = WalEntry::decode(&buffer[pos..pos + len]) {
            last_valid_seq = Some(entry.sequence);
        } else {
            break;
        }
        pos += len;
    }

    Ok(last_valid_seq.map(|s| s + 1).unwrap_or(0))
}
