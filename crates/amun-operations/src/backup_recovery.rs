use amun_state_sync::sync_package::ConstitutionalSyncPackage;
use amun_state_sync::stateless_verifier::{StatelessVerifier, SyncVerificationResult};
use amun_resource_core::ResourceRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeBackup {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    pub sync_package: ConstitutionalSyncPackage,
    pub created_at: u64,
    pub backup_hash: [u8; 32],
}

impl NodeBackup {
    pub fn new(
        height: u64,
        block_hash: [u8; 32],
        history_root: [u8; 32],
        sync_package: ConstitutionalSyncPackage,
        created_at: u64,
    ) -> Self {
        let state_root = sync_package.snapshot_certificate.state_root;
        let mut backup = Self {
            height, block_hash, state_root, history_root,
            sync_package, created_at, backup_hash: [0u8; 32],
        };
        backup.backup_hash = backup.compute_hash();
        backup
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_NODE_BACKUP_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.history_root);
        hasher.update(&self.created_at.to_le_bytes());
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.backup_hash == self.compute_hash()
    }

    pub fn restore(&self) -> Result<ResourceRegistry, String> {
        let result = StatelessVerifier::verify(&self.sync_package, self.history_root);
        match result {
            SyncVerificationResult::Verified { .. } => {
                let mut registry = ResourceRegistry::new(self.sync_package.total_resources() * 2);
                for chunk in &self.sync_package.chunks {
                    for meta in &chunk.resources {
                        registry.register_genesis(meta.clone())
                            .map_err(|e| format!("Restore error: {:?}", e))?;
                    }
                }
                Ok(registry)
            }
            SyncVerificationResult::Failed { reason } => Err(reason),
        }
    }
}

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

    #[test]
    fn n62_backup_create_and_verify() {
        let mut reg = ResourceRegistry::new(100);
        for i in 0..10u8 {
            reg.register_genesis(ResourceMetadata {
                resource_id: make_id(i),
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(make_id(i)),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            }).unwrap();
        }
        let history_root = [0x10; 32];
        let package = amun_validator_networking::sync_transport::SyncTransport::export_snapshot(
            &reg, 42, [0xab; 32], history_root, "backup-test".into(),
        );
        let backup = NodeBackup::new(42, [0xab; 32], history_root, package, 1000);
        assert!(backup.verify());
    }

    #[test]
    fn n62_backup_restore() {
        let mut reg = ResourceRegistry::new(100);
        for i in 0..10u8 {
            reg.register_genesis(ResourceMetadata {
                resource_id: make_id(i),
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(make_id(i)),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            }).unwrap();
        }
        let state_root = reg.compute_state_root();
        let history_root = [0x10; 32];
        let package = amun_validator_networking::sync_transport::SyncTransport::export_snapshot(
            &reg, 42, [0xab; 32], history_root, "backup-test".into(),
        );
        let backup = NodeBackup::new(42, [0xab; 32], history_root, package, 1000);
        let restored = backup.restore().unwrap();
        assert_eq!(restored.compute_state_root(), state_root);
        assert_eq!(restored.total(), 10);
    }
}
