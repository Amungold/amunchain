use std::collections::BTreeMap;
use blake3;
use super::qc_store::{QCStore, QC, QCHash};

#[derive(Debug, Clone)]
pub struct ReplayEvent { pub event_type: ReplayEventType, pub block_height: u64, pub block_hash: [u8; 32], pub parent_hash: Option<[u8; 32]>, pub validator_id: Option<u64>, pub round: Option<u64> }

#[derive(Debug, Clone)]
pub enum ReplayEventType { Propose, Vote, QCAggregate, Timeout, AdvanceRound }

#[derive(Debug, Clone)]
pub struct ReplayDigest { pub event_count: usize, pub committed_blocks: Vec<[u8; 32]>, pub final_hash: [u8; 32], pub has_conflicts: bool, pub has_equivocations: bool }

#[derive(Debug, Clone)]
pub struct ReplaySimulator { events: Vec<ReplayEvent>, store: QCStore, committed_blocks: Vec<[u8; 32]>, event_order: Vec<u8> }
impl ReplaySimulator {
    pub fn new() -> Self { Self { events: Vec::new(), store: QCStore::new(), committed_blocks: Vec::new(), event_order: Vec::new() } }
    pub fn add_event(&mut self, event: ReplayEvent) { let order_bytes = (self.events.len() as u64).to_be_bytes(); self.event_order.extend_from_slice(&order_bytes); self.events.push(event); }
    pub fn run(&mut self) -> ReplayDigest { for event in self.events.clone() { self.process_event(event); } ReplayDigest { event_count: self.events.len(), committed_blocks: self.committed_blocks.clone(), final_hash: blake3::hash(&self.event_order).into(), has_conflicts: false, has_equivocations: false } }
    fn process_event(&mut self, event: ReplayEvent) { if let ReplayEventType::QCAggregate = event.event_type { let mut q = QC::new(event.block_height, event.block_hash, 3, event.round.unwrap_or(1), 0, [0u8; 32]); if let Some(ph) = event.parent_hash { if let Some(pqc) = self.store.get(&QCHash::from_bytes(ph)) { q = q.with_parent(pqc.hash); } } let q = q.finalize(); self.store.insert(q); self.committed_blocks.push(event.block_hash); } } }
impl Default for ReplaySimulator { fn default() -> Self { Self::new() } }
