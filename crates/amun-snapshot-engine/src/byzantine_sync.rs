use super::constitutional_identity::ConstitutionalIdentity;
use super::manifest::SnapshotManifest;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerManifest {
    pub peer_id: [u8; 32],
    pub manifest: SnapshotManifest,
    pub identity: ConstitutionalIdentity,
    pub signature: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncDecision {
    Accepted {
        manifest: SnapshotManifest,
        agreeing_peers: u64,
        total_peers: u64,
    },
    InsufficientPeers {
        received: u64,
        required: u64,
    },
    ConflictingCivilizations {
        groups: Vec<CivilizationGroup>,
    },
    IdentityMismatch {
        local: ConstitutionalIdentity,
        remote: ConstitutionalIdentity,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivilizationGroup {
    pub identity_hash: [u8; 32],
    pub manifest_hash: [u8; 32],
    pub manifest_root: [u8; 32],
    pub peer_count: u64,
}

/// Composite key for grouping: (identity_hash, manifest_hash)
/// Two peers agree only if BOTH their constitutional identity AND
/// their manifest match. Same manifest with different identity
/// means different civilizations.
type ConsensusKey = ([u8; 32], [u8; 32]);

pub struct ByzantineSyncEngine {
    pub local_identity: ConstitutionalIdentity,
    pub required_quorum: u64,
    pub received_manifests: Vec<PeerManifest>,
}

impl ByzantineSyncEngine {
    pub fn new(local_identity: ConstitutionalIdentity, required_quorum: u64) -> Self {
        Self {
            local_identity,
            required_quorum,
            received_manifests: Vec::new(),
        }
    }

    pub fn add_peer_manifest(&mut self, peer: PeerManifest) {
        if peer.identity.matches(&self.local_identity) {
            self.received_manifests.push(peer);
        }
    }

    pub fn decide(&self) -> SyncDecision {
        if (self.received_manifests.len() as u64) < self.required_quorum {
            return SyncDecision::InsufficientPeers {
                received: self.received_manifests.len() as u64,
                required: self.required_quorum,
            };
        }

        // Group by (identity_hash, manifest_hash) composite key
        let mut groups: HashMap<ConsensusKey, Vec<&PeerManifest>> = HashMap::new();
        for pm in &self.received_manifests {
            let key = (pm.identity.identity_hash, pm.manifest.manifest_hash);
            groups.entry(key).or_default().push(pm);
        }

        // Find largest group
        let mut largest: Option<(&ConsensusKey, &Vec<&PeerManifest>)> = None;
        for (key, peers) in &groups {
            if largest.is_none() || peers.len() > largest.unwrap().1.len() {
                largest = Some((key, peers));
            }
        }

        if let Some((_, peers)) = largest {
            let agreeing = peers.len() as u64;
            if agreeing >= self.required_quorum {
                return SyncDecision::Accepted {
                    manifest: peers[0].manifest.clone(),
                    agreeing_peers: agreeing,
                    total_peers: self.received_manifests.len() as u64,
                };
            }
        }

        // No quorum - report all groups
        let civilization_groups: Vec<CivilizationGroup> = groups
            .iter()
            .map(|((id_hash, m_hash), peers)| CivilizationGroup {
                identity_hash: *id_hash,
                manifest_hash: *m_hash,
                manifest_root: peers[0].manifest.state_root,
                peer_count: peers.len() as u64,
            })
            .collect();

        SyncDecision::ConflictingCivilizations {
            groups: civilization_groups,
        }
    }

    pub fn verify_peer_identity(&self, peer: &PeerManifest) -> bool {
        peer.identity.matches(&self.local_identity) && peer.identity.verify()
    }
}
