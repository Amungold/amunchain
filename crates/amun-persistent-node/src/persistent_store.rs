use amun_operations::backup_recovery::NodeBackup;
use amun_resource_core::{ResourceMetadata, ResourceRegistry};
use amun_validator_networking::sync_transport::SyncTransport;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PersistentValidatorStore {
    data_dir: PathBuf,
    registry: ResourceRegistry,
    current_height: u64,
    current_block_hash: [u8; 32],
    history_root: [u8; 32],
}

impl PersistentValidatorStore {
    pub fn open(data_dir: &str) -> Result<Self, String> {
        let path = PathBuf::from(data_dir);
        fs::create_dir_all(&path).map_err(|e| format!("Cannot create data dir: {}", e))?;
        let backup_file = path.join("backup.json");
        if backup_file.exists() {
            Self::load_from_backup(&path)
        } else {
            Ok(Self {
                data_dir: path,
                registry: ResourceRegistry::new(1_000_000),
                current_height: 0,
                current_block_hash: [0u8; 32],
                history_root: [0u8; 32],
            })
        }
    }

    fn load_from_backup(path: &Path) -> Result<Self, String> {
        let backup_file = path.join("backup.json");
        let json = fs::read_to_string(&backup_file).map_err(|e| format!("Read error: {}", e))?;
        let backup: NodeBackup =
            serde_json::from_str(&json).map_err(|e| format!("Deserialization error: {}", e))?;
        if !backup.verify() {
            return Err("Backup verification failed".into());
        }
        let registry = backup.restore()?;
        Ok(Self {
            data_dir: path.to_path_buf(),
            registry,
            current_height: backup.height,
            current_block_hash: backup.block_hash,
            history_root: backup.history_root,
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let package = SyncTransport::export_snapshot(
            &self.registry,
            self.current_height,
            self.current_block_hash,
            self.history_root,
            "persistent-store".into(),
        );
        let backup = NodeBackup::new(
            self.current_height,
            self.current_block_hash,
            self.history_root,
            package,
            0,
        );
        let json = serde_json::to_string_pretty(&backup)
            .map_err(|e| format!("Serialization error: {}", e))?;
        let backup_file = self.data_dir.join("backup.json");
        let temp_file = self.data_dir.join("backup.tmp");
        fs::write(&temp_file, &json).map_err(|e| format!("Temp write error: {}", e))?;
        fs::rename(&temp_file, &backup_file).map_err(|e| format!("Rename error: {}", e))?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), String> {
        let backup_file = self.data_dir.join("backup.json");
        if !backup_file.exists() {
            return Err("No backup file found".into());
        }
        let json = fs::read_to_string(&backup_file).map_err(|e| format!("Read error: {}", e))?;
        let backup: NodeBackup =
            serde_json::from_str(&json).map_err(|e| format!("Deserialization error: {}", e))?;
        if !backup.verify() {
            return Err("Backup verification failed — file may be corrupted".into());
        }
        let registry = backup.restore()?;
        self.registry = registry;
        self.current_height = backup.height;
        self.current_block_hash = backup.block_hash;
        self.history_root = backup.history_root;
        Ok(())
    }

    pub fn advance(
        &mut self,
        height: u64,
        block_hash: [u8; 32],
        history_root: [u8; 32],
        new_resources: Vec<ResourceMetadata>,
    ) -> Result<(), String> {
        for meta in new_resources {
            self.registry
                .register_genesis(meta)
                .map_err(|e| format!("Register error: {:?}", e))?;
        }
        self.current_height = height;
        self.current_block_hash = block_hash;
        self.history_root = history_root;
        Ok(())
    }

    pub fn registry(&self) -> &ResourceRegistry {
        &self.registry
    }
    pub fn registry_mut(&mut self) -> &mut ResourceRegistry {
        &mut self.registry
    }
    pub fn current_height(&self) -> u64 {
        self.current_height
    }
    pub fn state_root(&self) -> [u8; 32] {
        self.registry.compute_state_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{
        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
    };

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    #[test]
    fn n63_open_new_store() {
        let dir = tempfile::tempdir().unwrap();
        let store = PersistentValidatorStore::open(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(store.current_height(), 0);
    }

    #[test]
    fn n63_save_and_restore() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        let mut store = PersistentValidatorStore::open(dir_str).unwrap();
        for i in 0..10u8 {
            store
                .registry_mut()
                .register_genesis(ResourceMetadata {
                    resource_id: make_id(i),
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(make_id(i)),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                })
                .unwrap();
        }
        let root_before = store.state_root();
        store.advance(42, [0xab; 32], [0x10; 32], vec![]).unwrap();
        store.save().unwrap();
        drop(store);
        let mut restored = PersistentValidatorStore::open(dir_str).unwrap();
        restored.restore().unwrap();
        assert_eq!(restored.current_height(), 42);
        assert_eq!(restored.state_root(), root_before);
    }

    #[test]
    fn n63_crash_recovery_full_cycle() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        let mut store = PersistentValidatorStore::open(dir_str).unwrap();
        for i in 0..50u8 {
            store
                .registry_mut()
                .register_genesis(ResourceMetadata {
                    resource_id: make_id(i),
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(make_id(i)),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                })
                .unwrap();
        }
        let state_root = store.state_root();
        store.advance(100, [0xcd; 32], [0x10; 32], vec![]).unwrap();
        store.save().unwrap();
        drop(store);
        let mut recovered = PersistentValidatorStore::open(dir_str).unwrap();
        recovered.restore().unwrap();
        assert_eq!(recovered.current_height(), 100);
        assert_eq!(recovered.state_root(), state_root);
    }

    #[test]
    fn n63_backup_tampering_detected() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        let mut store = PersistentValidatorStore::open(dir_str).unwrap();
        store.advance(1, [0xaa; 32], [0x10; 32], vec![]).unwrap();
        store.save().unwrap();
        let backup_file = dir.path().join("backup.json");
        let json = fs::read_to_string(&backup_file).unwrap();
        let mut backup: NodeBackup = serde_json::from_str(&json).unwrap();
        assert!(backup.verify());
        backup.height = 999;
        assert!(!backup.verify(), "Tampered backup should fail verification");
    }

    #[test]
    fn n63_open_rejects_corrupted_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        let mut store = PersistentValidatorStore::open(dir_str).unwrap();
        store.advance(1, [0xaa; 32], [0x10; 32], vec![]).unwrap();
        store.save().unwrap();
        drop(store);
        let backup_file = dir.path().join("backup.json");
        fs::write(&backup_file, "corrupted-garbage-data").unwrap();
        let result = PersistentValidatorStore::open(dir_str);
        assert!(
            result.is_err(),
            "Corrupted backup file should be rejected on open"
        );
    }
}
