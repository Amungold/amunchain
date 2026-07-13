use serde::{Deserialize, Serialize};

/// Describes how experimental evidence can be reproduced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reproducibility {
    pub command: String,
    pub environment_hash: String,
    pub expected_output_hash: String,
}

impl Reproducibility {
    pub fn new(command: String, environment_hash: String, expected_output_hash: String) -> Self {
        Self {
            command,
            environment_hash,
            expected_output_hash,
        }
    }
}
