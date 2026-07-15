use crate::traits::admin::ValidatorAdmin;
use crate::traits::consensus::ConsensusProvider;
use crate::traits::discovery::DiscoveryProvider;
use crate::traits::genesis::GenesisProvider;
use crate::traits::health::HealthProvider;
use crate::traits::identity::IdentityProvider;
use crate::traits::network::NetworkProvider;
use crate::traits::read::ValidatorRead;
use crate::traits::recovery::RecoveryProvider;
use crate::traits::storage::StorageProvider;
use crate::traits::sync::SyncProvider;
use std::fmt;

pub struct IdentityHandle {
    pub(crate) inner: Box<dyn IdentityProvider>,
}
impl IdentityHandle {
    pub fn new(inner: Box<dyn IdentityProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for IdentityHandle {
    type Target = dyn IdentityProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for IdentityHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityHandle").finish()
    }
}

pub struct StorageHandle {
    pub(crate) inner: Box<dyn StorageProvider>,
}
impl StorageHandle {
    pub fn new(inner: Box<dyn StorageProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for StorageHandle {
    type Target = dyn StorageProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for StorageHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageHandle").finish()
    }
}

pub struct NetworkHandle {
    pub(crate) inner: Box<dyn NetworkProvider>,
}
impl NetworkHandle {
    pub fn new(inner: Box<dyn NetworkProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for NetworkHandle {
    type Target = dyn NetworkProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for NetworkHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetworkHandle").finish()
    }
}

pub struct DiscoveryHandle {
    pub(crate) inner: Box<dyn DiscoveryProvider>,
}
impl DiscoveryHandle {
    pub fn new(inner: Box<dyn DiscoveryProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for DiscoveryHandle {
    type Target = dyn DiscoveryProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for DiscoveryHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiscoveryHandle").finish()
    }
}

pub struct GenesisHandle {
    pub(crate) inner: Box<dyn GenesisProvider>,
}
impl GenesisHandle {
    pub fn new(inner: Box<dyn GenesisProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for GenesisHandle {
    type Target = dyn GenesisProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for GenesisHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GenesisHandle").finish()
    }
}

pub struct SyncHandle {
    pub(crate) inner: Box<dyn SyncProvider>,
}
impl SyncHandle {
    pub fn new(inner: Box<dyn SyncProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for SyncHandle {
    type Target = dyn SyncProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for SyncHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyncHandle").finish()
    }
}

pub struct HealthHandle {
    pub(crate) inner: Box<dyn HealthProvider>,
}
impl HealthHandle {
    pub fn new(inner: Box<dyn HealthProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for HealthHandle {
    type Target = dyn HealthProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for HealthHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HealthHandle").finish()
    }
}

pub struct ConsensusHandle {
    pub(crate) inner: Box<dyn ConsensusProvider>,
}
impl ConsensusHandle {
    pub fn new(inner: Box<dyn ConsensusProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for ConsensusHandle {
    type Target = dyn ConsensusProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for ConsensusHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConsensusHandle").finish()
    }
}

pub struct RecoveryHandle {
    pub(crate) inner: Box<dyn RecoveryProvider>,
}
impl RecoveryHandle {
    pub fn new(inner: Box<dyn RecoveryProvider>) -> Self {
        Self { inner }
    }
}
impl std::ops::Deref for RecoveryHandle {
    type Target = dyn RecoveryProvider;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl fmt::Debug for RecoveryHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecoveryHandle").finish()
    }
}

pub struct RegistryHandle {
    pub(crate) read: Box<dyn ValidatorRead>,
    pub(crate) admin: Box<dyn ValidatorAdmin>,
}
impl RegistryHandle {
    pub fn new(read: Box<dyn ValidatorRead>, admin: Box<dyn ValidatorAdmin>) -> Self {
        Self { read, admin }
    }
    pub fn read(&self) -> &dyn ValidatorRead {
        &*self.read
    }
    pub fn admin(&self) -> &dyn ValidatorAdmin {
        &*self.admin
    }
}
impl fmt::Debug for RegistryHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistryHandle").finish()
    }
}
