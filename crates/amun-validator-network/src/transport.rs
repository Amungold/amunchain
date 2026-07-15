use crate::codec::BinaryCodec;
use crate::message::NetworkMessage;
use amun_validator_api::error::{NetworkError, NetworkErrorCode, PlatformError, PlatformResult};

pub trait TransportProvider: Send + Sync {
    fn send(&self, address: &str, message: &NetworkMessage) -> PlatformResult<()>;
    fn start_listening(&self, address: &str) -> PlatformResult<()>;
    fn stop(&self) -> PlatformResult<()>;
}

pub struct MockTransport;

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTransport {
    pub fn new() -> Self {
        MockTransport
    }
}

impl TransportProvider for MockTransport {
    fn send(&self, _address: &str, message: &NetworkMessage) -> PlatformResult<()> {
        BinaryCodec::encode(message)?;
        Ok(())
    }

    fn start_listening(&self, address: &str) -> PlatformResult<()> {
        if address.is_empty() {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::PortUnavailable,
                "Address empty".into(),
            )));
        }
        Ok(())
    }

    fn stop(&self) -> PlatformResult<()> {
        Ok(())
    }
}
