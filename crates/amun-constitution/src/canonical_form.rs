// Constitutional Canonical Form (CCF) — the protocol specification.

use crate::capacity::ProtocolCapacities;
use amun_kernel_types::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstitutionDomain;

#[derive(Clone, Debug)]
pub struct ConstitutionalCanonicalForm {
    pub ccf_version: u16,
    pub protocol_version: u16,
    pub rfc_ids: &'static [u16],
    pub capacities: ProtocolCapacities,
    pub encoding_version: u8,
    pub domain_key_version: u8,
    pub transition_algebra_version: u16,
    pub governance_rules_version: u16,
    pub economics_version: u16,
    pub parent_constitution_hash: Option<Hash<ConstitutionDomain>>,
}

impl ConstitutionalCanonicalForm {
    pub fn compute_hash(&self) -> Hash<ConstitutionDomain> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.ccf_version.to_le_bytes());
        hasher.update(&self.protocol_version.to_le_bytes());
        let rfc_count = self.rfc_ids.len() as u32;
        hasher.update(&rfc_count.to_le_bytes());
        for &rfc_id in self.rfc_ids {
            hasher.update(&rfc_id.to_le_bytes());
        }
        hasher.update(&[self.encoding_version, self.domain_key_version]);
        hasher.update(&self.transition_algebra_version.to_le_bytes());
        hasher.update(&self.governance_rules_version.to_le_bytes());
        hasher.update(&self.economics_version.to_le_bytes());
        Hash::new(hasher.finalize().into())
    }
}

pub const CURRENT_CCF: ConstitutionalCanonicalForm = ConstitutionalCanonicalForm {
    ccf_version: 1,
    protocol_version: 1,
    rfc_ids: &[1, 2, 3],
    capacities: ProtocolCapacities::constitutional(),
    encoding_version: 1,
    domain_key_version: 1,
    transition_algebra_version: 1,
    governance_rules_version: 1,
    economics_version: 1,
    parent_constitution_hash: None,
};
