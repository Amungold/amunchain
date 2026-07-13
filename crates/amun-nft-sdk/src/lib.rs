use amun_nft_constitutional_registry::ConstitutionalRegistry;
use amun_nft_core::{NftEvidence, NftMetadata};
use amun_nft_indexer::NftIndexer;
use amun_nft_marketplace::MarketplaceEngine;
use amun_resource_core::{
    RegistryError, ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
    ResourceRegistry, ResourceState,
};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

pub struct NftSdk {
    pub registry: Arc<Mutex<ResourceRegistry>>,
    pub marketplace: Arc<Mutex<MarketplaceEngine>>,
    pub constitutional_registry: Arc<Mutex<ConstitutionalRegistry>>,
    pub indexer: Arc<Mutex<NftIndexer>>,
    pub evidence_log: Arc<Mutex<Vec<NftEvidence>>>,
}

impl Default for NftSdk {
    fn default() -> Self {
        Self::new()
    }
}

impl NftSdk {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(ResourceRegistry::new(1000))),
            marketplace: Arc::new(Mutex::new(MarketplaceEngine::new())),
            constitutional_registry: Arc::new(Mutex::new(ConstitutionalRegistry::new())),
            indexer: Arc::new(Mutex::new(NftIndexer::new())),
            evidence_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register_collection(
        &self,
        id: [u8; 32],
        creator: [u8; 32],
    ) -> Result<(), RegistryError> {
        let meta = ResourceMetadata {
            resource_id: ResourceId(id),
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(ResourceId(id)),
            contract_id: [0u8; 32],
            owner: creator,
        };
        self.registry.lock().unwrap().register_genesis(meta)
    }

    pub fn mint_nft(
        &self,
        collection_id: [u8; 32],
        token_id: [u8; 32],
        owner: [u8; 32],
        _metadata: &NftMetadata,
    ) -> Result<ResourceId, RegistryError> {
        let col_id = ResourceId(collection_id);
        let tok_id = ResourceId(token_id);
        let parent_hash = self.registry.lock().unwrap().resource_hash(&col_id)?;
        let version = self
            .registry
            .lock()
            .unwrap()
            .get(&col_id)
            .unwrap()
            .lineage
            .version
            + 1;
        let child = ResourceMetadata {
            resource_id: tok_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(tok_id, col_id, parent_hash, version),
            contract_id: [0u8; 32],
            owner,
        };
        self.registry
            .lock()
            .unwrap()
            .derive_from_collection(&col_id, child)
    }

    pub fn transfer_nft(
        &self,
        token_id: [u8; 32],
        new_owner: [u8; 32],
    ) -> Result<ResourceId, RegistryError> {
        let tid = ResourceId(token_id);
        let parent = self
            .registry
            .lock()
            .unwrap()
            .get(&tid)
            .cloned()
            .ok_or(RegistryError::NotFound(tid))?;
        let parent_hash = self.registry.lock().unwrap().resource_hash(&tid)?;
        let version = parent.lineage.version + 1;
        let new_id = ResourceId(sha256_combine(&token_id, &new_owner));
        let child = ResourceMetadata {
            resource_id: new_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(new_id, tid, parent_hash, version),
            contract_id: [0u8; 32],
            owner: new_owner,
        };
        self.registry
            .lock()
            .unwrap()
            .consume_and_derive(&tid, child)
    }

    pub fn list_nft(
        &self,
        token_id: [u8; 32],
        seller: [u8; 32],
        price: u64,
    ) -> Result<(), RegistryError> {
        let tid = ResourceId(token_id);
        let reg = self.registry.lock().unwrap();
        self.marketplace
            .lock()
            .unwrap()
            .list_nft(&reg, tid, &seller, price, None)
    }

    pub fn buy_nft(
        &self,
        token_id: [u8; 32],
        buyer: [u8; 32],
        block_height: u64,
        timestamp: u64,
    ) -> Result<ResourceId, RegistryError> {
        let tid = ResourceId(token_id);
        let mut reg = self.registry.lock().unwrap();
        self.marketplace
            .lock()
            .unwrap()
            .buy_nft(&mut reg, &tid, &buyer, block_height, timestamp)
    }

    pub fn start_auction(
        &self,
        token_id: [u8; 32],
        seller: [u8; 32],
        end_time: u64,
    ) -> Result<(), RegistryError> {
        let tid = ResourceId(token_id);
        let reg = self.registry.lock().unwrap();
        self.marketplace
            .lock()
            .unwrap()
            .start_auction(&reg, tid, &seller, end_time)
    }

    pub fn place_bid(
        &self,
        token_id: [u8; 32],
        bidder: [u8; 32],
        amount: u64,
        current_time: u64,
    ) -> Result<(), RegistryError> {
        let tid = ResourceId(token_id);
        self.marketplace
            .lock()
            .unwrap()
            .place_bid(&tid, &bidder, amount, current_time)
    }

    pub fn end_auction(
        &self,
        token_id: [u8; 32],
        current_time: u64,
        block_height: u64,
        timestamp: u64,
    ) -> Result<ResourceId, RegistryError> {
        let tid = ResourceId(token_id);
        let mut reg = self.registry.lock().unwrap();
        self.marketplace.lock().unwrap().end_auction(
            &mut reg,
            &tid,
            current_time,
            block_height,
            timestamp,
        )
    }

    pub fn index_all(&self) {
        let reg = self.constitutional_registry.lock().unwrap();
        self.indexer.lock().unwrap().index_registry(&reg);
    }

    pub fn get_nft_info(&self, token_id: [u8; 32]) -> Option<amun_nft_indexer::IndexedNft> {
        self.indexer.lock().unwrap().get_nft(&token_id).cloned()
    }
}

fn sha256_combine(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().into()
}
