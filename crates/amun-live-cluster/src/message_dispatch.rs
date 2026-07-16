use crate::network_adapter::ValidatorNetworkAdapter;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::consensus_message::ConsensusMessage;
use amun_consensus_network::engine::ConsensusEngine;
use amun_networking::frame::{FrameKind, NetworkFrame};
use amun_sync::request_manager::SyncRequestManager;
use amun_sync::sync_messages::{
    BlockRangeRequestPayload, BlockRangeResponsePayload, RequestKey, TipRequestPayload,
    TipResponsePayload, MAX_BLOCK_RANGE,
};
use postcard;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MessageDispatcher {
    engine: Arc<Mutex<ConsensusEngine>>,
    store: Arc<Mutex<ChainStore>>,
    network: ValidatorNetworkAdapter,
    sync_requests: SyncRequestManager,
}

impl MessageDispatcher {
    pub fn new(
        engine: Arc<Mutex<ConsensusEngine>>,
        store: Arc<Mutex<ChainStore>>,
        network: ValidatorNetworkAdapter,
        sync_requests: SyncRequestManager,
    ) -> Self {
        Self {
            engine,
            store,
            network,
            sync_requests,
        }
    }

    pub fn dispatch(&self, peer: SocketAddr, frame: NetworkFrame) {
        match frame.kind {
            FrameKind::Ping => {
                // Ping received — transport layer handles Pong response
            }
            FrameKind::Pong => {
                // Pong received — RTT tracked at transport layer
            }
            FrameKind::TipRequest => self.handle_tip_request(peer, &frame),
            FrameKind::TipResponse => self.handle_tip_response(peer, &frame),
            FrameKind::BlockRangeRequest => self.handle_block_range_request(peer, &frame),
            FrameKind::BlockRangeResponse => self.handle_block_range_response(peer, &frame),
            FrameKind::Vote => self.handle_vote(&frame),
            FrameKind::ConsensusMessage => self.handle_consensus_message(&frame),
            _ => {}
        }
    }

    fn handle_tip_request(&self, peer: SocketAddr, frame: &NetworkFrame) {
        if let Ok(request) = postcard::from_bytes::<TipRequestPayload>(&frame.payload) {
            let store = self.store.lock().expect("mutex poisoned");
            let tip = store.load_tip();
            let height = tip.as_ref().map(|r| r.height).unwrap_or(0);
            let hash = tip.map(|r| r.block_hash).unwrap_or([0u8; 32]);

            let response_payload = TipResponsePayload {
                request_id: request.request_id,
                height,
                hash,
            };

            let response = NetworkFrame::new(
                FrameKind::TipResponse,
                bytes::Bytes::from(
                    postcard::to_stdvec(&response_payload).expect("Response serialization failed"),
                ),
            );

            if let Err(e) = self.network.send_to(peer, response) {
                eprintln!("Failed to send TipResponse to {}: {}", peer, e);
            }
        }
    }

    fn handle_tip_response(&self, peer: SocketAddr, frame: &NetworkFrame) {
        if let Ok(response) = postcard::from_bytes::<TipResponsePayload>(&frame.payload) {
            let key = RequestKey {
                peer,
                request_id: response.request_id,
            };
            self.sync_requests.complete_request(key, frame.clone());
        }
    }

    fn handle_block_range_request(&self, peer: SocketAddr, frame: &NetworkFrame) {
        if let Ok(request) = postcard::from_bytes::<BlockRangeRequestPayload>(&frame.payload) {
            if request.start > request.end {
                return;
            }
            let range_size = request.end - request.start + 1;
            if range_size > MAX_BLOCK_RANGE {
                return;
            }

            let store = self.store.lock().expect("mutex poisoned");
            let records = store.load_height_range(request.start, request.end);
            let encoded: Vec<Vec<u8>> = records.iter().map(|r| r.encode()).collect();

            let response_payload = BlockRangeResponsePayload {
                request_id: request.request_id,
                records: encoded,
            };

            let response = NetworkFrame::new(
                FrameKind::BlockRangeResponse,
                bytes::Bytes::from(
                    postcard::to_stdvec(&response_payload).expect("Response serialization failed"),
                ),
            );

            if let Err(e) = self.network.send_to(peer, response) {
                eprintln!("Failed to send BlockRangeResponse to {}: {}", peer, e);
            }
        }
    }

    fn handle_block_range_response(&self, peer: SocketAddr, frame: &NetworkFrame) {
        if let Ok(response) = postcard::from_bytes::<BlockRangeResponsePayload>(&frame.payload) {
            let key = RequestKey {
                peer,
                request_id: response.request_id,
            };
            self.sync_requests.complete_request(key, frame.clone());
        }
    }

    fn handle_consensus_message(&self, frame: &NetworkFrame) {
        if let Ok(msg) = postcard::from_bytes::<ConsensusMessage>(&frame.payload) {
            match msg {
                ConsensusMessage::Vote(vote) => {
                    let mut eng = self.engine.lock().expect("mutex poisoned");
                    let _ = eng.process_vote(&vote);
                }
                ConsensusMessage::Proposal(_proposal) => {
                    // R3.3: Proposal routing placeholder
                    eprintln!("CONSENSUS_MSG: Proposal received (routing TBD)");
                }
                ConsensusMessage::QuorumCertificate(_qc) => {
                    // R3.3: QC routing placeholder
                    eprintln!("CONSENSUS_MSG: QC received (routing TBD)");
                }
                ConsensusMessage::Finality(_cert) => {
                    // R3.3: Finality routing placeholder
                    eprintln!("CONSENSUS_MSG: Finality received (routing TBD)");
                }
            }
        }
    }

    fn handle_vote(&self, frame: &NetworkFrame) {
        if let Ok(vote) =
            postcard::from_bytes::<amun_consensus_network::messages::ConsensusVote>(&frame.payload)
        {
            let mut eng = self.engine.lock().expect("mutex poisoned");
            let _ = eng.process_vote(&vote);
        }
    }
}
