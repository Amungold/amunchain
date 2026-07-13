use serde::{Deserialize, Serialize};

/// Classification of constitutional evidence by its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceType {
    #[serde(rename = "experimental")]
    ExperimentalEvidence,
    #[serde(rename = "replay")]
    ReplayEvidence,
    #[serde(rename = "consensus")]
    ConsensusEvidence,
    #[serde(rename = "certificate")]
    CertificateEvidence,
    #[serde(rename = "formal_proof")]
    FormalProofEvidence,
    #[serde(rename = "audit")]
    AuditEvidence,
    #[serde(rename = "simulation")]
    SimulationEvidence,
}
