use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct EquivocationDetector { votes: BTreeMap<u64, BTreeMap<u64, Vec<[u8; 32]>>> }
impl EquivocationDetector {
    pub fn new() -> Self { Self { votes: BTreeMap::new() } }
    pub fn record_vote(&mut self, vid: u64, round: u64, vote_hash: [u8; 32]) -> Option<[u8; 32]> {
        let v_votes = self.votes.entry(vid).or_default();
        let r_votes = v_votes.entry(round).or_default();
        if r_votes.contains(&vote_hash) { return None; }
        if !r_votes.is_empty() { let existing = r_votes[0]; r_votes.push(vote_hash); return Some(existing); }
        r_votes.push(vote_hash); None
    }
}
impl Default for EquivocationDetector { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_equivocation() { let mut d = EquivocationDetector::new(); assert!(d.record_vote(1, 1, [1u8; 32]).is_none()); assert!(d.record_vote(1, 1, [2u8; 32]).is_some()); }
}
