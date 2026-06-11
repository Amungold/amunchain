use amun_resource_core::ResourceRegistry;
use amun_state_sync::snapshot_certificate::StateSnapshotCertificate;
use amun_state_sync::state_chunk::{build_chunk_merkle_tree, StateChunk};
use amun_state_sync::stateless_verifier::{StatelessVerifier, SyncVerificationResult};
use amun_state_sync::sync_package::ConstitutionalSyncPackage;

pub struct SyncTransport;

impl SyncTransport {
    pub fn export_snapshot(
        registry: &ResourceRegistry,
        height: u64,
        block_hash: [u8; 32],
        history_root: [u8; 32],
        verifier: String,
    ) -> ConstitutionalSyncPackage {
        let active = registry.active_resources();
        let state_root = registry.compute_state_root();
        let chunk_size = 100.max(active.len() / 10);
        let chunks: Vec<StateChunk> = active
            .chunks(chunk_size)
            .enumerate()
            .map(|(i, batch)| {
                StateChunk::new(i as u32, batch.iter().map(|m| (*m).clone()).collect())
            })
            .collect();
        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
        let cert = StateSnapshotCertificate::new(
            height,
            block_hash,
            state_root,
            history_root,
            chunk_root,
            chunks.len() as u32,
            active.len() as u64,
            0,
            verifier,
        );
        ConstitutionalSyncPackage::new(cert, chunks, chunk_proofs)
    }

    pub fn import_snapshot(
        package: &ConstitutionalSyncPackage,
        trusted_history_root: [u8; 32],
    ) -> Result<ResourceRegistry, String> {
        let result = StatelessVerifier::verify(package, trusted_history_root);
        match result {
            SyncVerificationResult::Verified {
                state_root,
                resources_imported: _,
            } => {
                let mut registry = ResourceRegistry::new(package.total_resources() * 2);
                for chunk in &package.chunks {
                    for meta in &chunk.resources {
                        registry
                            .register_genesis(meta.clone())
                            .map_err(|e| format!("Import error: {:?}", e))?;
                    }
                }
                let imported_root = registry.compute_state_root();
                if imported_root != state_root {
                    return Err(format!(
                        "State root mismatch after import: expected {}, got {}",
                        hex::encode(state_root),
                        hex::encode(imported_root),
                    ));
                }
                Ok(registry)
            }
            SyncVerificationResult::Failed { reason } => {
                Err(format!("Stateless verification failed: {}", reason))
            }
        }
    }
}
