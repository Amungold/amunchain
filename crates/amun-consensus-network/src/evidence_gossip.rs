// ============================================================================
// N111.2 — Evidence Gossip
// ============================================================================
// Propagates evidence records across the network so that every node
// possesses the evidence referenced by a SlashingCertificate.
//
// Without this layer, a node that did not witness the original
// misbehaviour will reject a certificate that references unknown
// evidence IDs (the critical limitation documented in v0.4.0-N110).
//
// The gossip layer:
//   1. Receives an EvidenceAnnouncement from the network
//   2. Validates and stores the announcement
//   3. Tracks broadcast state to avoid infinite loops
// ============================================================================

use crate::evidence_store::EvidenceType;
use std::collections::HashMap;

/// N111.1: Lightweight announcement of new evidence.
/// Carries the evidence_id and metadata so peers can request
/// the full EvidenceRecord if they need it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct EvidenceAnnouncement {
    pub evidence_id: [u8; 32],
    pub validator_id: [u8; 32],
    pub evidence_type: EvidenceType,
    pub height: u64,
    pub timestamp: u64,
}

/// N111.2: Manages received evidence announcements.
pub struct EvidenceGossip {
    /// Announcements indexed by evidence_id
    pub announcements: HashMap<[u8; 32], EvidenceAnnouncement>,
    /// Set of evidence_ids already broadcast to prevent loops
    pub broadcasted: HashMap<[u8; 32], bool>,
}

impl EvidenceGossip {
    pub fn new() -> Self {
        Self {
            announcements: HashMap::new(),
            broadcasted: HashMap::new(),
        }
    }

    /// Receive an evidence announcement from the network.
    /// Returns true if the announcement is new.
    pub fn receive_announcement(&mut self, announcement: EvidenceAnnouncement) -> bool {
        let id = announcement.evidence_id;
        if self.announcements.contains_key(&id) {
            return false; // already have it
        }
        self.announcements.insert(id, announcement);
        true
    }

    /// Check if we should broadcast this evidence_id.
    pub fn should_broadcast(&self, evidence_id: &[u8; 32]) -> bool {
        !self.broadcasted.contains_key(evidence_id)
    }

    /// Mark an evidence_id as broadcasted.
    pub fn mark_broadcasted(&mut self, evidence_id: &[u8; 32]) {
        self.broadcasted.insert(*evidence_id, true);
    }

    /// Check if we have an announcement for a given evidence_id.
    pub fn has_evidence(&self, evidence_id: &[u8; 32]) -> bool {
        self.announcements.contains_key(evidence_id)
    }

    pub fn len(&self) -> usize {
        self.announcements.len()
    }

    /// N111.4: Verify an incoming evidence announcement before accepting it.
    pub fn verify_announcement(
        announcement: &EvidenceAnnouncement,
        current_height: u64,
    ) -> Result<(), String> {
        // Height must not be in the far future
        if announcement.height > current_height + 100 {
            return Err(format!(
                "N111.4: evidence height {} is too far in the future (current={})",
                announcement.height, current_height
            ));
        }
        // evidence_id must not be all zeros
        if announcement.evidence_id == [0u8; 32] {
            return Err("N111.4: evidence_id is zero".into());
        }
        // validator_id must not be all zeros
        if announcement.validator_id == [0u8; 32] {
            return Err("N111.4: validator_id is zero".into());
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.announcements.is_empty()
    }
}

impl Default for EvidenceGossip {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n111_2_receive_new_announcement() {
        let mut gossip = EvidenceGossip::new();
        let ann = EvidenceAnnouncement {
            evidence_id: [0xA1; 32],
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 10,
            timestamp: 1000,
        };
        assert!(gossip.receive_announcement(ann));
        assert_eq!(gossip.len(), 1);
    }

    #[test]
    fn n111_2_duplicate_ignored() {
        let mut gossip = EvidenceGossip::new();
        let ann = EvidenceAnnouncement {
            evidence_id: [0xA1; 32],
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 10,
            timestamp: 1000,
        };
        assert!(gossip.receive_announcement(ann.clone()));
        assert!(!gossip.receive_announcement(ann));
        assert_eq!(gossip.len(), 1);
    }

    #[test]
    fn n111_2_broadcast_tracking() {
        let mut gossip = EvidenceGossip::new();
        let id = [0xB1; 32];
        assert!(gossip.should_broadcast(&id));
        gossip.mark_broadcasted(&id);
        assert!(!gossip.should_broadcast(&id));
    }

    #[test]
    fn n111_2_has_evidence() {
        let mut gossip = EvidenceGossip::new();
        let id = [0xC1; 32];
        assert!(!gossip.has_evidence(&id));
        gossip.receive_announcement(EvidenceAnnouncement {
            evidence_id: id,
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::StateRootMismatch,
            height: 5,
            timestamp: 2000,
        });
        assert!(gossip.has_evidence(&id));
    }

    #[test]
    fn n111_4_valid_announcement_passes() {
        let ann = EvidenceAnnouncement {
            evidence_id: [0xA1; 32],
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 10,
            timestamp: 1000,
        };
        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_ok());
    }

    #[test]
    fn n111_4_future_height_rejected() {
        let ann = EvidenceAnnouncement {
            evidence_id: [0xA1; 32],
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 200,
            timestamp: 1000,
        };
        let err = EvidenceGossip::verify_announcement(&ann, 50).unwrap_err();
        assert!(err.contains("too far in the future"));
    }

    #[test]
    fn n111_4_zero_evidence_id_rejected() {
        let ann = EvidenceAnnouncement {
            evidence_id: [0u8; 32],
            validator_id: [0x42; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 10,
            timestamp: 1000,
        };
        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
    }

    #[test]
    fn n111_4_zero_validator_id_rejected() {
        let ann = EvidenceAnnouncement {
            evidence_id: [0xA1; 32],
            validator_id: [0u8; 32],
            evidence_type: EvidenceType::DoubleVote,
            height: 10,
            timestamp: 1000,
        };
        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
    }
}
