use amun_constitutional_commitments::SparseMerkleTree;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod keys {
    pub const TRANSITION: &[u8] = b"transition/";
    pub const AMENDMENT: &[u8] = b"amendment/";
    pub const VALIDATOR: &[u8] = b"validator/";
    pub const GOVERNANCE: &[u8] = b"governance/";
    pub const EVIDENCE: &[u8] = b"evidence/";
}

/// A record of a state transition for deterministic replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransitionRecord {
    pub transition_id: [u8; 32],
    pub transition_hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct ConstitutionalStateRuntime {
    state: BTreeMap<Vec<u8>, [u8; 32]>,
    journal: Vec<StateTransitionRecord>,
}

impl ConstitutionalStateRuntime {
    pub fn new() -> Self {
        Self {
            state: BTreeMap::new(),
            journal: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8; 32]) {
        self.state.insert(key.to_vec(), *value);
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.state.remove(key);
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8; 32]> {
        self.state.get(key)
    }

    pub fn canonical_key(prefix: &[u8], id: &[u8; 32]) -> Vec<u8> {
        let mut key = prefix.to_vec();
        key.extend_from_slice(hex::encode(id).as_bytes());
        key
    }

    pub fn state_root(&self) -> [u8; 32] {
        let mut tree = SparseMerkleTree::new(b"AMUN_STATE_DOMAIN");
        for (key, value) in &self.state {
            tree.insert(key, value);
        }
        tree.root()
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }
    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    /// Record a transition and update state.
    pub fn apply_transition(&mut self, transition_id: &[u8; 32], transition_hash: &[u8; 32]) {
        let key = Self::canonical_key(keys::TRANSITION, transition_id);
        self.set(&key, transition_hash);
        self.journal.push(StateTransitionRecord {
            transition_id: *transition_id,
            transition_hash: *transition_hash,
        });
    }

    /// Returns the journal of all applied transitions.
    pub fn journal(&self) -> &[StateTransitionRecord] {
        &self.journal
    }

    /// Replay a journal to reconstruct the state.
    pub fn replay(records: &[StateTransitionRecord]) -> Self {
        let mut rt = Self::new();
        for record in records {
            rt.apply_transition(&record.transition_id, &record.transition_hash);
        }
        rt
    }
}

impl Default for ConstitutionalStateRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.set(b"alice", &[100u8; 32]);
        assert_eq!(rt.get(b"alice"), Some(&[100u8; 32]));
    }

    #[test]
    fn test_delete() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.set(b"alice", &[100u8; 32]);
        rt.delete(b"alice");
        assert_eq!(rt.get(b"alice"), None);
    }

    #[test]
    fn test_state_root_determinism() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        let mut rt2 = ConstitutionalStateRuntime::new();
        rt1.set(b"alice", &[100u8; 32]);
        rt2.set(b"alice", &[100u8; 32]);
        assert_eq!(rt1.state_root(), rt2.state_root());
    }

    #[test]
    fn test_state_root_sensitivity() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        let mut rt2 = ConstitutionalStateRuntime::new();
        rt1.set(b"alice", &[100u8; 32]);
        rt2.set(b"alice", &[200u8; 32]);
        assert_ne!(rt1.state_root(), rt2.state_root());
    }

    #[test]
    fn test_delete_returns_empty_root() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.set(b"alice", &[100u8; 32]);
        rt.delete(b"alice");
        let empty = ConstitutionalStateRuntime::new().state_root();
        assert_eq!(rt.state_root(), empty);
    }

    #[test]
    fn test_canonical_key() {
        let id = [0xAAu8; 32];
        let key = ConstitutionalStateRuntime::canonical_key(keys::TRANSITION, &id);
        assert!(key.starts_with(b"transition/"));
    }

    #[test]
    fn test_transition_changes_state_root() {
        let mut rt = ConstitutionalStateRuntime::new();
        let root_before = rt.state_root();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        assert_ne!(root_before, rt.state_root());
    }

    #[test]
    fn test_same_transition_same_root() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        let mut rt2 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt2.apply_transition(&[1u8; 32], &[0xAA; 32]);
        assert_eq!(rt1.state_root(), rt2.state_root());
    }

    #[test]
    fn test_replay_produces_same_root() {
        let mut original = ConstitutionalStateRuntime::new();
        original.apply_transition(&[1u8; 32], &[0xAA; 32]);
        original.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let root1 = original.state_root();

        let replayed = ConstitutionalStateRuntime::replay(original.journal());
        let root2 = replayed.state_root();
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_replay_order_independence() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt1.apply_transition(&[2u8; 32], &[0xBB; 32]);

        let records = rt1.journal().to_vec();
        let mut reversed = records.clone();
        reversed.reverse();
        let rt2 = ConstitutionalStateRuntime::replay(&reversed);
        // Same set, different order → SMT uses hash keys → same root
        assert_eq!(rt1.state_root(), rt2.state_root());
    }
}

/// A certificate proving that a state root was produced by a specific journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayCertificate {
    pub block_height: u64,
    pub transition_count: u64,
    pub pre_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
    pub journal_root: [u8; 32],
}

impl ConstitutionalStateRuntime {
    /// Compute a Merkle root of the journal entries.
    pub fn journal_root(&self) -> [u8; 32] {
        let mut tree = SparseMerkleTree::new(b"AMUN_JOURNAL_DOMAIN");
        for (i, record) in self.journal.iter().enumerate() {
            let key = format!("{:020}", i);
            let mut value = [0u8; 32];
            value[..].copy_from_slice(&record.transition_hash);
            tree.insert(key.as_bytes(), &value);
        }
        tree.root()
    }

    /// Create a replay certificate for a block transition.
    pub fn create_certificate(
        &self,
        block_height: u64,
        pre_state_root: [u8; 32],
    ) -> ReplayCertificate {
        ReplayCertificate {
            block_height,
            transition_count: self.journal.len() as u64,
            pre_state_root,
            post_state_root: self.state_root(),
            journal_root: self.journal_root(),
        }
    }
}

impl ReplayCertificate {
    /// Verify that replaying the journal produces the claimed post_state_root.
    pub fn verify(&self, records: &[StateTransitionRecord]) -> bool {
        let replayed = ConstitutionalStateRuntime::replay(records);
        replayed.state_root() == self.post_state_root
            && replayed.journal_root() == self.journal_root
            && records.len() as u64 == self.transition_count
    }
}

#[cfg(test)]
mod certificate_tests {
    use super::*;

    #[test]
    fn test_replay_certificate_valid() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt.apply_transition(&[2u8; 32], &[0xBB; 32]);

        let pre_root = [0u8; 32]; // genesis empty root
        let cert = rt.create_certificate(1, pre_root);
        assert!(cert.verify(rt.journal()));
    }

    #[test]
    fn test_replay_certificate_detects_tampering() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt.apply_transition(&[2u8; 32], &[0xBB; 32]);

        let cert = rt.create_certificate(1, [0u8; 32]);
        let mut tampered = rt.journal().to_vec();
        tampered[0].transition_hash = [0xFF; 32]; // corrupt first transition

        assert!(!cert.verify(&tampered));
    }
}

// ============================================================
// Phase N1: certificate_hash() — blake3 + domain separation
// ============================================================

impl ReplayCertificate {
    /// Compute a deterministic constitutional hash of this certificate.
    ///
    /// Uses blake3 with domain separation: `AMUN_REPLAY_CERTIFICATE_V1`.
    /// All 5 fields are included: block_height, transition_count,
    /// pre_state_root, post_state_root, journal_root.
    pub fn certificate_hash(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.block_height.to_le_bytes());
        bytes.extend_from_slice(&self.transition_count.to_le_bytes());
        bytes.extend_from_slice(&self.pre_state_root);
        bytes.extend_from_slice(&self.post_state_root);
        bytes.extend_from_slice(&self.journal_root);

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_REPLAY_CERTIFICATE_V1");
        hasher.update(&bytes);
        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(hash.as_bytes());
        result
    }
}

#[cfg(test)]
mod n1_tests {
    use super::*;

    #[test]
    fn n1_hash_deterministic() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt.create_certificate(1, [0u8; 32]);
        let c2 = rt.create_certificate(1, [0u8; 32]);
        assert_eq!(c1.certificate_hash(), c2.certificate_hash());
    }

    #[test]
    fn n1_hash_sensitive_to_height() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt.create_certificate(1, [0u8; 32]);
        let c2 = rt.create_certificate(2, [0u8; 32]);
        assert_ne!(c1.certificate_hash(), c2.certificate_hash());
    }

    #[test]
    fn n1_hash_sensitive_to_state() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        let mut rt2 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt2.apply_transition(&[1u8; 32], &[0xBB; 32]);
        let c1 = rt1.create_certificate(1, [0u8; 32]);
        let c2 = rt2.create_certificate(1, [0u8; 32]);
        assert_ne!(c1.certificate_hash(), c2.certificate_hash());
    }

    #[test]
    fn n1_hash_sensitive_to_pre_state() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt.create_certificate(1, [0u8; 32]);
        let c2 = rt.create_certificate(1, [1u8; 32]);
        assert_ne!(c1.certificate_hash(), c2.certificate_hash());
    }

    #[test]
    fn n1_hash_sensitive_to_journal() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        let mut rt2 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt2.apply_transition(&[1u8; 32], &[0xAA; 32]);
        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let c1 = rt1.create_certificate(1, [0u8; 32]);
        let c2 = rt2.create_certificate(1, [0u8; 32]);
        assert_ne!(c1.certificate_hash(), c2.certificate_hash());
    }
}

impl ConstitutionalStateRuntime {
    /// Build a Merkle root from multiple ReplayCertificates.
    /// Even with a single certificate, this provides future-proof structure.
    pub fn certificate_merkle_root(certificates: &[ReplayCertificate]) -> [u8; 32] {
        let mut tree = SparseMerkleTree::new(b"AMUN_CERTIFICATE_MERKLE_DOMAIN");
        for (i, cert) in certificates.iter().enumerate() {
            let key = format!("{:020}", i);
            tree.insert(key.as_bytes(), &cert.certificate_hash());
        }
        tree.root()
    }
}

#[cfg(test)]
mod n3_tests {
    use super::*;

    #[test]
    fn n3_single_certificate_merkle_root() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let root = ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&cert));
        // Single cert: Merkle root must be deterministic
        let root2 = ConstitutionalStateRuntime::certificate_merkle_root(&[cert]);
        assert_eq!(root, root2);
        assert_ne!(root, [0u8; 32]); // must not be empty
    }

    #[test]
    fn n3_multiple_certificates_different_root() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt1.create_certificate(1, [0u8; 32]);

        let mut rt2 = ConstitutionalStateRuntime::new();
        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let c2 = rt2.create_certificate(1, [0u8; 32]);

        let root_single =
            ConstitutionalStateRuntime::certificate_merkle_root(std::slice::from_ref(&c1));
        let root_multi = ConstitutionalStateRuntime::certificate_merkle_root(&[c1, c2]);
        assert_ne!(root_single, root_multi);
    }
}

use amun_constitutional_commitments::smt::MerkleProof;

/// A Merkle proof that a specific ReplayCertificate is included
/// in a certificate merkle root stored in a block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInclusionProof {
    pub certificate_hash: [u8; 32],
    pub index: u64,
    pub proof: MerkleProof,
    pub root: [u8; 32],
}

impl ConstitutionalStateRuntime {
    /// Generate an inclusion proof that a certificate with the given hash
    /// is part of the merkle root computed from the provided certificates.
    pub fn prove_certificate_inclusion(
        certificates: &[ReplayCertificate],
        cert_hash: &[u8; 32],
    ) -> Option<CertificateInclusionProof> {
        let index = certificates
            .iter()
            .position(|c| c.certificate_hash() == *cert_hash)?;
        let mut tree = SparseMerkleTree::new(b"AMUN_CERTIFICATE_MERKLE_DOMAIN");
        for (i, cert) in certificates.iter().enumerate() {
            let key = format!("{:020}", i);
            tree.insert(key.as_bytes(), &cert.certificate_hash());
        }
        let key = format!("{:020}", index);
        let proof = tree.prove(key.as_bytes());
        Some(CertificateInclusionProof {
            certificate_hash: *cert_hash,
            index: index as u64,
            proof,
            root: tree.root(),
        })
    }
}

impl CertificateInclusionProof {
    /// Verify that this proof is valid against its stored root.
    pub fn verify(&self) -> bool {
        // Verify that the stored certificate_hash matches the leaf_value in the Merkle proof
        if self.proof.leaf_value != Some(self.certificate_hash) {
            return false;
        }
        let tree = SparseMerkleTree::new(b"AMUN_CERTIFICATE_MERKLE_DOMAIN");
        tree.verify(&self.root, &self.proof)
    }
}

#[cfg(test)]
mod n7_tests {
    use super::*;

    #[test]
    fn n7_inclusion_proof_valid() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt1.create_certificate(1, [0u8; 32]);

        let mut rt2 = ConstitutionalStateRuntime::new();
        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let c2 = rt2.create_certificate(1, [0u8; 32]);

        let certs = vec![c1.clone(), c2.clone()];
        let root = ConstitutionalStateRuntime::certificate_merkle_root(&certs);
        let hash = c1.certificate_hash();

        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        assert_eq!(proof.root, root);
        assert!(proof.verify());
    }

    #[test]
    fn n7_inclusion_proof_wrong_certificate_fails() {
        let mut rt1 = ConstitutionalStateRuntime::new();
        rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let c1 = rt1.create_certificate(1, [0u8; 32]);

        let mut rt2 = ConstitutionalStateRuntime::new();
        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
        let c2 = rt2.create_certificate(1, [0u8; 32]);

        let certs = vec![c1.clone(), c2.clone()];
        let hash = c1.certificate_hash();

        let mut proof =
            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        // Tamper with the certificate hash
        proof.certificate_hash = [0xFF; 32];
        assert!(!proof.verify());
    }

    #[test]
    fn n7_inclusion_proof_missing_certificate() {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(1, [0u8; 32]);
        let certs = vec![cert];

        let unknown_hash = [0xFF; 32];
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &unknown_hash);
        assert!(proof.is_none());
    }
}
