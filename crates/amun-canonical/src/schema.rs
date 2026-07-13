#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaVersion {
    V4,
}

impl SchemaVersion {
    pub fn as_tag(&self) -> &[u8] {
        match self {
            Self::V4 => b"AMUN_CANONICAL_V4",
        }
    }
}
