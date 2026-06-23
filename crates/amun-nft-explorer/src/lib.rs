use amun_nft_core::NftMetadata;
use amun_resource_core::{ResourceArchetype, ResourceId, ResourceRegistry};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Data structures for the explorer API
#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerCollection {
    pub collection_id: ResourceId,
    pub name: String,
    pub creator: [u8; 32],
    pub total_supply: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerNft {
    pub token_id: ResourceId,
    pub owner: [u8; 32],
    pub archetype: String,
    pub metadata: Option<NftMetadata>,
    pub mining_origin: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerOwner {
    pub address: [u8; 32],
    pub nft_count: usize,
    pub nfts: Vec<ResourceId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerTransfer {
    pub token_id: ResourceId,
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub timestamp: u64,
}

/// Query engine
pub struct ExplorerEngine;

impl ExplorerEngine {
    /// Get all collections (filter by archetype NFTCollection)
    pub fn get_collections(registry: &ResourceRegistry) -> Vec<ExplorerCollection> {
        registry
            .active_resources()
            .iter()
            .filter(|m| m.archetype == ResourceArchetype::NFTCollection)
            .map(|m| ExplorerCollection {
                collection_id: m.resource_id,
                name: "Constitutional Collection".into(), // can be extended
                creator: m.owner,
                total_supply: 0, // can be computed
            })
            .collect()
    }

    /// Get a single NFT by ID
    pub fn get_nft(registry: &ResourceRegistry, token_id: &ResourceId) -> Option<ExplorerNft> {
        registry.get(token_id).map(|m| ExplorerNft {
            token_id: m.resource_id,
            owner: m.owner,
            archetype: format!("{:?}", m.archetype),
            metadata: None,
            mining_origin: None,
        })
    }

    /// Get all NFTs owned by an address
    pub fn get_owner_nfts(registry: &ResourceRegistry, owner: &[u8; 32]) -> ExplorerOwner {
        let nfts: Vec<ResourceId> = registry
            .active_resources()
            .iter()
            .filter(|m| m.owner == *owner && m.archetype == ResourceArchetype::NFTAsset)
            .map(|m| m.resource_id)
            .collect();
        ExplorerOwner {
            address: *owner,
            nft_count: nfts.len(),
            nfts,
        }
    }

    /// Get transfer history (from lineage) - simplified
    pub fn get_transfer_history(
        registry: &ResourceRegistry,
        token_id: &ResourceId,
    ) -> Vec<ExplorerTransfer> {
        let mut history = vec![];
        let mut current_id = *token_id;
        // Walk backwards through lineage until genesis
        while let Some(meta) = registry.get(&current_id) {
            if meta.lineage.parent_resource_ids.is_empty() {
                break;
            }
            let parent_id = meta.lineage.parent_resource_ids[0];
            if let Some(parent_meta) = registry.get(&parent_id) {
                history.push(ExplorerTransfer {
                    token_id: current_id,
                    from: parent_meta.owner,
                    to: meta.owner,
                    timestamp: 0, // no timestamp in metadata, can be extended
                });
            }
            current_id = parent_id;
        }
        history.reverse();
        history
    }
}

/// Simple HTTP server to serve API and static pages.
pub fn start_explorer_server(registry: Arc<Mutex<ResourceRegistry>>, bind_addr: &str) {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener = TcpListener::bind(bind_addr).expect("Failed to bind");
    println!("Explorer server running on http://{}", bind_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                let _ = stream.read(&mut buf);
                let request = String::from_utf8_lossy(&buf);
                let path = if let Some(line) = request.lines().next() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        parts[1].to_string()
                    } else {
                        "/".into()
                    }
                } else {
                    "/".into()
                };

                let response = handle_request(&path, &registry.lock().unwrap());
                let _ = stream.write_all(response.as_bytes());
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

fn handle_request(path: &str, registry: &ResourceRegistry) -> String {
    let json_header = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
    match path {
        "/api/collections" => {
            let data = ExplorerEngine::get_collections(registry);
            format!("{}{}", json_header, serde_json::to_string(&data).unwrap())
        }
        "/api/nfts" => {
            // Return all active NFTs
            let nfts: Vec<ExplorerNft> = registry
                .active_resources()
                .iter()
                .filter(|m| m.archetype == ResourceArchetype::NFTAsset)
                .map(|m| ExplorerNft {
                    token_id: m.resource_id,
                    owner: m.owner,
                    archetype: format!("{:?}", m.archetype),
                    metadata: None,
                    mining_origin: None,
                })
                .collect();
            format!("{}{}", json_header, serde_json::to_string(&nfts).unwrap())
        }
        _ => {
            let html = "<html><body><h1>Amun NFT Explorer</h1></body></html>";
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", html)
        }
    }
}
