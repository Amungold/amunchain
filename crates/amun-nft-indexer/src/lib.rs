use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use amun_nft_constitutional_registry::ConstitutionalRegistry;
use amun_nft_marketplace::MarketplaceEvent;

/// Indexed NFT data for fast query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedNft {
    pub token_id: [u8; 32],
    pub owner: [u8; 32],
    pub collection_id: Option<[u8; 32]>,
    pub creator: [u8; 32],
    pub royalty_bps: Option<u16>,
    pub governance_voting_power: u64,
    pub bridge_locked: bool,
    pub mining_origin: Option<String>,
}

/// Indexed event for marketplace/bridge/governance activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedEvent {
    pub token_id: [u8; 32],
    pub event_type: String,
    pub data: String,
    pub block_height: u64,
}

/// Indexer engine
#[derive(Debug, Clone, Default)]
pub struct NftIndexer {
    pub nfts: BTreeMap<[u8; 32], IndexedNft>,
    pub events: Vec<IndexedEvent>,
}

impl NftIndexer {
    pub fn new() -> Self {
        Self { nfts: BTreeMap::new(), events: Vec::new() }
    }

    /// Index all NFTs from the constitutional registry
    pub fn index_registry(&mut self, registry: &ConstitutionalRegistry) {
        for (id, record) in &registry.records {
            self.nfts.insert(*id, IndexedNft {
                token_id: *id,
                owner: record.owner,
                collection_id: record.collection_id,
                creator: record.creator,
                royalty_bps: record.royalty_policy.as_ref().map(|p| p.royalty_bps),
                governance_voting_power: record.governance_right.as_ref().map(|g| g.voting_power).unwrap_or(0),
                bridge_locked: record.bridge_lock.is_some(),
                mining_origin: record.mining_origin.clone(),
            });
        }
    }

    /// Index marketplace events
    pub fn index_marketplace_events(&mut self, events: &[MarketplaceEvent], block_height: u64) {
        for ev in events {
            let (token_id, event_type, data) = match ev {
                MarketplaceEvent::ListingCreated { token_id, seller, price } =>
                    (token_id.0, "ListingCreated", format!("seller={:?}, price={}", seller, price)),
                MarketplaceEvent::ListingCancelled { token_id } =>
                    (token_id.0, "ListingCancelled", String::new()),
                MarketplaceEvent::SaleCompleted { token_id, seller, buyer, price } =>
                    (token_id.0, "SaleCompleted", format!("seller={:?}, buyer={:?}, price={}", seller, buyer, price)),
                MarketplaceEvent::AuctionStarted { token_id, seller, end_time } =>
                    (token_id.0, "AuctionStarted", format!("seller={:?}, end_time={}", seller, end_time)),
                MarketplaceEvent::BidPlaced { token_id, bidder, amount } =>
                    (token_id.0, "BidPlaced", format!("bidder={:?}, amount={}", bidder, amount)),
                MarketplaceEvent::AuctionEnded { token_id, winner, price } =>
                    (token_id.0, "AuctionEnded", format!("winner={:?}, price={}", winner, price)),
            };
            self.events.push(IndexedEvent { token_id, event_type: event_type.to_string(), data, block_height });
        }
    }
    /// Query NFT by token ID
    pub fn get_nft(&self, token_id: &[u8; 32]) -> Option<&IndexedNft> {
        self.nfts.get(token_id)
    }

    /// Query all NFTs owned by an address
    pub fn get_nfts_by_owner(&self, owner: &[u8; 32]) -> Vec<&IndexedNft> {
        self.nfts.values().filter(|n| n.owner == *owner).collect()
    }

    /// Query events for a token
    pub fn get_events_by_token(&self, token_id: &[u8; 32]) -> Vec<&IndexedEvent> {
        self.events.iter().filter(|e| e.token_id == *token_id).collect()
    }

    /// Compute deterministic index root
    pub fn compute_index_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_NFT_INDEX_V1");
        for (id, nft) in &self.nfts {
            hasher.update(id);
            let bytes = serde_json::to_vec(nft).unwrap();
            hasher.update(&bytes);
        }
        for event in &self.events {
            let bytes = serde_json::to_vec(event).unwrap();
            hasher.update(&bytes);
        }
        hasher.finalize().into()
    }
}
