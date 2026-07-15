#[derive(Debug, Clone)]
pub struct PlatformVersion {
    pub manifest_version: u32,
    pub platform_version: String,
    pub protocol_version: u32,
    pub consensus_version: u32,
    pub storage_version: u32,
    pub api_version: u32,
    pub identity_version: u32,
    pub constitution_version: u32,
    pub network_version: u32,
    pub serialization_version: u32,
    pub audit_version: u32,
}

impl Default for PlatformVersion {
    fn default() -> Self {
        PlatformVersion {
            manifest_version: 1,
            platform_version: String::from("1.0.0"),
            protocol_version: 1,
            consensus_version: 1,
            storage_version: 1,
            api_version: 1,
            identity_version: 1,
            constitution_version: 1,
            network_version: 1,
            serialization_version: 1,
            audit_version: 1,
        }
    }
}

impl PlatformVersion {
    pub fn is_compatible(&self, other: &PlatformVersion) -> bool {
        self.protocol_version == other.protocol_version
            && self.constitution_version == other.constitution_version
    }
}
