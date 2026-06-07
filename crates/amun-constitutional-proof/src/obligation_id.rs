use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::{ObligationNamespace, RegistryError};

/// A unique constitutional identifier for a proof obligation.
///
/// Internally structured (namespace + sequence) for fast lookup.
/// Externally rendered and serialized as a canonical string like "SAFETY-001".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObligationId {
    pub namespace: ObligationNamespace,
    pub sequence: u32,
}

impl ObligationId {
    pub fn new(namespace: ObligationNamespace, sequence: u32) -> Self {
        Self { namespace, sequence }
    }

    pub fn namespace(&self) -> ObligationNamespace {
        self.namespace
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }
}

impl std::fmt::Display for ObligationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:03}", self.namespace, self.sequence)
    }
}

impl Serialize for ObligationId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObligationId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for ObligationId {
    type Err = RegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(RegistryError::InvalidObligationIdFormat(s.to_string()));
        }
        let namespace: ObligationNamespace = parts[0].try_into()?;
        let sequence: u32 = parts[1]
            .parse()
            .map_err(|_| RegistryError::InvalidObligationIdFormat(s.to_string()))?;
        Ok(Self { namespace, sequence })
    }
}
