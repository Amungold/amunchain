use crate::capability::AuthorityCapability;
use amun_peer_identity::ConstitutionalPeerId;
use serde::{Deserialize, Serialize};

/// An institutional actor is a peer that possesses a set of
/// constitutional capabilities, making it an institution within
/// the civilisation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstitutionalActor {
    pub peer_id: ConstitutionalPeerId,
    pub capabilities: Vec<AuthorityCapability>,
}

/// A signed institutional witness binds an institutional actor
/// to a set of capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstitutionalWitness {
    pub actor: InstitutionalActor,
    pub signature: amun_constitutional_signing::ConstitutionalSignature,
}
