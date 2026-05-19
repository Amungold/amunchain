use amun_kernel_types::PublicHash32;
use amun_failure::{ConstitutionalFault, FailureContext};
use heapless::Vec;
use crate::law::StorageLaw;
use blake3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WalPayload {
    Set { key: Vec<u8, 32>, value: Vec<u8, 64> },
    Delete { key: Vec<u8, 32> },
    Commit { sequence: u64, state_root: PublicHash32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalRecord {
    pub sequence: u64,
    pub previous_hash: PublicHash32,
    pub entry_hash: PublicHash32,
    pub payload: WalPayload,
}

pub struct WriteAheadLog {
    records: Vec<WalRecord, { StorageLaw::WAL_MAX_ENTRIES }>,
    last_hash: PublicHash32,
    pub next_sequence: u64,
}

impl WriteAheadLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            last_hash: PublicHash32::default(),
            next_sequence: 0,
        }
    }

    fn hash_payload(payload: &WalPayload, sequence: u64, previous_hash: &PublicHash32) -> PublicHash32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&sequence.to_le_bytes());
        hasher.update(&previous_hash.0);
        match payload {
            WalPayload::Set { key, value } => {
                hasher.update(&[0u8]);
                hasher.update(key);
                hasher.update(value);
            }
            WalPayload::Delete { key } => {
                hasher.update(&[1u8]);
                hasher.update(key);
            }
            WalPayload::Commit { sequence: commit_seq, state_root } => {
                hasher.update(&[2u8]);
                hasher.update(&commit_seq.to_le_bytes());
                hasher.update(&state_root.0);
            }
        }
        let hash = hasher.finalize();
        let mut result = PublicHash32::default();
        result.0.copy_from_slice(&hash.as_bytes()[..32]);
        result
    }

    pub fn append(&mut self, payload: WalPayload) -> Result<u64, FailureContext> {
        if self.records.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000B,
                0x0001,
            ));
        }
        let seq = self.next_sequence;
        let entry_hash = Self::hash_payload(&payload, seq, &self.last_hash);
        let record = WalRecord {
            sequence: seq,
            previous_hash: self.last_hash,
            entry_hash,
            payload,
        };
        self.records.push(record).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0002)
        })?;
        self.last_hash = entry_hash;
        self.next_sequence = self.next_sequence.checked_add(1).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x000B, 0x0003)
        })?;
        Ok(seq)
    }

    pub fn verify_chain_integrity(&self) -> bool {
        let mut expected_prev = PublicHash32::default();
        let mut expected_seq: u64 = 0;
        for record in &self.records {
            if record.sequence != expected_seq {
                return false;
            }
            if record.previous_hash != expected_prev {
                return false;
            }
            let computed =
                Self::hash_payload(&record.payload, record.sequence, &record.previous_hash);
            if computed != record.entry_hash {
                return false;
            }
            expected_prev = computed;
            expected_seq = expected_seq.saturating_add(1);
        }
        true
    }

    pub fn last_commit_index(&self) -> Option<usize> {
        self.records
            .iter()
            .rposition(|r| matches!(&r.payload, WalPayload::Commit { .. }))
    }

    pub fn committed_records(&self) -> &[WalRecord] {
        match self.last_commit_index() {
            Some(idx) => &self.records[..=idx],
            None => &[],
        }
    }
}
