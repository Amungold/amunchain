use crate::config::NetworkConfig;
use crate::connection::ConnectionManager;
use crate::discovery::DiscoveryService;
use crate::handshake::HandshakeProtocol;
use crate::message::{MessageType, NetworkMessage};
use crate::peer::PeerTable;
use crate::router::Router;
use crate::scheduler::HeartbeatScheduler;
use crate::sync::SyncService;
use crate::transport::{MockTransport, TransportProvider};
use amun_validator_api::error::{NetworkError, NetworkErrorCode, PlatformError, PlatformResult};
use amun_validator_api::types::id::PeerId;
use amun_validator_api::NetworkProvider;
use amun_validator_identity::IdentityProvider;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct NetworkService {
    config: NetworkConfig,
    peer_table: Arc<PeerTable>,
    discovery: DiscoveryService,
    handshake: HandshakeProtocol,
    connection_manager: Arc<ConnectionManager>,
    router: Router,
    transport: Arc<dyn TransportProvider>,
    scheduler: HeartbeatScheduler,
    sync: Arc<SyncService>,
    running: AtomicBool,
    sender_id: [u8; 32],
}

impl NetworkService {
    pub fn new(identity: Arc<dyn IdentityProvider>, config: NetworkConfig) -> Self {
        let peer_table = Arc::new(PeerTable::new());
        let connection_manager =
            Arc::new(ConnectionManager::new(peer_table.clone(), config.clone()));
        let discovery = DiscoveryService::new(peer_table.clone(), config.bootstrap_peers.clone());
        let handshake = HandshakeProtocol::new(identity.clone(), config.clone());
        let transport: Arc<dyn TransportProvider> = Arc::new(MockTransport::new());
        let router = Router::new(peer_table.clone(), transport.clone());
        let scheduler = HeartbeatScheduler::new(peer_table.clone(), transport.clone());
        let sync = Arc::new(SyncService::new());
        let sender_id = *identity.validator_id().as_bytes();

        NetworkService {
            config,
            peer_table,
            discovery,
            handshake,
            connection_manager,
            router,
            transport,
            scheduler,
            sync,
            running: AtomicBool::new(false),
            sender_id,
        }
    }

    pub fn discover_peers(&self) -> PlatformResult<usize> {
        self.discovery.discover()
    }
    pub fn sync_service(&self) -> &Arc<SyncService> {
        &self.sync
    }
}

impl NetworkProvider for NetworkService {
    fn start(&self) -> PlatformResult<()> {
        self.transport
            .start_listening(&self.config.listen_address())?;
        self.discover_peers()?;
        self.scheduler.start();
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }
    fn stop(&self) -> PlatformResult<()> {
        self.running.store(false, Ordering::SeqCst);
        self.scheduler.stop();
        self.transport.stop()?;
        Ok(())
    }
    fn peer_count(&self) -> PlatformResult<usize> {
        Ok(self.peer_table.connected_peers().len())
    }
    fn is_connected(&self) -> PlatformResult<bool> {
        Ok(self.peer_count()? > 0)
    }
    fn listen_address(&self) -> PlatformResult<String> {
        Ok(self.config.listen_address())
    }

    fn check_ports(&self) -> PlatformResult<()> {
        if self.config.listen_host.is_empty() {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::PortUnavailable,
                "Empty host".into(),
            )));
        }
        Ok(())
    }

    fn connect_to_peer(&self, address: &str) -> PlatformResult<PeerId> {
        let mut id = [0u8; 32];
        let b = address.as_bytes();
        id[..b.len().min(32)].copy_from_slice(&b[..b.len().min(32)]);
        let peer_id = PeerId(id);
        self.handshake.perform_handshake(
            &peer_id,
            self.config.protocol_version,
            &self.config.chain_id,
            &self.config.network_id,
            &self.config.genesis_hash,
        )?;
        self.connection_manager.connect(&peer_id, address)?;
        Ok(peer_id)
    }

    fn disconnect_peer(&self, pid: &PeerId) -> PlatformResult<()> {
        self.connection_manager.disconnect(pid);
        Ok(())
    }

    fn broadcast(&self, payload: &[u8]) -> PlatformResult<()> {
        let msg = NetworkMessage::new(MessageType::Vote, self.sender_id, payload.to_vec());
        self.router.broadcast(&msg)
    }

    fn send_to_peer(&self, pid: &PeerId, payload: &[u8]) -> PlatformResult<()> {
        let msg = NetworkMessage::new(MessageType::Vote, self.sender_id, payload.to_vec());
        self.router.route_to_peer(pid, &msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_validator_api::types::id::PublicKey;
    use amun_validator_identity::authority_store::AuthorityStore;
    use amun_validator_identity::certificate_store::CertificateStore;
    use amun_validator_identity::key_store::KeyStore;
    use amun_validator_identity::IdentityService;

    fn id() -> Arc<dyn IdentityProvider> {
        let k = Arc::new(KeyStore::generate());
        let c = Arc::new(CertificateStore::new(
            CertificateStore::load_from_file("x").unwrap(),
        ));
        let a = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        Arc::new(IdentityService::new(c, k, a))
    }

    #[test]
    fn test_start_stop() {
        let s = NetworkService::new(id(), NetworkConfig::default());
        assert!(s.start().is_ok());
        assert!(s.stop().is_ok());
    }
    #[test]
    fn test_connect() {
        let s = NetworkService::new(id(), NetworkConfig::default());
        assert!(s.connect_to_peer("test:8000").is_ok());
    }
    #[test]
    fn test_max_peers() {
        let c = NetworkConfig { max_peers: 0, ..NetworkConfig::default() };
        assert!(NetworkService::new(id(), c)
            .connect_to_peer("t:8000")
            .is_err());
    }
    #[test]
    fn test_discover() {
        let c = NetworkConfig { bootstrap_peers: vec!["s:9000".into()], ..NetworkConfig::default() };
        let s = NetworkService::new(id(), c);
        s.discover_peers().unwrap();
        assert!(s.peer_table.count() >= 1);
    }
}
