//! Constitutional Governance Primitives
//!
//! These are the ONLY governance objects permitted in AmunChain.
//! All governance operations must use CanonicalEncode for determinism.

use crate::canonical::{CanonicalEncode, CanonicalEncoder};
use crate::hashing::domain_tags;

// ─── Signature types ──────────────────────────────────────

/// A cryptographic signature with algorithm awareness.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signature {
    Ed25519([u8; 64]),
    Threshold {
        signer_ids: Vec<[u8; 32]>,
        signatures: Vec<[u8; 64]>,
    },
    Emergency([u8; 64]),
}

impl CanonicalEncode for Signature {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        match self {
            Signature::Ed25519(sig) => {
                (0u8).encode_canonical(out);
                out.extend_from_slice(sig);
            }
            Signature::Threshold {
                signer_ids,
                signatures,
            } => {
                (1u8).encode_canonical(out);
                (signer_ids.len() as u64).encode_canonical(out);
                for id in signer_ids {
                    out.extend_from_slice(id);
                }
                for sig in signatures {
                    out.extend_from_slice(sig);
                }
            }
            Signature::Emergency(sig) => {
                (2u8).encode_canonical(out);
                out.extend_from_slice(sig);
            }
        }
    }
}

// ─── Authority ─────────────────────────────────────────────

/// A governance authority identified by its public key.
/// The `id` is derived canonically: `id = H(public_key)` — role does NOT affect identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Authority {
    pub id: [u8; 32],
    pub public_key: [u8; 32],
    pub role: AuthorityRole,
    pub valid_from: u64,
    pub valid_until: u64,
}

impl Authority {
    /// Create an authority with a deterministic, key-derived ID.
    /// Role does NOT affect the identity — same key = same authority.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        public_key: [u8; 32],
        role: AuthorityRole,
        valid_from: u64,
        valid_until: u64,
    ) -> Self {
        let id = CanonicalEncoder::hash_value(&public_key, domain_tags::AUTHORITY);
        Self {
            id,
            public_key,
            role,
            valid_from,
            valid_until,
        }
    }

    /// Check whether this authority is currently valid.
    pub fn is_valid_at(&self, timestamp: u64) -> bool {
        timestamp >= self.valid_from && timestamp <= self.valid_until
    }
}

impl CanonicalEncode for Authority {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.id);
        out.extend_from_slice(&self.public_key);
        self.role.encode_canonical(out);
        self.valid_from.encode_canonical(out);
        self.valid_until.encode_canonical(out);
    }
}

/// The role of an authority in governance.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthorityRole {
    Signer,
    Reviewer,
    Approver,
    EmergencyAuthority,
}

impl CanonicalEncode for AuthorityRole {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        let tag: u8 = match self {
            AuthorityRole::Signer => 0,
            AuthorityRole::Reviewer => 1,
            AuthorityRole::Approver => 2,
            AuthorityRole::EmergencyAuthority => 3,
        };
        tag.encode_canonical(out);
    }
}

// ─── FreezeBoundary ────────────────────────────────────────

/// A freeze boundary — an immutable protocol commitment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreezeBoundary {
    pub name: String,
    pub version: u64,
    pub domain: FreezeDomain,
    /// Hash of the canonical source that defines this boundary.
    pub canonical_hash: [u8; 32],
    /// Reproducible artifact: what was hashed to produce canonical_hash.
    pub artifact_definition: String,
}

impl CanonicalEncode for FreezeBoundary {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.name.encode_canonical(out);
        self.version.encode_canonical(out);
        self.domain.encode_canonical(out);
        out.extend_from_slice(&self.canonical_hash);
        self.artifact_definition.encode_canonical(out);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FreezeDomain {
    WireFormat,
    DiskFormat,
    Codec,
    SnapshotSchema,
    GovernanceSchema,
}

impl CanonicalEncode for FreezeDomain {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        let tag: u8 = match self {
            FreezeDomain::WireFormat => 0,
            FreezeDomain::DiskFormat => 1,
            FreezeDomain::Codec => 2,
            FreezeDomain::SnapshotSchema => 3,
            FreezeDomain::GovernanceSchema => 4,
        };
        tag.encode_canonical(out);
    }
}

// ─── Authority Registry ────────────────────────────────────

/// A registry of governance authorities.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorityRegistry {
    pub authorities: Vec<Authority>,
    pub epoch: u64,
}

impl AuthorityRegistry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(epoch: u64) -> Self {
        Self {
            authorities: Vec::new(),
            epoch,
        }
    }

    pub fn add(&mut self, authority: Authority) {
        self.authorities.push(authority);
    }

    /// Compute the canonical root of this registry.
    pub fn registry_root(&self) -> [u8; 32] {
        let mut sorted = self.authorities.clone();
        sorted.sort_by_key(|a| a.id);
        CanonicalEncoder::hash_value(&sorted, domain_tags::AUTHORITY)
    }

    /// Look up an authority by ID.
    pub fn find(&self, id: &[u8; 32], at_timestamp: u64) -> Option<&Authority> {
        self.authorities
            .iter()
            .find(|a| &a.id == id && a.is_valid_at(at_timestamp))
    }
}

impl CanonicalEncode for AuthorityRegistry {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.epoch.encode_canonical(out);
        let mut sorted = self.authorities.clone();
        sorted.sort_by_key(|a| a.id);
        (sorted.len() as u64).encode_canonical(out);
        for authority in &sorted {
            authority.encode_canonical(out);
        }
    }
}

// ─── Attestation ───────────────────────────────────────────

/// A signed governance attestation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attestation {
    pub snapshot: Snapshot,
    pub signer_id: [u8; 32],
    pub signature: Signature,
}

impl Attestation {
    /// Compute the canonical signing payload (detached).
    pub fn signing_payload(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(&self.snapshot, domain_tags::SIGNATURE_PAYLOAD)
    }
}

impl CanonicalEncode for Attestation {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.snapshot.encode_canonical(out);
        out.extend_from_slice(&self.signer_id);
        self.signature.encode_canonical(out);
    }
}

// ─── Snapshot ──────────────────────────────────────────────

/// The canonical snapshot body.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snapshot {
    pub timestamp: u64,
    pub commit_hash: [u8; 32],
    pub parent_attestation_hash: [u8; 32],
    pub registry_root: [u8; 32],
    pub graph_root: [u8; 32],
    pub authority_root: [u8; 32],
    pub freeze_root: [u8; 32],
    pub release_root: [u8; 32],
    pub policy_root: [u8; 32],
}

impl Snapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timestamp: u64,
        commit_hash: [u8; 32],
        parent_attestation_hash: [u8; 32],
        registry_root: [u8; 32],
        graph_root: [u8; 32],
        authority_root: [u8; 32],
        freeze_root: [u8; 32],
        release_root: [u8; 32],
        policy_root: [u8; 32],
    ) -> Self {
        Self {
            timestamp,
            commit_hash,
            parent_attestation_hash,
            registry_root,
            graph_root,
            authority_root,
            freeze_root,
            release_root,
            policy_root,
        }
    }

    pub fn snapshot_hash(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::SNAPSHOT)
    }
}

impl CanonicalEncode for Snapshot {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.timestamp.encode_canonical(out);
        out.extend_from_slice(&self.commit_hash);
        out.extend_from_slice(&self.parent_attestation_hash);
        out.extend_from_slice(&self.registry_root);
        out.extend_from_slice(&self.graph_root);
        out.extend_from_slice(&self.authority_root);
        out.extend_from_slice(&self.freeze_root);
        out.extend_from_slice(&self.release_root);
        out.extend_from_slice(&self.policy_root);
    }
}

// ─── ReleaseSeal ───────────────────────────────────────────

/// A release seal binding artifacts to constitutional state.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseSeal {
    pub governance_schema_version: u64,
    pub protocol_version: u64,
    pub attestation_hash: [u8; 32],
    pub binary_root: [u8; 32],
    pub freeze_roots: Vec<[u8; 32]>,
    pub signer_id: [u8; 32],
    pub signature: Signature,
}

impl ReleaseSeal {
    pub fn seal_hash(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::RELEASE_SEAL)
    }

    pub fn signing_payload(&self) -> [u8; 32] {
        let mut payload = Vec::new();

        self.governance_schema_version
            .encode_canonical(&mut payload);

        self.protocol_version.encode_canonical(&mut payload);

        payload.extend_from_slice(&self.attestation_hash);
        payload.extend_from_slice(&self.binary_root);

        let mut sorted = self.freeze_roots.clone();
        sorted.sort();

        (sorted.len() as u64).encode_canonical(&mut payload);

        for root in &sorted {
            payload.extend_from_slice(root);
        }

        CanonicalEncoder::hash_value(
            &crate::canonical::CanonicalBytes(&payload),
            domain_tags::SIGNATURE_PAYLOAD,
        )
    }
}

impl CanonicalEncode for ReleaseSeal {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.protocol_version.encode_canonical(out);
        out.extend_from_slice(&self.attestation_hash);
        out.extend_from_slice(&self.binary_root);
        let mut sorted = self.freeze_roots.clone();
        sorted.sort();
        (sorted.len() as u64).encode_canonical(out);
        for root in &sorted {
            out.extend_from_slice(root);
        }
        out.extend_from_slice(&self.signer_id);
        self.signature.encode_canonical(out);
    }
}
