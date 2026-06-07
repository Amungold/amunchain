use blake3::Hasher;

/// A globally unique cryptographic identifier for a constitutional resource.
///
/// Computed as: Blake3(RESOURCE_ID_V1 || tx_hash || contract_id ||
///   resource_type || version || derivation_index)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(pub [u8; 32]);

impl ResourceId {
    pub const VERSION: u8 = 1;

    pub fn compute(
        tx_hash: &[u8; 32],
        contract_id: &[u8; 32],
        resource_type: &str,
        version: u64,
        derivation_index: u32,
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&[Self::VERSION]);
        hasher.update(tx_hash);
        hasher.update(contract_id);
        hasher.update(resource_type.as_bytes());
        hasher.update(&version.to_le_bytes());
        hasher.update(&derivation_index.to_le_bytes());
        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] { &self.0 }

    pub fn to_hex(&self) -> String { hex::encode(self.0) }
}

impl serde::Serialize for ResourceId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_hex().serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for ResourceId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hex_str = String::deserialize(d)?;
        let bytes = hex::decode(&hex_str).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("ResourceId must be 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
