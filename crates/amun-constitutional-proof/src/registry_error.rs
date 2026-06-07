use thiserror::Error;
use crate::ObligationId;

/// Errors that can occur in the Obligation Registry.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistryError {
    #[error("duplicate obligation id: {0}")]
    DuplicateId(ObligationId),

    #[error("registry is frozen — cannot modify")]
    RegistryFrozen,

    #[error("obligation is frozen — cannot modify: {0}")]
    ObligationFrozen(ObligationId),

    #[error("circular dependency detected involving: {0}")]
    CircularDependency(ObligationId),

    #[error("missing dependency: {0} depends on {1} which is not registered")]
    MissingDependency(ObligationId, ObligationId),

    #[error("derived obligation does not terminate in primary: {0}")]
    DerivedNotTerminatingInPrimary(ObligationId),

    #[error("unknown namespace: {0}")]
    UnknownNamespace(String),

    #[error("invalid obligation id format: {0}")]
    InvalidObligationIdFormat(String),
}
