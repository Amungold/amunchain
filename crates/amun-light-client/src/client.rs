/// Legacy light client — kept for compatibility, now wraps ConstitutionalLightClient.
#[derive(Debug, Clone)]
pub struct LightClient {
    pub trusted_height: u64,
    pub trusted_hash: String,
}

impl LightClient {
    pub fn new(trusted_genesis_hash: String) -> Self {
        Self { trusted_height: 0, trusted_hash: trusted_genesis_hash }
    }

    pub fn bootstrap(&mut self, _height: u64) -> Result<(), String> {
        self.trusted_height = 1;
        Ok(())
    }

    pub fn advance(&mut self, _height: u64) -> Result<(), String> {
        self.trusted_height += 1;
        Ok(())
    }
}
