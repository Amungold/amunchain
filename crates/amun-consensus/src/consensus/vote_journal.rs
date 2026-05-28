use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord { pub validator_id: u64, pub round: u64, pub vote_hash: [u8; 32], pub block_height: u64 }

#[derive(Debug, Clone, Default)]
pub struct VoteJournal { votes: BTreeMap<u64, BTreeMap<u64, Vec<[u8; 32]>>>, path: PathBuf }
impl VoteJournal {
    pub fn new(data_dir: &str) -> Self { let path = PathBuf::from(data_dir).join("votes.journal"); let mut j = Self { votes: BTreeMap::new(), path }; j.load(); j }
    pub fn record_vote(&mut self, vid: u64, round: u64, vote_hash: [u8; 32], _height: u64) -> Option<[u8; 32]> {
        let v = self.votes.entry(vid).or_default(); let r = v.entry(round).or_default();
        if r.contains(&vote_hash) { return None; }
        if !r.is_empty() { let existing = r[0]; r.push(vote_hash); let _ = self.persist(); return Some(existing); }
        r.push(vote_hash); let _ = self.persist(); None
    }
    fn persist(&self) -> std::io::Result<()> { let bytes = bincode::serialize(&self.votes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; let mut f = OpenOptions::new().create(true).write(true).truncate(true).open(&self.path)?; f.write_all(&bytes)?; f.sync_all()?; Ok(()) }
    fn load(&mut self) { if let Ok(mut f) = File::open(&self.path) { let mut bytes = Vec::new(); if f.read_to_end(&mut bytes).is_ok() { if let Ok(votes) = bincode::deserialize(&bytes) { self.votes = votes; } } } }
}
