const MAX_TRACE: usize = 5000;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEntry { pub from_root: [u8; 32], pub to_root: [u8; 32], pub via_message_hash: [u8; 32] }
pub struct StateTrace { entries: Vec<TraceEntry> }
impl StateTrace {
    pub fn new() -> Self { Self { entries: Vec::with_capacity(MAX_TRACE) } }
    pub fn append(&mut self, from: [u8; 32], to: [u8; 32], hash: [u8; 32]) -> Result<(), &'static str> {
        if self.entries.len() >= MAX_TRACE { return Err("trace full"); }
        self.entries.push(TraceEntry { from_root: from, to_root: to, via_message_hash: hash }); Ok(())
    }
    pub fn find_transition(&self, from: [u8; 32], hash: [u8; 32]) -> Option<&TraceEntry> {
        self.entries.iter().rev().find(|e| e.from_root == from && e.via_message_hash == hash)
    }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn truncate(&mut self, len: usize) { self.entries.truncate(len); }
}
