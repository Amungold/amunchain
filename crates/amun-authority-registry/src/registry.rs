use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::authority::ConstitutionalAuthority;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorityRegistry {
    authorities: BTreeMap<u64, ConstitutionalAuthority>,
    active_version: u64,
    pub transition: Option<AuthorityTransition>,
}

/// Describes a scheduled transition from one authority version to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityTransition {
    pub from_version: u64,
    pub to_version: u64,
    pub activation_height: u64,
    pub grace_period_blocks: u64,
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
        self.authorities.get(&version).is_some_and(|a| a.revoked)
    }

    /// Activate a new authority, making it the active one.
    pub fn activate(&mut self, authority: ConstitutionalAuthority) {
        let version = authority.authority_version;
        self.authorities.insert(version, authority);
        self.active_version = version;
    }

    /// Retire an authority so it can no longer issue new certificates.
    pub fn retire(&mut self, authority_version: u64, height: u64) {
        if let Some(auth) = self.authorities.get_mut(&authority_version) {
            auth.revoked = true;
            auth.revoked_at_height = Some(height);
        }
    }

    /// Schedule a transition to a new authority version.
    pub fn schedule_transition(&mut self, transition: AuthorityTransition) {
        self.transition = Some(transition);
    }

    /// Return the authority (or authorities) valid at a given block height.
    pub fn valid_authorities_at(&self, height: u64) -> Vec<&ConstitutionalAuthority> {
        let mut result = Vec::new();
        if let Some(ref t) = self.transition {
            if height < t.activation_height {
                // Pre-activation: only the old authority is valid
                if let Some(a) = self.by_version(t.from_version) {
                    result.push(a);
                }
            } else if height < t.activation_height + t.grace_period_blocks {
                // Grace period: both authorities are valid
                if let Some(a) = self.by_version(t.from_version) {
                    result.push(a);
                }
                if let Some(a) = self.by_version(t.to_version) {
                    result.push(a);
                }
            } else {
                // Post-grace: only the new authority is valid
                if let Some(a) = self.by_version(t.to_version) {
                    result.push(a);
                }
            }
        } else {
            // No transition scheduled: return the active authority
            if let Some(a) = self.active() {
                result.push(a);
            }
        }
        result
    }

    /// Check if an authority can issue new certificates at a given height.
    pub fn can_issue_at(&self, authority_version: u64, height: u64) -> bool {
        if self.is_revoked(authority_version) {
            return false;
        }
        if let Some(ref t) = self.transition {
            // After activation, the old authority cannot issue new certs
            if authority_version == t.from_version && height >= t.activation_height {
                return false;
            }
        }
        // The active authority can always issue (if not revoked)
        if let Some(active) = self.active() {
            if active.authority_version == authority_version {
                return true;
            }
        }
        false
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
        reg.retire(1, 1000);
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
        reg.retire(1, 1000);
        assert!(reg.is_revoked(1));
        assert!(reg.by_version(1).is_some());
    }

    #[test]
    fn n107_6_schedule_transition() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 0);
        reg.register(a1);
        reg.register(a2);
        let t = AuthorityTransition {
            from_version: 1,
            to_version: 2,
            activation_height: 100,
            grace_period_blocks: 50,
        };
        reg.schedule_transition(t);
        assert!(reg.transition.is_some());
    }

    #[test]
    fn n107_6_pre_activation() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 0);
        reg.register(a1);
        reg.register(a2);
        reg.schedule_transition(AuthorityTransition {
            from_version: 1, to_version: 2,
            activation_height: 100, grace_period_blocks: 50,
        });
        let valid = reg.valid_authorities_at(50);
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].authority_version, 1);
    }

    #[test]
    fn n107_6_dual_validation_window() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 0);
        reg.register(a1);
        reg.register(a2);
        reg.schedule_transition(AuthorityTransition {
            from_version: 1, to_version: 2,
            activation_height: 100, grace_period_blocks: 50,
        });
        let valid = reg.valid_authorities_at(120);
        assert_eq!(valid.len(), 2);
    }

    #[test]
    fn n107_6_post_grace_period() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 0);
        reg.register(a1);
        reg.register(a2);
        reg.schedule_transition(AuthorityTransition {
            from_version: 1, to_version: 2,
            activation_height: 100, grace_period_blocks: 50,
        });
        let valid = reg.valid_authorities_at(200);
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].authority_version, 2);
    }

    #[test]
    fn n107_6_old_authority_cannot_issue_after_activation() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 2, 0);
        reg.register(a1);
        reg.register(a2);
        reg.schedule_transition(AuthorityTransition {
            from_version: 1, to_version: 2,
            activation_height: 100, grace_period_blocks: 50,
        });
        assert!(!reg.can_issue_at(1, 150));
        assert!(reg.can_issue_at(2, 150));
    }

    #[test]
    fn n107_6_historical_certificates_survive() {
        let mut reg = AuthorityRegistry::new();
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        reg.register(a1);
        reg.retire(1, 1000);
        // Retired authority can still be looked up for verification
        assert!(reg.by_version(1).is_some());
    }
}
