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

    /// Activate a new authority, making it the active one.
    pub fn activate(&mut self, authority: ConstitutionalAuthority) {
        let version = authority.authority_version;
        self.authorities.insert(version, authority);
        self.active_version = version;
    }

    /// Retire an authority so it can no longer issue new certificates.
    pub fn retire(&mut self, authority_version: u64) {
        if let Some(auth) = self.authorities.get_mut(&authority_version) {
            auth.revoked = true;
        }
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

    #[test]
    fn n107_5_activate_new_authority() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 100);
        reg.activate(a2);
        assert_eq!(reg.active().unwrap().authority_version, 2);
        assert!(reg.by_version(1).is_some());
        assert!(reg.by_version(2).is_some());
    }

    #[test]
    fn n107_5_retire_authority() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1);
        reg.retire(1);
        assert!(reg.is_revoked(1));
        assert!(reg.by_version(1).is_some());
    }

    #[test]
    fn n107_5_multi_version_registry() {
        let mut reg = AuthorityRegistry::new();
        reg.register(ConstitutionalAuthority::new([1u8; 32], 1, 0));
        reg.register(ConstitutionalAuthority::new([2u8; 32], 2, 100));
        reg.register(ConstitutionalAuthority::new([3u8; 32], 3, 200));
        assert_eq!(reg.active().unwrap().authority_version, 3);
        assert!(reg.by_version(1).is_some());
        assert!(reg.by_version(2).is_some());
        assert!(reg.by_version(3).is_some());
    }

    #[test]
    fn n107_5_cross_epoch_verification() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 100);
        reg.register(a1.clone());
        reg.register(a2.clone());
        let cert_v1_key = reg.by_version(1).unwrap().authority_public_key;
        let cert_v2_key = reg.by_version(2).unwrap().authority_public_key;
        assert_eq!(cert_v1_key, a1.authority_public_key);
        assert_eq!(cert_v2_key, a2.authority_public_key);
    }

    #[test]
    fn n107_5_sunset_rejection() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1);
        reg.retire(1);
        assert!(reg.is_revoked(1));
        assert!(reg.by_version(1).is_some());
    }
}
