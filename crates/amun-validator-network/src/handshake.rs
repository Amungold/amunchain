use crate::config::NetworkConfig;
use amun_validator_api::error::{NetworkError, NetworkErrorCode, PlatformError, PlatformResult};
use amun_validator_api::types::id::PeerId;
use amun_validator_identity::IdentityProvider;
use std::sync::Arc;

pub struct HandshakeProtocol {
    identity: Arc<dyn IdentityProvider>,
    config: NetworkConfig,
}

impl HandshakeProtocol {
    pub fn new(identity: Arc<dyn IdentityProvider>, config: NetworkConfig) -> Self {
        HandshakeProtocol { identity, config }
    }

    pub fn perform_handshake(
        &self,
        peer_id: &PeerId,
        peer_protocol: u32,
        peer_chain: &str,
        peer_network: &str,
        peer_genesis: &[u8; 32],
    ) -> PlatformResult<HandshakeResult> {
        self.identity.self_check()?;
        if peer_id.as_bytes() == &[0u8; 32] {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                "Zero peer ID".into(),
            )));
        }
        if peer_protocol != self.config.protocol_version {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                format!(
                    "Protocol {} vs {}",
                    peer_protocol, self.config.protocol_version
                ),
            )));
        }
        if peer_chain != self.config.chain_id {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                format!("Chain {} vs {}", peer_chain, self.config.chain_id),
            )));
        }
        if peer_network != self.config.network_id {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                format!("Network {} vs {}", peer_network, self.config.network_id),
            )));
        }
        if peer_genesis != &self.config.genesis_hash {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                "Genesis hash mismatch".into(),
            )));
        }
        Ok(HandshakeResult {
            peer_id: *peer_id,
            our_validator_id: *self.identity.validator_id(),
            handshake_complete: true,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HandshakeResult {
    pub peer_id: PeerId,
    pub our_validator_id: amun_validator_api::types::id::ValidatorId,
    pub handshake_complete: bool,
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
    fn test_ok() {
        assert!(HandshakeProtocol::new(id(), NetworkConfig::default())
            .perform_handshake(&PeerId([1u8; 32]), 1, "amun-testnet-1", "amun", &[0u8; 32])
            .is_ok());
    }
    #[test]
    fn test_protocol() {
        assert!(HandshakeProtocol::new(id(), NetworkConfig::default())
            .perform_handshake(&PeerId([1u8; 32]), 99, "amun-testnet-1", "amun", &[0u8; 32])
            .is_err());
    }
    #[test]
    fn test_chain() {
        assert!(HandshakeProtocol::new(id(), NetworkConfig::default())
            .perform_handshake(&PeerId([1u8; 32]), 1, "other", "amun", &[0u8; 32])
            .is_err());
    }
    #[test]
    fn test_network() {
        assert!(HandshakeProtocol::new(id(), NetworkConfig::default())
            .perform_handshake(
                &PeerId([1u8; 32]),
                1,
                "amun-testnet-1",
                "other-net",
                &[0u8; 32]
            )
            .is_err());
    }
    #[test]
    fn test_genesis() {
        assert!(HandshakeProtocol::new(id(), NetworkConfig::default())
            .perform_handshake(&PeerId([1u8; 32]), 1, "amun-testnet-1", "amun", &[1u8; 32])
            .is_err());
    }
}
