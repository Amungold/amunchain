use std::collections::BTreeSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion(u32);
impl SchemaVersion { pub const V1: Self = Self(1); pub fn new(v: u32) -> Self { Self(v) } pub fn as_u32(&self) -> u32 { self.0 } }
pub struct SchemaValidator { allowed: BTreeSet<u8>, required: BTreeSet<u8> }
impl SchemaValidator { pub fn for_vote(_version: SchemaVersion) -> Self { Self { allowed: BTreeSet::new(), required: BTreeSet::new() } } pub fn validate_tags(&self, _tags: &[u8]) -> Result<(), String> { Ok(()) } }
