use amun_resource_core::ResourceRegistry;

use crate::sync_package::ConstitutionalSyncPackage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncVerificationResult {
    Verified {
        state_root: [u8; 32],
        resources_imported: usize,
    },
    Failed {
        reason: String,
    },
}

pub struct StatelessVerifier;

impl StatelessVerifier {
    pub fn verify(
        package: &ConstitutionalSyncPackage,
        trusted_history_root: [u8; 32],
    ) -> SyncVerificationResult {
        if !package.verify() {
            return SyncVerificationResult::Failed {
                reason: "Package hash mismatch".into(),
            };
        }

        let cert = &package.snapshot_certificate;
        if !cert.verify() {
            return SyncVerificationResult::Failed {
                reason: "Snapshot certificate verification failed".into(),
            };
        }

        if cert.history_root != trusted_history_root {
            return SyncVerificationResult::Failed {
                reason: "History root mismatch".into(),
            };
        }

        if package.chunks.len() as u32 != cert.chunk_count {
            return SyncVerificationResult::Failed {
                reason: "Chunk count mismatch".into(),
            };
        }

        if package.chunk_proofs.len() != package.chunks.len() {
            return SyncVerificationResult::Failed {
                reason: "Proof count mismatch".into(),
            };
        }

        for chunk in &package.chunks {
            if !chunk.verify() {
                return SyncVerificationResult::Failed {
                    reason: format!("Chunk {} integrity failed", chunk.chunk_id),
                };
            }
        }

        for (chunk, proof) in package.chunks.iter().zip(package.chunk_proofs.iter()) {
            if chunk.chunk_id != proof.chunk_id || chunk.chunk_hash != proof.chunk_hash {
                return SyncVerificationResult::Failed {
                    reason: format!("Chunk/proof mismatch at chunk {}", chunk.chunk_id),
                };
            }
            if !proof.verify() {
                return SyncVerificationResult::Failed {
                    reason: format!("Merkle proof failed for chunk {}", proof.chunk_id),
                };
            }
            if proof.chunk_root != cert.chunk_root {
                return SyncVerificationResult::Failed {
                    reason: format!("Chunk root mismatch at chunk {}", proof.chunk_id),
                };
            }
        }

        let mut all_resources: Vec<amun_resource_core::ResourceMetadata> = Vec::new();
        for chunk in &package.chunks {
            for meta in &chunk.resources {
                all_resources.push(meta.clone());
            }
        }

        let mut temp_reg = ResourceRegistry::new(all_resources.len() * 2);
        for meta in &all_resources {
            let _ = temp_reg.register_genesis(meta.clone());
        }
        let reconstructed_root = temp_reg.compute_state_root();

        if reconstructed_root != cert.state_root {
            return SyncVerificationResult::Failed {
                reason: "State root mismatch after reconstruction".into(),
            };
        }

        let total_resources: u64 = package
            .chunks
            .iter()
            .map(|c| c.resource_count() as u64)
            .sum();
        if total_resources != cert.total_resources {
            return SyncVerificationResult::Failed {
                reason: "Resource count mismatch".into(),
            };
        }

        SyncVerificationResult::Verified {
            state_root: reconstructed_root,
            resources_imported: all_resources.len(),
        }
    }
}
