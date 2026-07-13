use crate::RegistryError;
use serde::{Deserialize, Serialize};

/// The constitutional domains defined in Article I of the N47 constitution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ObligationNamespace {
    #[serde(rename = "safety")]
    Safety,
    #[serde(rename = "replay")]
    Replay,
    #[serde(rename = "evidence")]
    Evidence,
    #[serde(rename = "finality")]
    Finality,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "performance")]
    Performance,
}

impl std::fmt::Display for ObligationNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Safety => "SAFETY",
            Self::Replay => "REPLAY",
            Self::Evidence => "EVIDENCE",
            Self::Finality => "FINALITY",
            Self::Cluster => "CLUSTER",
            Self::Fault => "FAULT",
            Self::Recovery => "RECOVERY",
            Self::Performance => "PERFORMANCE",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for ObligationNamespace {
    type Error = RegistryError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_str() {
            "SAFETY" => Ok(Self::Safety),
            "REPLAY" => Ok(Self::Replay),
            "EVIDENCE" => Ok(Self::Evidence),
            "FINALITY" => Ok(Self::Finality),
            "CLUSTER" => Ok(Self::Cluster),
            "FAULT" => Ok(Self::Fault),
            "RECOVERY" => Ok(Self::Recovery),
            "PERFORMANCE" => Ok(Self::Performance),
            other => Err(RegistryError::UnknownNamespace(other.to_string())),
        }
    }
}
