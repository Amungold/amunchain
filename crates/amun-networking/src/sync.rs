use amun_civilizational_relations::relation::CivilizationalRelation;
use amun_constitutional_quarantine::pipeline::QuarantinePipeline;
use amun_snapshot_engine::ConstitutionalIdentity;

/// Constitutional sync engine - orchestration only.
/// All constitutional semantics delegated to dedicated crates.
pub struct SyncEngine {
    pub local_identity: ConstitutionalIdentity,
    pub remote_relation: Option<CivilizationalRelation>,
    pub quarantine_pipeline: Option<QuarantinePipeline>,
    pub state: SyncState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncState {
    Idle,
    ClassifyingRelation,
    QuarantinePipelineActive,
    ManifestRequesting,
    ChunkDownloading { current: u64, total: u64 },
    Verifying,
    Complete { final_root: [u8; 32] },
    Rejected { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncResult {
    Complete { final_root: [u8; 32] },
    Rejected { reason: String },
    Quarantined { pipeline: QuarantinePipeline },
    InProgress { state: SyncState },
}

impl SyncEngine {
    pub fn new(local_identity: ConstitutionalIdentity) -> Self {
        Self {
            local_identity,
            remote_relation: None,
            quarantine_pipeline: None,
            state: SyncState::Idle,
        }
    }

    /// Initiate sync by classifying the civilizational relation first.
    /// Networking does NOT interpret constitutional semantics.
    pub fn initiate_sync(&mut self, remote: ConstitutionalIdentity) -> SyncResult {
        self.state = SyncState::ClassifyingRelation;

        let relation = CivilizationalRelation::classify(&self.local_identity, &remote);
        self.remote_relation = Some(relation.clone());

        if relation.can_interact() {
            self.state = SyncState::ManifestRequesting;
            return SyncResult::InProgress {
                state: self.state.clone(),
            };
        }

        if relation.requires_quarantine() {
            let pipeline = QuarantinePipeline::new(relation);
            self.quarantine_pipeline = Some(pipeline.clone());
            self.state = SyncState::QuarantinePipelineActive;
            return SyncResult::Quarantined { pipeline };
        }

        self.state = SyncState::Rejected {
            reason: format!("Cannot interact: {:?}", relation.relation_type()),
        };
        SyncResult::Rejected {
            reason: "Civilizational relation prohibits interaction".to_string(),
        }
    }
}
