pub mod snapshot_certificate;
pub mod state_chunk;
pub mod sync_package;
pub mod stateless_verifier;

pub use snapshot_certificate::*;
pub use state_chunk::*;
pub use sync_package::*;
pub use stateless_verifier::*;

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{
        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
        ResourceRegistry, ResourceState,
    };

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    fn make_meta(id: ResourceId) -> ResourceMetadata {
        ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }
    }

    #[test]
    fn n56_snapshot_certificate_verify() {
        let cert = StateSnapshotCertificate::new(
            100, [0xab; 32], [0x01; 32], [0x02; 32], [0x03; 32],
            4, 1000, 5000, "test-verifier".into(),
        );
        assert!(cert.verify());
    }

    #[test]
    fn n56_snapshot_certificate_detects_tampering() {
        let mut cert = StateSnapshotCertificate::new(
            100, [0xab; 32], [0x01; 32], [0x02; 32], [0x03; 32],
            4, 1000, 5000, "test-verifier".into(),
        );
        cert.height = 999;
        assert!(!cert.verify());
    }

    #[test]
    fn n56_chunk_verify() {
        let resources = vec![make_meta(make_id(1)), make_meta(make_id(2))];
        let chunk = StateChunk::new(0, resources);
        assert!(chunk.verify());
    }

    #[test]
    fn n56_chunk_detects_tampering() {
        let resources = vec![make_meta(make_id(1))];
        let mut chunk = StateChunk::new(0, resources);
        chunk.resources[0].owner = [0xff; 32];
        assert!(!chunk.verify());
    }

    #[test]
    fn n56_chunk_merkle_tree_proofs() {
        let chunks: Vec<StateChunk> = (0..8).map(|i| {
            StateChunk::new(i, vec![make_meta(make_id(i as u8))])
        }).collect();
        let (root, proofs) = build_chunk_merkle_tree(&chunks);
        assert_ne!(root, [0u8; 32]);
        assert_eq!(proofs.len(), 8);
        for proof in &proofs {
            assert!(proof.verify());
            assert_eq!(proof.chunk_root, root);
        }
    }

    #[test]
    fn n56_full_sync_package_roundtrip() {
        // Build state
        let mut reg = ResourceRegistry::new(1000);
        for i in 0..20u8 {
            reg.register_genesis(make_meta(make_id(i))).unwrap();
        }
        let state_root = reg.compute_state_root();
        let history_root = [0x10; 32];
        let active = reg.active_resources();

        // Chunk the state
        let chunk_size = 5;
        let chunks: Vec<StateChunk> = active.chunks(chunk_size).enumerate().map(|(i, batch)| {
            StateChunk::new(i as u32, batch.iter().map(|m| (*m).clone()).collect())
        }).collect();
        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);

        // Create snapshot certificate
        let cert = StateSnapshotCertificate::new(
            42, [0xbb; 32], state_root, history_root, chunk_root,
            chunks.len() as u32, active.len() as u64, 6000, "amun-state-sync".into(),
        );

        // Create sync package
        let pkg = ConstitutionalSyncPackage::new(cert, chunks, chunk_proofs);
        assert!(pkg.verify());

        // Stateless verification
        let result = StatelessVerifier::verify(&pkg, history_root);
        match result {
            SyncVerificationResult::Verified { state_root: verified_root, resources_imported } => {
                assert_eq!(verified_root, state_root);
                assert_eq!(resources_imported, 20);
            }
            SyncVerificationResult::Failed { reason } => {
                panic!("Verification failed: {}", reason);
            }
        }
    }

    #[test]
    fn n56_reject_wrong_history_root() {
        let cert = StateSnapshotCertificate::new(
            1, [0u8; 32], [0u8; 32], [0x02; 32], [0u8; 32], 0, 0, 0, "v".into(),
        );
        let pkg = ConstitutionalSyncPackage::new(cert, vec![], vec![]);
        let result = StatelessVerifier::verify(&pkg, [0x99; 32]);
        assert!(matches!(result, SyncVerificationResult::Failed { .. }));
    }

    #[test]
    fn n56_reject_tampered_chunk() {
        let mut reg = ResourceRegistry::new(100);
        reg.register_genesis(make_meta(make_id(1))).unwrap();
        let state_root = reg.compute_state_root();
        let history_root = [0x10; 32];
        let active = reg.active_resources();
        let chunks = vec![StateChunk::new(0, active.iter().map(|m| (*m).clone()).collect())];
        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
        let cert = StateSnapshotCertificate::new(
            1, [0u8; 32], state_root, history_root, chunk_root,
            1, 1, 0, "v".into(),
        );

        // Tamper with a chunk
        let mut tampered_chunks = chunks.clone();
        tampered_chunks[0].resources[0].owner = [0xff; 32];
        tampered_chunks[0] = StateChunk::new(0, tampered_chunks[0].resources.clone());

        let pkg = ConstitutionalSyncPackage::new(cert, tampered_chunks, chunk_proofs);
        let result = StatelessVerifier::verify(&pkg, history_root);
        assert!(matches!(result, SyncVerificationResult::Failed { .. }));
    }

    #[test]
    fn n56_reject_missing_chunk() {
        let mut reg = ResourceRegistry::new(100);
        for i in 0..10u8 {
            reg.register_genesis(make_meta(make_id(i))).unwrap();
        }
        let state_root = reg.compute_state_root();
        let history_root = [0x10; 32];
        let active = reg.active_resources();
        let chunks: Vec<StateChunk> = active.chunks(5).enumerate().map(|(i, batch)| {
            StateChunk::new(i as u32, batch.iter().map(|m| (*m).clone()).collect())
        }).collect();
        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
        let cert = StateSnapshotCertificate::new(
            1, [0u8; 32], state_root, history_root, chunk_root,
            2, 10, 0, "v".into(),
        );

        // Remove one chunk
        let partial_chunks = vec![chunks[0].clone()];
        let partial_proofs = vec![chunk_proofs[0].clone()];

        let pkg = ConstitutionalSyncPackage::new(cert, partial_chunks, partial_proofs);
        let result = StatelessVerifier::verify(&pkg, history_root);
        assert!(matches!(result, SyncVerificationResult::Failed { .. }));
    }
}

// N65 — Sync Protocol module
pub mod sync_protocol;
