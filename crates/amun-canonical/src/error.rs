#[derive(Debug, PartialEq, Eq)]
pub enum CanonicalError {
    BufferOverflow,
    InvalidSchema,
    TruncatedData,
    InvalidLength,
    EnumOutOfRange,
    FloatDetected,
}
