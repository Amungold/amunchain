use crate::law::StorageLaw;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicHash32;
use heapless::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateSnapshot {
    pub block_height: u64,
    pub state_root: PublicHash32,
    pub entries: Vec<(Vec<u8, 32>, Vec<u8, 32>), { StorageLaw::MAX_ENTRIES_PER_COMMIT }>,
}

impl StateSnapshot {
    pub fn new(block_height: u64, state_root: PublicHash32) -> Self {
        Self {
            block_height,
            state_root,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, key: &[u8], value: &[u8]) -> Result<(), FailureContext> {
        if self.entries.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000B,
                0x0010,
            ));
        }
        let mut k = Vec::new();
        k.extend_from_slice(key).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0011)
        })?;
        let mut v = Vec::new();
        v.extend_from_slice(value).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0012)
        })?;
        self.entries.push((k, v)).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0013)
        })?;
        Ok(())
    }

    pub fn compute_checksum(&self) -> PublicHash32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"SNAPSHOT_V1:");
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(self.state_root.as_bytes());
        for (k, v) in &self.entries {
            hasher.update(&(k.len() as u16).to_le_bytes());
            hasher.update(k.as_slice());
            hasher.update(&(v.len() as u16).to_le_bytes());
            hasher.update(v.as_slice());
        }
        PublicHash32::new(hasher.finalize().into())
    }
}

impl CanonicalEncode for StateSnapshot {
    const MAX_ENCODED_SIZE: usize = StorageLaw::SNAPSHOT_MAX_SIZE;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.block_height.encode_to_writer(writer)?;
        self.state_root.encode_to_writer(writer)?;
        (self.entries.len() as u32).encode_to_writer(writer)?;
        for (k, v) in &self.entries {
            (k.len() as u16).encode_to_writer(writer)?;
            writer.write_bytes(k.as_slice())?;
            (v.len() as u16).encode_to_writer(writer)?;
            writer.write_bytes(v.as_slice())?;
        }
        Ok(())
    }
}

impl CanonicalDecode for StateSnapshot {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 44 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                0x000B,
                0x0030,
            ));
        }
        let (block_height, len1) = u64::decode(input)?;
        let (state_root, len2) = PublicHash32::decode(&input[len1..])?;
        let mut pos = len1 + len2;

        if input.len() < pos + 4 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                0x000B,
                0x0031,
            ));
        }
        let (entry_count, len3) = u32::decode(&input[pos..])?;
        pos += len3;
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            // Bounds-checked key read
            if input.len() < pos + 2 {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    0x000B,
                    0x0032,
                ));
            }
            let (key_len, len4) = u16::decode(&input[pos..])?;
            pos += len4;
            let key_len = key_len as usize;
            if key_len > 128 {
                return Err(FailureContext::new(
                    ConstitutionalFault::CapacityExceeded,
                    0x000B,
                    0x0033,
                ));
            }
            let end = pos.checked_add(key_len).ok_or_else(|| {
                FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x000B, 0x0034)
            })?;
            if input.len() < end {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    0x000B,
                    0x0035,
                ));
            }
            let mut key = Vec::new();
            key.extend_from_slice(&input[pos..end]).map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0036)
            })?;
            pos = end;

            // Bounds-checked value read
            if input.len() < pos + 2 {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    0x000B,
                    0x0037,
                ));
            }
            let (val_len, len5) = u16::decode(&input[pos..])?;
            pos += len5;
            let val_len = val_len as usize;
            if val_len > 256 {
                return Err(FailureContext::new(
                    ConstitutionalFault::CapacityExceeded,
                    0x000B,
                    0x0038,
                ));
            }
            let end = pos.checked_add(val_len).ok_or_else(|| {
                FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x000B, 0x0039)
            })?;
            if input.len() < end {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    0x000B,
                    0x003A,
                ));
            }
            let mut value = Vec::new();
            value.extend_from_slice(&input[pos..end]).map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x003B)
            })?;
            pos = end;

            entries.push((key, value)).map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x003C)
            })?;
        }
        Ok((
            Self {
                block_height,
                state_root,
                entries,
            },
            pos,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_snapshot_checksum_deterministic() {
    let mut snap = StateSnapshot::new(100, PublicHash32::new([0xAAu8; 32]));
    snap.add_entry(b"key1", b"value1").expect("test invariant");
    assert_eq!(snap.compute_checksum(), snap.compute_checksum());
}

#[test]
fn test_snapshot_roundtrip() {
    let mut snap = StateSnapshot::new(42, PublicHash32::new([0xBBu8; 32]));
    snap.add_entry(b"k1", b"v1").expect("test invariant");
    let mut buf = [0u8; 4096];
    let len = snap.encode(&mut buf).expect("test invariant");
    let (decoded, consumed) = StateSnapshot::decode(&buf[..len]).expect("test invariant");
    assert_eq!(consumed, len);
    assert_eq!(decoded.block_height, 42);
    assert_eq!(decoded.entries.len(), 1);
}
