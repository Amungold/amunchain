use serde::{Deserialize, Serialize};

/// Whether an obligation is a primary claim (proved directly) or derived from others.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationKind {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "derived")]
    Derived,
}
