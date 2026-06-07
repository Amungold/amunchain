#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::must_use_candidate)]
use blake3::Hasher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRecord {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub commit_hash: [u8; 32],
    pub replay_certificate: [u8; 32],
    pub previous_audit_record: [u8; 32],
    pub timestamp: u64,
}

impl AuditRecord {
    pub fn record_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_AUDIT_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.commit_hash);
        hasher.update(&self.replay_certificate);
        hasher.update(&self.previous_audit_record);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuditTrail {
    pub records: Vec<AuditRecord>,
}

impl AuditTrail {
    pub fn new() -> Self { Self::default() }

    pub fn record(
        &mut self,
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        commit_hash: [u8; 32],
        replay_certificate: [u8; 32],
        timestamp: u64,
    ) {
        let previous = self.records.last().map(|r| r.record_hash()).unwrap_or([0u8; 32]);
        self.records.push(AuditRecord {
            height, block_hash, state_root, commit_hash, replay_certificate,
            previous_audit_record: previous, timestamp,
        });
    }

    pub fn verify(&self) -> bool {
        for i in 1..self.records.len() {
            if self.records[i].previous_audit_record != self.records[i - 1].record_hash() {
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n38_single_audit_record() {
        let mut trail = AuditTrail::new();
        trail.record(1, [1u8; 32], [10u8; 32], [11u8; 32], [12u8; 32], 1000);
        assert_eq!(trail.len(), 1);
        assert!(trail.verify());
    }

    #[test]
    fn n38_audit_trail_chain() {
        let mut trail = AuditTrail::new();
        trail.record(1, [1u8; 32], [10u8; 32], [11u8; 32], [12u8; 32], 1000);
        trail.record(2, [2u8; 32], [20u8; 32], [21u8; 32], [22u8; 32], 2000);
        trail.record(3, [3u8; 32], [30u8; 32], [31u8; 32], [32u8; 32], 3000);
        assert!(trail.verify());
        assert_eq!(trail.len(), 3);
    }

    #[test]
    fn n38_broken_trail_detected() {
        let mut trail = AuditTrail::new();
        trail.record(1, [1u8; 32], [10u8; 32], [11u8; 32], [12u8; 32], 1000);
        trail.record(2, [2u8; 32], [20u8; 32], [21u8; 32], [22u8; 32], 2000);
        trail.record(3, [3u8; 32], [30u8; 32], [31u8; 32], [32u8; 32], 3000);
        trail.records[1].state_root = [0xFF; 32];
        assert!(!trail.verify());
    }

    #[test]
    fn n38_record_hash_deterministic() {
        let mut trail = AuditTrail::new();
        trail.record(1, [1u8; 32], [10u8; 32], [11u8; 32], [12u8; 32], 1000);
        let h1 = trail.records[0].record_hash();
        let h2 = trail.records[0].record_hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn n38_different_records_different_hash() {
        let mut trail = AuditTrail::new();
        trail.record(1, [1u8; 32], [10u8; 32], [11u8; 32], [12u8; 32], 1000);
        trail.record(2, [2u8; 32], [20u8; 32], [21u8; 32], [22u8; 32], 2000);
        assert_ne!(trail.records[0].record_hash(), trail.records[1].record_hash());
    }
}
