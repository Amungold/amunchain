use amun_resource_core::ResourceRegistry;
use std::collections::BTreeMap;

pub struct CachedMerkleState {
    pub root: [u8; 32],
    pub version: u64,
}

pub struct OptimizedRegistry {
    pub registry: ResourceRegistry,
    pub cache: BTreeMap<[u8; 32], CachedMerkleState>,
    pub state_version: u64,
}

impl OptimizedRegistry {
    pub fn new(max_lineage_depth: usize) -> Self {
        Self {
            registry: ResourceRegistry::new(max_lineage_depth),
            cache: BTreeMap::new(),
            state_version: 0,
        }
    }

    pub fn compute_state_root(&mut self) -> [u8; 32] {
        let root = self.registry.compute_state_root();
        self.cache.insert(
            root,
            CachedMerkleState {
                root,
                version: self.state_version,
            },
        );
        self.state_version += 1;
        root
    }

    pub fn state_root(&self) -> [u8; 32] {
        self.registry.compute_state_root()
    }
}
