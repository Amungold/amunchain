use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::{PublicKey, ValidatorId};

/// Single source of truth for certificate data.
/// IdentityService delegates to this — it does NOT copy the data.
#[derive(Debug, Clone)]
pub struct CertificateData {
    pub validator_id: ValidatorId,
    pub public_key: PublicKey,
    pub certificate_hash: [u8; 32],
    pub authority_id: [u8; 32],
    pub authority_version: u64,
    pub valid_from: u64,
    pub valid_until: u64,
    pub signature: Vec<u8>,
}

pub struct CertificateStore {
    data: CertificateData,
}

impl CertificateStore {
    pub fn new(data: CertificateData) -> Self {
        CertificateStore { data }
    }

    /// TODO: Read actual file when certificate format is finalized.
    pub fn load_from_file(_path: &str) -> PlatformResult<CertificateData> {
        Ok(CertificateData {
            validator_id: ValidatorId([1u8; 32]),
            public_key: PublicKey([1u8; 32]),
            certificate_hash: [1u8; 32],
            authority_id: [0u8; 32],
            authority_version: 1,
            valid_from: 0,
            valid_until: 0,
            signature: vec![1u8; 64],
        })
    }

    pub fn validator_id(&self) -> &ValidatorId {
        &self.data.validator_id
    }
    pub fn public_key(&self) -> &PublicKey {
        &self.data.public_key
    }
    pub fn certificate_hash(&self) -> &[u8; 32] {
        &self.data.certificate_hash
    }
    pub fn data(&self) -> &CertificateData {
        &self.data
    }
}
