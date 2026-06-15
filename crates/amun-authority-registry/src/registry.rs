use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::authority::ConstitutionalAuthority;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorityRegistry {
    authorities: BTreeMap<u64, ConstitutionalAuthority>,
    active_version: u64,
}

impl AuthorityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(&self) -> Option<&ConstitutionalAuthority> {
        self.authorities.get(&self.active_version)
    }

    pub fn by_version(&self, version: u64) -> Option<&ConstitutionalAuthority> {
        self.authorities.get(&version)
    }

    pub fn register(&mut self, authority: ConstitutionalAuthority) {
        let version = authority.authority_version;
        self.authorities.insert(version, authority);
        if version > self.active_version {
            self.active_version = version;
        }
    }

    pub fn revoke(&mut self, version: u64) {
        if let Some(auth) = self.authorities.get_mut(&version) {
            auth.revoked = true;
        }
    }


    /// Create a registry pre-populated with a single genesis authority.
    pub fn from_genesis(authority: ConstitutionalAuthority) -> Self {
        let mut registry = Self::new();
        registry.register(authority);
        registry
    }

    pub fn is_revoked(&self, version: u64) -> bool {
        self.authorities.get(&version).map_or(false, |a| a.revoked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n107_2_register_and_activate() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1.clone());
        assert_eq!(reg.active().unwrap().authority_version, 1);
    }

    #[test]
    fn n107_2_revoke() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1);
        reg.revoke(1);
        assert!(reg.is_revoked(1));
    }

    #[test]
    fn n107_2_active_tracks_latest() {
        let mut reg = AuthorityRegistry::new();
        reg.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));
        reg.register(ConstitutionalAuthority::new([2u8; 32], 2, 100));
        assert_eq!(reg.active().unwrap().authority_version, 2);
    }

    #[test]
    fn n107_3_registry_bootstrap() {
        let authority = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let registry = AuthorityRegistry::from_genesis(authority);
        assert_eq!(registry.active().unwrap().authority_version, 1);
    }
}
