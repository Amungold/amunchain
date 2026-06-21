use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry, RegistryError,
};
use std::collections::HashMap;

/// Marketplace listing
#[derive(Debug, Clone)]
pub struct Listing {
    pub token_id: ResourceId,
    pub seller: [u8; 32],
    pub price: u64,
    pub buyer: Option<[u8; 32]>,
    pub active: bool,
}

/// Auction
#[derive(Debug, Clone)]
pub struct Auction {
    pub token_id: ResourceId,
    pub seller: [u8; 32],
    pub highest_bidder: Option<[u8; 32]>,
    pub highest_bid: u64,
    pub end_time: u64,
    pub ended: bool,
}

#[derive(Debug, Clone)]
pub struct Bid {
    pub bidder: [u8; 32],
    pub amount: u64,
}

/// Marketplace event (for evidence)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplaceEvent {
    ListingCreated { token_id: ResourceId, seller: [u8; 32], price: u64 },
    ListingCancelled { token_id: ResourceId },
    SaleCompleted { token_id: ResourceId, seller: [u8; 32], buyer: [u8; 32], price: u64 },
    AuctionStarted { token_id: ResourceId, seller: [u8; 32], end_time: u64 },
    BidPlaced { token_id: ResourceId, bidder: [u8; 32], amount: u64 },
    AuctionEnded { token_id: ResourceId, winner: [u8; 32], price: u64 },
}

/// Marketplace engine with hardening
pub struct MarketplaceEngine {
    listings: HashMap<ResourceId, Listing>,
    auctions: HashMap<ResourceId, Auction>,
    event_log: Vec<MarketplaceEvent>,
}

impl Default for MarketplaceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketplaceEngine {
    pub fn new() -> Self {
        Self {
            listings: HashMap::new(),
            auctions: HashMap::new(),
            event_log: Vec::new(),
        }
    }

    /// List an NFT for sale
    pub fn list_nft(
        &mut self,
        registry: &ResourceRegistry,
        token_id: ResourceId,
        seller: &[u8; 32],
        price: u64,
        intended_buyer: Option<[u8; 32]>,
    ) -> Result<(), RegistryError> {
        // Verify token exists and seller owns it
        let token = registry.get(&token_id)
            .ok_or(RegistryError::NotFound(token_id))?;
        if token.owner != *seller {
            return Err(RegistryError::NotActive(token_id));
        }
        if !matches!(token.state, ResourceState::Active) {
            return Err(RegistryError::NotActive(token_id));
        }
        // Prevent double listing
        if self.listings.contains_key(&token_id) && self.listings.get(&token_id).unwrap().active {
            return Err(RegistryError::NotActive(token_id));
        }

        let listing = Listing {
            token_id,
            seller: *seller,
            price,
            buyer: intended_buyer,
            active: true,
        };
        self.listings.insert(token_id, listing);
        self.event_log.push(MarketplaceEvent::ListingCreated { token_id, seller: *seller, price });
        Ok(())
    }

    /// Buy a listed NFT
    pub fn buy_nft(
        &mut self,
        registry: &mut ResourceRegistry,
        token_id: &ResourceId,
        buyer: &[u8; 32],
        _block_height: u64,
        _timestamp: u64,
    ) -> Result<ResourceId, RegistryError> {
        // Check listing exists and is active
        let listing = self.listings.get(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        if !listing.active {
            return Err(RegistryError::NotActive(*token_id));
        }
        // Prevent self-purchase
        if listing.seller == *buyer {
            return Err(RegistryError::NotActive(*token_id));
        }
        // Check intended buyer if set
        if let Some(intended) = listing.buyer {
            if intended != *buyer {
                return Err(RegistryError::NotActive(*token_id));
            }
        }
        // Verify token still exists and seller still owns it
        let token = registry.get(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        if token.owner != listing.seller {
            return Err(RegistryError::NotActive(*token_id));
        }

        let seller = listing.seller;
        let price = listing.price;

        // Transfer ownership: consume current, produce new
        let parent_hash = registry.resource_hash(token_id)?;
        let version = token.lineage.version + 1;
        let new_id = derive_transfer_id(token_id, buyer);
        let child_meta = ResourceMetadata {
            resource_id: new_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(new_id, *token_id, parent_hash, version),
            contract_id: [0u8; 32],
            owner: *buyer,
        };
        registry.consume_and_derive(token_id, child_meta)?;

        // Mark listing inactive AFTER successful transfer
        if let Some(l) = self.listings.get_mut(token_id) {
            l.active = false;
        }

        self.event_log.push(MarketplaceEvent::SaleCompleted {
            token_id: *token_id,
            seller,
            buyer: *buyer,
            price,
        });
        Ok(new_id)
    }

    /// Cancel a listing
    pub fn cancel_listing(&mut self, token_id: &ResourceId) -> Result<(), RegistryError> {
        let listing = self.listings.get_mut(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        if !listing.active {
            return Err(RegistryError::NotActive(*token_id));
        }
        listing.active = false;
        self.event_log.push(MarketplaceEvent::ListingCancelled { token_id: *token_id });
        Ok(())
    }

    /// Start an auction
    pub fn start_auction(
        &mut self,
        registry: &ResourceRegistry,
        token_id: ResourceId,
        seller: &[u8; 32],
        end_time: u64,
    ) -> Result<(), RegistryError> {
        let token = registry.get(&token_id)
            .ok_or(RegistryError::NotFound(token_id))?;
        if token.owner != *seller {
            return Err(RegistryError::NotActive(token_id));
        }
        if self.auctions.contains_key(&token_id) {
            return Err(RegistryError::NotActive(token_id));
        }

        let auction = Auction {
            token_id,
            seller: *seller,
            highest_bidder: None,
            highest_bid: 0,
            end_time,
            ended: false,
        };
        self.auctions.insert(token_id, auction);
        self.event_log.push(MarketplaceEvent::AuctionStarted { token_id, seller: *seller, end_time });
        Ok(())
    }

    /// Place a bid
    pub fn place_bid(
        &mut self,
        token_id: &ResourceId,
        bidder: &[u8; 32],
        amount: u64,
        current_time: u64,
    ) -> Result<(), RegistryError> {
        let auction = self.auctions.get_mut(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        if auction.ended || current_time >= auction.end_time {
            return Err(RegistryError::NotActive(*token_id));
        }
        if amount <= auction.highest_bid {
            return Err(RegistryError::VersionMismatch { expected: auction.highest_bid + 1, actual: amount });
        }
        auction.highest_bid = amount;
        auction.highest_bidder = Some(*bidder);
        self.event_log.push(MarketplaceEvent::BidPlaced { token_id: *token_id, bidder: *bidder, amount });
        Ok(())
    }

    /// End auction
    pub fn end_auction(
        &mut self,
        registry: &mut ResourceRegistry,
        token_id: &ResourceId,
        current_time: u64,
        _block_height: u64,
        _timestamp: u64,
    ) -> Result<ResourceId, RegistryError> {
        let auction = self.auctions.get(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        if auction.ended || current_time < auction.end_time {
            return Err(RegistryError::NotActive(*token_id));
        }
        let winner = auction.highest_bidder
            .ok_or(RegistryError::NotActive(*token_id))?;
        let price = auction.highest_bid;

        let token = registry.get(token_id)
            .ok_or(RegistryError::NotFound(*token_id))?;
        let parent_hash = registry.resource_hash(token_id)?;
        let version = token.lineage.version + 1;
        let new_id = derive_transfer_id(token_id, &winner);
        let child_meta = ResourceMetadata {
            resource_id: new_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(new_id, *token_id, parent_hash, version),
            contract_id: [0u8; 32],
            owner: winner,
        };
        registry.consume_and_derive(token_id, child_meta)?;

        if let Some(a) = self.auctions.get_mut(token_id) {
            a.ended = true;
        }

        self.event_log.push(MarketplaceEvent::AuctionEnded { token_id: *token_id, winner, price });
        Ok(new_id)
    }

    /// Evidence log accessor
    pub fn event_log(&self) -> &[MarketplaceEvent] {
        &self.event_log
    }

    /// Compute marketplace evidence root from event log
    pub fn compute_evidence_root(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_MARKETPLACE_EVIDENCE_V1");
        for event in &self.event_log {
            let event_bytes = format!("{:?}", event);
            hasher.update(event_bytes.as_bytes());
        }
        hasher.finalize().into()
    }
}

fn derive_transfer_id(old_id: &ResourceId, new_owner: &[u8; 32]) -> ResourceId {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(old_id.0);
    hasher.update(new_owner);
    ResourceId(hasher.finalize().into())
}
