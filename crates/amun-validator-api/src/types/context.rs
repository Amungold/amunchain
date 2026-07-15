use crate::error::PlatformResult;
use crate::handles::{
    ConsensusHandle, DiscoveryHandle, GenesisHandle, HealthHandle, IdentityHandle, NetworkHandle,
    RecoveryHandle, RegistryHandle, StorageHandle, SyncHandle,
};
use crate::policy::ValidatorPolicy;
use crate::types::capabilities::ValidatorCapabilities;
use crate::types::state::StateMachine;
use crate::types::version::PlatformVersion;

pub struct RuntimeContext {
    pub identity: IdentityHandle,
    pub storage: StorageHandle,
    pub network: NetworkHandle,
    pub discovery: DiscoveryHandle,
    pub genesis: GenesisHandle,
    pub sync: SyncHandle,
    pub health: HealthHandle,
    pub consensus: ConsensusHandle,
    pub recovery: RecoveryHandle,
    pub registry: RegistryHandle,
    pub state_machine: StateMachine,
    pub capabilities: ValidatorCapabilities,
    pub platform_version: PlatformVersion,
    pub policy: ValidatorPolicy,
}

pub struct RuntimeContextBuilder {
    identity: Option<IdentityHandle>,
    storage: Option<StorageHandle>,
    network: Option<NetworkHandle>,
    discovery: Option<DiscoveryHandle>,
    genesis: Option<GenesisHandle>,
    sync: Option<SyncHandle>,
    health: Option<HealthHandle>,
    consensus: Option<ConsensusHandle>,
    recovery: Option<RecoveryHandle>,
    registry: Option<RegistryHandle>,
    capabilities: Option<ValidatorCapabilities>,
    platform_version: Option<PlatformVersion>,
    policy: Option<ValidatorPolicy>,
}

impl RuntimeContextBuilder {
    pub fn new() -> Self {
        RuntimeContextBuilder {
            identity: None,
            storage: None,
            network: None,
            discovery: None,
            genesis: None,
            sync: None,
            health: None,
            consensus: None,
            recovery: None,
            registry: None,
            capabilities: None,
            platform_version: None,
            policy: None,
        }
    }

    pub fn identity(mut self, v: IdentityHandle) -> Self {
        self.identity = Some(v);
        self
    }
    pub fn storage(mut self, v: StorageHandle) -> Self {
        self.storage = Some(v);
        self
    }
    pub fn network(mut self, v: NetworkHandle) -> Self {
        self.network = Some(v);
        self
    }
    pub fn discovery(mut self, v: DiscoveryHandle) -> Self {
        self.discovery = Some(v);
        self
    }
    pub fn genesis(mut self, v: GenesisHandle) -> Self {
        self.genesis = Some(v);
        self
    }
    pub fn sync(mut self, v: SyncHandle) -> Self {
        self.sync = Some(v);
        self
    }
    pub fn health(mut self, v: HealthHandle) -> Self {
        self.health = Some(v);
        self
    }
    pub fn consensus(mut self, v: ConsensusHandle) -> Self {
        self.consensus = Some(v);
        self
    }
    pub fn recovery(mut self, v: RecoveryHandle) -> Self {
        self.recovery = Some(v);
        self
    }
    pub fn registry(mut self, v: RegistryHandle) -> Self {
        self.registry = Some(v);
        self
    }
    pub fn capabilities(mut self, v: ValidatorCapabilities) -> Self {
        self.capabilities = Some(v);
        self
    }
    pub fn platform_version(mut self, v: PlatformVersion) -> Self {
        self.platform_version = Some(v);
        self
    }
    pub fn policy(mut self, v: ValidatorPolicy) -> Self {
        self.policy = Some(v);
        self
    }

    pub fn build(self) -> PlatformResult<RuntimeContext> {
        Ok(RuntimeContext {
            identity: self.identity.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "identity handle is required".into(),
                ))
            })?,
            storage: self.storage.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "storage handle is required".into(),
                ))
            })?,
            network: self.network.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "network handle is required".into(),
                ))
            })?,
            discovery: self.discovery.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "discovery handle is required".into(),
                ))
            })?,
            genesis: self.genesis.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "genesis handle is required".into(),
                ))
            })?,
            sync: self.sync.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "sync handle is required".into(),
                ))
            })?,
            health: self.health.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "health handle is required".into(),
                ))
            })?,
            consensus: self.consensus.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "consensus handle is required".into(),
                ))
            })?,
            recovery: self.recovery.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "recovery handle is required".into(),
                ))
            })?,
            registry: self.registry.ok_or_else(|| {
                crate::error::PlatformError::Config(crate::error::ConfigError::new(
                    crate::error::ConfigErrorCode::MissingField,
                    "registry handle is required".into(),
                ))
            })?,
            capabilities: self.capabilities.unwrap_or_default(),
            platform_version: self.platform_version.unwrap_or_default(),
            policy: self.policy.unwrap_or_default(),
            state_machine: StateMachine::new(),
        })
    }
}

impl Default for RuntimeContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
