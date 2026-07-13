use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub const MAGIC_BYTES: [u8; 4] = [0x41, 0x4D, 0x55, 0x4E];
pub const PROTOCOL_VERSION: u32 = 1;
pub const MIN_COMPATIBLE_VERSION: u32 = 1;
pub const HANDSHAKE_TIMEOUT_SECS: u64 = 10;
pub const NONCE_TIMEOUT_SECS: u64 = 30;
pub const MAX_TIMESTAMP_DRIFT_SECS: i64 = 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionInfo {
    pub version: u64,
    pub hash: [u8; 32],
    pub proof_system_version: u32,
    pub state_commitment_algorithm: String,
    pub accepted_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloRequest {
    pub magic: [u8; 4],
    pub protocol_version: u32,
    pub network_id: [u8; 32],
    pub genesis_hash: [u8; 32],
    pub node_id: [u8; 32],
    pub verifying_key: [u8; 32],
    pub constitution: ConstitutionInfo,
    pub capabilities: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    pub accepted: bool,
    pub reason: Option<String>,
    pub node_id: [u8; 32],
    pub verifying_key: [u8; 32],
    pub nonce: [u8; 32],
    pub constitution: ConstitutionInfo,
    pub capabilities: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProof {
    pub node_id: [u8; 32],
    // Use Vec<u8> instead of [u8; 64] for serde compatibility
    pub nonce_signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfirmation {
    pub accepted: bool,
    pub session_id: [u8; 32],
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeError {
    InvalidMagic,
    IncompatibleVersion(u32),
    WrongNetwork {
        expected: [u8; 32],
        received: [u8; 32],
    },
    WrongGenesis {
        expected: [u8; 32],
        received: [u8; 32],
    },
    IncompatibleConstitution {
        reason: String,
    },
    CapabilityMismatch {
        required: Vec<String>,
        provided: Vec<String>,
    },
    AuthenticationFailed,
    ReplayAttack,
    ExpiredNonce,
    TimestampDrift(u64),
    Blacklisted,
    TooManyConnections,
    Timeout,
    IoError(String),
}

impl HelloRequest {
    pub fn new(
        network_id: [u8; 32],
        genesis_hash: [u8; 32],
        node_id: [u8; 32],
        signing_key: &SigningKey,
        constitution: ConstitutionInfo,
        capabilities: Vec<String>,
    ) -> Self {
        Self {
            magic: MAGIC_BYTES,
            protocol_version: PROTOCOL_VERSION,
            network_id,
            genesis_hash,
            node_id,
            verifying_key: signing_key.verifying_key().to_bytes(),
            constitution,
            capabilities,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn validate_basic(
        &self,
        expected_network_id: [u8; 32],
        expected_genesis: [u8; 32],
        expected_constitution: &ConstitutionInfo,
    ) -> Result<(), HandshakeError> {
        if self.magic != MAGIC_BYTES {
            return Err(HandshakeError::InvalidMagic);
        }
        if self.protocol_version < MIN_COMPATIBLE_VERSION {
            return Err(HandshakeError::IncompatibleVersion(self.protocol_version));
        }
        if self.network_id != expected_network_id {
            return Err(HandshakeError::WrongNetwork {
                expected: expected_network_id,
                received: self.network_id,
            });
        }
        if self.genesis_hash != expected_genesis {
            return Err(HandshakeError::WrongGenesis {
                expected: expected_genesis,
                received: self.genesis_hash,
            });
        }
        if self.constitution.hash != expected_constitution.hash {
            return Err(HandshakeError::IncompatibleConstitution {
                reason: format!(
                    "Constitution hash mismatch: expected {:?}, got {:?}",
                    expected_constitution.hash, self.constitution.hash
                ),
            });
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let drift = (self.timestamp as i64 - now).abs();
        if drift > MAX_TIMESTAMP_DRIFT_SECS {
            return Err(HandshakeError::TimestampDrift(self.timestamp));
        }
        Ok(())
    }

    pub fn validate_capabilities(
        &self,
        required_capabilities: &[String],
    ) -> Result<(), HandshakeError> {
        let provided: std::collections::HashSet<_> = self.capabilities.iter().collect();
        let missing: Vec<_> = required_capabilities
            .iter()
            .filter(|c| !provided.contains(*c))
            .cloned()
            .collect();
        if !missing.is_empty() {
            return Err(HandshakeError::CapabilityMismatch {
                required: required_capabilities.to_vec(),
                provided: self.capabilities.clone(),
            });
        }
        Ok(())
    }
}

impl ChallengeResponse {
    pub fn create_challenge(
        node_id: [u8; 32],
        signing_key: &SigningKey,
        constitution: ConstitutionInfo,
        capabilities: Vec<String>,
    ) -> Self {
        let mut nonce = [0u8; 32];
        rand::thread_rng().fill(&mut nonce);
        Self {
            accepted: true,
            reason: None,
            node_id,
            verifying_key: signing_key.verifying_key().to_bytes(),
            nonce,
            constitution,
            capabilities,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn reject(reason: String) -> Self {
        Self {
            accepted: false,
            reason: Some(reason),
            node_id: [0u8; 32],
            verifying_key: [0u8; 32],
            nonce: [0u8; 32],
            constitution: ConstitutionInfo {
                version: 0,
                hash: [0u8; 32],
                proof_system_version: 0,
                state_commitment_algorithm: String::new(),
                accepted_features: Vec::new(),
            },
            capabilities: Vec::new(),
            timestamp: 0,
        }
    }
}

impl AuthProof {
    pub fn create(node_id: [u8; 32], nonce: [u8; 32], signing_key: &SigningKey) -> Self {
        let mut message = Vec::new();
        message.extend_from_slice(b"AMUN_HANDSHAKE_AUTH_V1");
        message.extend_from_slice(&node_id);
        message.extend_from_slice(&nonce);
        let signature = signing_key.sign(&message);
        Self {
            node_id,
            nonce_signature: signature.to_bytes().to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn verify(
        &self,
        nonce: [u8; 32],
        verifying_key_bytes: [u8; 32],
    ) -> Result<(), HandshakeError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let drift = (self.timestamp as i64 - now).abs();
        if drift > NONCE_TIMEOUT_SECS as i64 {
            return Err(HandshakeError::ExpiredNonce);
        }
        let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes)
            .map_err(|_| HandshakeError::AuthenticationFailed)?;
        let signature = Signature::from_slice(&self.nonce_signature)
            .map_err(|_| HandshakeError::AuthenticationFailed)?;
        let mut message = Vec::new();
        message.extend_from_slice(b"AMUN_HANDSHAKE_AUTH_V1");
        message.extend_from_slice(&self.node_id);
        message.extend_from_slice(&nonce);
        verifying_key
            .verify_strict(&message, &signature)
            .map_err(|_| HandshakeError::AuthenticationFailed)
    }
}

impl AuthConfirmation {
    pub fn accept() -> Self {
        let mut session_id = [0u8; 32];
        rand::thread_rng().fill(&mut session_id);
        Self {
            accepted: true,
            session_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn reject() -> Self {
        Self {
            accepted: false,
            session_id: [0u8; 32],
            timestamp: 0,
        }
    }
}
