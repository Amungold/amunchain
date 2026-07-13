use super::entry::WalEntry;
use crate::{Key256, SparseMerkleTree};
use std::fs::File;
use std::io::Read;

pub struct WalIterator {
    data: Vec<u8>,
    pos: usize,
    expected_seq: u64,
    last_entry_hash: [u8; 32],
    corrupted: bool,
    corruption_reason: Option<String>,
}

impl WalIterator {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap_or_else(|_| File::create(path).unwrap());
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        Self {
            data,
            pos: 0,
            expected_seq: 0,
            last_entry_hash: [0u8; 32],
            corrupted: false,
            corruption_reason: None,
        }
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }
    pub fn corruption_reason(&self) -> Option<&str> {
        self.corruption_reason.as_deref()
    }
}

impl Iterator for WalIterator {
    type Item = WalEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.corrupted {
            return None;
        }
        if self.pos + 4 > self.data.len() {
            return None;
        }
        let len = u32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]) as usize;
        self.pos += 4;
        if self.pos + len > self.data.len() {
            self.corrupted = true;
            self.corruption_reason = Some("frame truncated".to_string());
            return None;
        }
        let entry = WalEntry::decode(&self.data[self.pos..self.pos + len])?;
        self.pos += len;

        if entry.sequence != self.expected_seq {
            self.corrupted = true;
            self.corruption_reason = Some(format!(
                "sequence gap: expected {} got {}",
                self.expected_seq, entry.sequence
            ));
            return None;
        }

        if !entry.verify_chain(&self.last_entry_hash) {
            self.corrupted = true;
            self.corruption_reason = Some("hash chain broken".to_string());
            return None;
        }

        self.expected_seq += 1;
        self.last_entry_hash = entry.entry_hash;
        Some(entry)
    }
}

pub struct ReplayVerifier;
impl ReplayVerifier {
    pub fn verify_full_replay(wal_path: &str) -> Result<([u8; 32], u64), String> {
        let iter = WalIterator::new(wal_path);
        let mut tree = SparseMerkleTree::empty();
        let mut count: u64 = 0;
        let mut last_root = tree.root().0;
        let mut last_epoch: Option<u64> = None;
        let mut last_generation: Option<u64> = None;

        for entry in iter {
            if let Some(pe) = last_epoch {
                if entry.epoch < pe {
                    return Err(format!(
                        "epoch regression at {}: {} -> {}",
                        entry.sequence, pe, entry.epoch
                    ));
                }
                if entry.epoch == pe {
                    if let Some(pg) = last_generation {
                        if entry.generation <= pg {
                            return Err(format!(
                                "generation not monotonic at {}: {} -> {}",
                                entry.sequence, pg, entry.generation
                            ));
                        }
                    }
                } else {
                    if entry.generation != 0 {
                        return Err(format!(
                            "epoch transition must reset gen at {}: epoch {} gen {}",
                            entry.sequence, entry.epoch, entry.generation
                        ));
                    }
                }
            }
            last_epoch = Some(entry.epoch);
            last_generation = Some(entry.generation);

            if entry.op_type == 0x05 {
                if entry.key_hash.len() != 32 || entry.value_hash.len() != 32 {
                    return Err(format!("invalid key/value len at {}", entry.sequence));
                }
                let mut kb = [0u8; 32];
                let mut vb = [0u8; 32];
                kb.copy_from_slice(&entry.key_hash);
                vb.copy_from_slice(&entry.value_hash);
                tree = tree.insert(&Key256(kb), &vb, entry.version);
                let rr = tree.root().0;
                if rr != entry.state_root {
                    return Err(format!(
                        "replay divergence at {}: {:?} != {:?}",
                        entry.sequence,
                        &rr[..8],
                        &entry.state_root[..8]
                    ));
                }
                last_root = rr;
                count += 1;
            }
        }

        if count == 0 {
            return Err("empty WAL".to_string());
        }

        Ok((last_root, count))
    }
}
