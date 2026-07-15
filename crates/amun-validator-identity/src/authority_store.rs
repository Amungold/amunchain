use amun_validator_api::types::id::PublicKey;

/// Trust anchor representing a constitutional authority.
#[derive(Debug, Clone)]
pub struct TrustAnchor {
    pub authority_id: [u8; 32],
    pub public_key: PublicKey,
    pub version: u64,
    pub epoch: u64,
}

/// Single source of truth for trusted authorities.
/// Used by AuthorityVerifier to validate certificates.
pub struct AuthorityStore {
    anchors: Vec<TrustAnchor>,
}

impl AuthorityStore {
    pub fn new(anchors: Vec<TrustAnchor>) -> Self {
        AuthorityStore { anchors }
    }

    pub fn from_single(authority_id: [u8; 32], public_key: PublicKey, version: u64) -> Self {
        AuthorityStore {
            anchors: vec![TrustAnchor {
                authority_id,
                public_key,
                version,
                epoch: 0,
            }],
        }
    }

    pub fn find_anchor(&self, authority_id: &[u8; 32]) -> Option<&TrustAnchor> {
        self.anchors
            .iter()
            .find(|a| a.authority_id == *authority_id)
    }

    pub fn anchors(&self) -> &[TrustAnchor] {
        &self.anchors
    }

    pub fn is_trusted(&self, authority_id: &[u8; 32]) -> bool {
        self.find_anchor(authority_id).is_some()
    }
}
