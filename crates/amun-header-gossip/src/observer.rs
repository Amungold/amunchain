use crate::GossipMessage;
use amun_light_client::client::LightClient;

pub struct VerificationReport {
    pub verified: bool,
    pub height: u64,
}

pub struct GossipObserver {
    pub client: LightClient,
}

impl GossipObserver {
    pub fn observe(&mut self, _msg: &GossipMessage) -> Result<VerificationReport, String> {
        // Legacy observer — delegates to the constitutional light client
        self.client.advance(1)?;
        Ok(VerificationReport {
            verified: true,
            height: self.client.trusted_height,
        })
    }

    pub fn bootstrap_from_genesis(&mut self) -> Result<(), String> {
        self.client.bootstrap(0)
    }
}
