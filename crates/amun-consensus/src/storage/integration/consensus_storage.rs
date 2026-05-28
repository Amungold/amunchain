use std::sync::Arc;
use crate::storage::persistent_node_store::PersistentNodeStore;
use crate::storage::persistent_value_store::PersistentValueStore;
use crate::storage::smt::SparseMerkleTree;
use crate::storage::wal::WALWriter;
use crate::storage::transaction::AtomicTransaction;
use crate::storage::staging::StagingArea;
use crate::storage::version_manifest::{VersionManifest, VersionEntry};
use crate::storage::snapshot::SnapshotEngine;
use crate::storage::root_persistence::RootPersistence;
use crate::storage::recovery::RecoveryCoordinator;
use crate::state_tree::{Key256, ValueBlob, StateRoot};

pub struct ConsensusStorage {
    node_store: Arc<PersistentNodeStore>,
    value_store: Arc<PersistentValueStore>,
    tree: SparseMerkleTree,
    wal: Arc<WALWriter>,
    manifest: VersionManifest,
    snapshot_engine: SnapshotEngine,
    next_tx_id: std::sync::atomic::AtomicU64,
}

impl ConsensusStorage {
    pub fn new(data_dir: &str, wal_path: &str) -> Result<Self, &'static str> {
        let node_store = Arc::new(PersistentNodeStore::new(&format!("{}/nodes", data_dir)));
        let value_store = Arc::new(PersistentValueStore::new(&format!("{}/values", data_dir)));
        let tree = SparseMerkleTree::new(node_store.clone(), value_store.clone());
        let wal = Arc::new(WALWriter::new(wal_path).map_err(|_| "WAL init failed")?);
        let manifest = VersionManifest::new(&format!("{}/manifest.dat", data_dir));
        let snapshot_engine = SnapshotEngine::new(data_dir);
        Ok(Self {
            node_store, value_store, tree, wal, manifest, snapshot_engine,
            next_tx_id: std::sync::atomic::AtomicU64::new(1),
        })
    }
    
    pub fn begin_transaction(&self) -> AtomicTransaction {
        let tx_id = self.next_tx_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        AtomicTransaction::begin(tx_id)
    }
    
    pub fn commit_transaction(&mut self, tx: AtomicTransaction) -> Result<StateRoot, &'static str> {
        let (new_tree, new_root) = tx.commit(&self.wal, &self.tree)?;
        self.tree = new_tree;
        let version = self.next_tx_id.load(std::sync::atomic::Ordering::SeqCst);
        self.manifest.add(VersionEntry { version, parent: version-1, state_root: new_root, epoch: 0 })?;
        self.snapshot_engine.create(version, new_root, 0)?;
        Ok(new_root)
    }
    
    pub fn root(&self) -> StateRoot { self.tree.root() }
    
    pub fn recover() -> Result<Self, &'static str> {
        let (root, version) = RecoveryCoordinator::recover();
        // Would reconstruct tree from root using persistence
        Self::new("data", "wal.log")
    }
    
    pub fn create_staging(&self) -> StagingArea { StagingArea::new() }
    pub fn apply_staging(&mut self, staging: StagingArea) -> Result<StateRoot, &'static str> {
        let (new_tree, new_root) = staging.apply(&self.tree)?;
        self.tree = new_tree;
        Ok(new_root)
    }
}
